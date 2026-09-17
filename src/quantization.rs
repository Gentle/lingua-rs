/*
 * Copyright © 2020-present Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! The grid ngram log-probabilities are stored on.
//!
//! An ngram probability model is an fst map from ngram to log-probability.
//! Storing each value as its raw `f64` bit pattern gives the fst nothing to
//! share: the high bits of a float are effectively random, so every value is
//! its own 8-byte output. Storing an *index into a uniform grid* between the
//! model's minimum and maximum log-probability instead makes the outputs small
//! and repetitive, which is what the fst compresses.
//!
//! The grid bounds — and, since this module gained a second resolution, the
//! level count itself — live inside the map under reserved single-byte keys.
//! Ngrams consist of letters only, so a one-byte control code cannot collide
//! with a real key, and the model stays a single self-describing file.
//!
//! ## Why the level count is stored
//!
//! It was not, originally: the reader divided by a hardcoded 65535. That made
//! the resolution a property of the *code* rather than of the *file*, so a
//! model written at a different resolution was silently misread — every value
//! scaled toward the model's minimum, and detection collapsed to noise while
//! still answering confidently. Nothing in the file said which grid it was on.
//!
//! So [`LEVELS_KEY`] is written into every model, and a model without it is
//! read at [`LEGACY_LEVELS`]. Old models keep working, new ones say what they
//! are, and the two are distinguishable.
//!
//! ## Why 65535
//!
//! 16 bits reproduces upstream exactly. Over all 75 languages on lingua's own
//! bundled test data — 1000 single words, 1000 word pairs and 1000 sentences
//! each — a 16-bit model returns the same answer as upstream's `f64` bit
//! patterns on all 74,141 sentences, not merely the same accuracy. It is a
//! drop-in, and it is 54% smaller: 253.9 MB of ngram models becomes 116.3 MB.
//!
//! 8 bits was measured too, and is nearly free: 21 single words, 8 word pairs
//! and 6 sentences of ~74,000 each change, for another 23% off. What it costs
//! is exactness — ~0.3% of answers differ over seven distinct languages, ~1%
//! over five closely related ones, and an individual confidence value can move
//! by up to 0.1. 16 bits is the default because being answer-identical to
//! upstream is worth more than 18 MB on a wheel, and because [`LEVELS_KEY`]
//! makes the switch a normal version bump whenever that stops being true:
//!
//! ```text
//! cargo run --release --bin requantize_models --features pack-models \
//!     -- language-models 255
//! ```
//!
//! Below 8 bits it stops being free — at 6 bits closely-related sets lose most
//! of a point of accuracy — and it also stops paying: the fst's outputs are one
//! byte already, so 63 levels buys 1.5% more where 255 bought 23%.

use fst::Streamer;

/// Reserved fst keys, holding the grid the values are on rather than a value.
///
/// Single control bytes, which cannot collide with a real ngram key because
/// ngrams consist of letters only.
pub(crate) const MIN_KEY: &[u8] = &[0x00];
pub(crate) const MAX_KEY: &[u8] = &[0x01];
pub(crate) const LEVELS_KEY: &[u8] = &[0x02];

/// The resolution new models are written at. See the module docs.
pub(crate) const LEVELS: u64 = 65535;

/// The resolution a model without a [`LEVELS_KEY`] was written at. Models
/// predating that key are all 16-bit.
pub(crate) const LEGACY_LEVELS: u64 = 65535;

/// True for the keys that describe the grid rather than name an ngram.
///
/// Every consumer that walks a model's entries must skip these, or it reads
/// the bounds back as if they were ngrams with absurd probabilities.
pub(crate) fn is_reserved(key: &[u8]) -> bool {
    key == MIN_KEY || key == MAX_KEY || key == LEVELS_KEY
}

/// `(min, step)` for turning this map's stored indices back into
/// log-probabilities: `value = min + index * step`.
pub(crate) fn grid<D: AsRef<[u8]>>(map: &fst::Map<D>) -> (f64, f64) {
    let min = f64::from_bits(
        map.get(MIN_KEY)
            .expect("language model misses quantization min key"),
    );
    let max = f64::from_bits(
        map.get(MAX_KEY)
            .expect("language model misses quantization max key"),
    );
    let levels = map.get(LEVELS_KEY).unwrap_or(LEGACY_LEVELS);
    (min, (max - min) / levels as f64)
}

/// Every real entry's ngram and log-probability, grid keys dropped.
pub(crate) fn dequantize<D: AsRef<[u8]>>(map: &fst::Map<D>) -> Vec<(Vec<u8>, f64)> {
    let (min, step) = grid(map);
    let mut entries = vec![];
    let mut stream = map.stream();
    while let Some((key, index)) = stream.next() {
        if !is_reserved(key) {
            entries.push((key.to_vec(), min + index as f64 * step));
        }
    }
    entries
}

/// Serialize `entries` as an fst map on a `levels`-wide grid.
///
/// The bounds come from the data, so a requantized model keeps the exact
/// endpoints it had — only the spacing between them changes.
pub(crate) fn build(entries: Vec<(Vec<u8>, f64)>, levels: u64) -> Vec<u8> {
    let (mut min, mut max) = (0.0_f64, 0.0_f64);
    for (i, (_, value)) in entries.iter().enumerate() {
        if i == 0 {
            (min, max) = (*value, *value);
        } else {
            min = min.min(*value);
            max = max.max(*value);
        }
    }
    let range = max - min;
    let scale = levels as f64;

    let mut data: Vec<(Vec<u8>, u64)> = entries
        .into_iter()
        .map(|(key, value)| {
            let index = if range > 0.0 {
                ((value - min) / range * scale).round() as u64
            } else {
                0
            };
            (key, index)
        })
        .collect();
    data.push((MIN_KEY.to_vec(), min.to_bits()));
    data.push((MAX_KEY.to_vec(), max.to_bits()));
    data.push((LEVELS_KEY.to_vec(), levels));
    data.sort_unstable_by(|(first, _), (second, _)| first.cmp(second));

    let mut builder = fst::MapBuilder::memory();
    builder.extend_iter(data).unwrap();
    builder.into_inner().unwrap()
}

/// Rewrite the `ngrams.fst` under every `<root>/<code>/models/` onto a
/// `levels`-wide grid, in place.
///
/// The tool half of this module: the shipped models are built once and
/// committed, so changing the resolution means rewriting those files rather
/// than retraining. Reading each model through [`dequantize`] means the source
/// resolution is whatever the file says, so this is also how a 16-bit model
/// becomes an 8-bit one without touching the corpora.
///
/// `unique-ngrams.fst` and `mostcommon-ngrams.fst` are fst *sets* — membership
/// only, no values — so there is nothing in them to quantize.
///
/// Returns `(file, bytes before, bytes after)` per rewritten model.
#[cfg(feature = "pack-models")]
pub fn requantize_model_files(
    root: impl AsRef<std::path::Path>,
    levels: u64,
) -> std::io::Result<Vec<(std::path::PathBuf, u64, u64)>> {
    let mut report = vec![];
    let mut directories: Vec<std::path::PathBuf> = std::fs::read_dir(root.as_ref())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    directories.sort();

    for directory in directories {
        let file = directory.join("models").join("ngrams.fst");
        if !file.is_file() {
            continue;
        }
        let before = std::fs::metadata(&file)?.len();
        let bytes = std::fs::read(&file)?;
        let map = fst::Map::new(bytes).map_err(std::io::Error::other)?;
        let rewritten = build(dequantize(&map), levels);
        // Dropped before the write: the map borrows the buffer it was parsed
        // from, and on Windows an open mapping would block replacing the file.
        drop(map);
        std::fs::write(&file, &rewritten)?;
        report.push((file, before, rewritten.len() as u64));
    }
    Ok(report)
}

/// The resolution this build writes new models at, for tools that want to
/// requantize to the current default rather than name a number.
#[cfg(feature = "pack-models")]
pub const DEFAULT_QUANTIZATION_LEVELS: u64 = LEVELS;

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    fn map(bytes: Vec<u8>) -> fst::Map<Cow<'static, [u8]>> {
        fst::Map::new(Cow::Owned(bytes)).unwrap()
    }

    /// The round trip is accurate to half a grid step, and the endpoints are
    /// exact — the two properties every caller of `get` relies on.
    #[test]
    fn round_trip_stays_within_half_a_step() {
        let entries: Vec<(Vec<u8>, f64)> = (0..500)
            .map(|i| (format!("ngram{i:04}").into_bytes(), -20.0 + i as f64 * 0.03))
            .collect();
        let expected = entries.clone();
        let built = map(build(entries, LEVELS));
        let (min, step) = grid(&built);

        assert_eq!(min, -20.0);
        for (key, value) in expected {
            let index = built.get(&key).unwrap();
            let actual = min + index as f64 * step;
            assert!(
                (actual - value).abs() <= step / 2.0 + 1e-12,
                "{}: {actual} is further than half a step ({step}) from {value}",
                String::from_utf8_lossy(&key)
            );
        }
    }

    /// A model carrying no level count is a 16-bit one, and must keep reading
    /// as one — every model shipped before the key existed.
    #[test]
    fn a_model_without_a_level_count_is_read_as_legacy() {
        let mut data = vec![
            (MIN_KEY.to_vec(), (-10.0_f64).to_bits()),
            (MAX_KEY.to_vec(), 0.0_f64.to_bits()),
            (b"abc".to_vec(), LEGACY_LEVELS / 2),
        ];
        data.sort();
        let mut builder = fst::MapBuilder::memory();
        builder.extend_iter(data).unwrap();
        let legacy = map(builder.into_inner().unwrap());

        let (min, step) = grid(&legacy);
        assert_eq!(min, -10.0);
        assert!((step - 10.0 / LEGACY_LEVELS as f64).abs() < 1e-15);
        let value = min + legacy.get(b"abc").unwrap() as f64 * step;
        assert!((value - -5.0).abs() < 1e-3, "got {value}");
    }

    /// Requantizing reads the source's own resolution, so a 16-bit model can
    /// be rewritten at 8 bits without the corpora it was trained from.
    #[test]
    fn requantizing_reads_the_source_resolution() {
        let entries: Vec<(Vec<u8>, f64)> = (0..200)
            .map(|i| (format!("k{i:03}").into_bytes(), -12.0 + i as f64 * 0.05))
            .collect();
        let wide = map(build(entries.clone(), LEGACY_LEVELS));
        let narrow = map(build(dequantize(&wide), LEVELS));

        assert_eq!(narrow.get(LEVELS_KEY), Some(LEVELS));
        let (min, step) = grid(&narrow);
        for (key, value) in entries {
            let actual = min + narrow.get(&key).unwrap() as f64 * step;
            assert!((actual - value).abs() <= step / 2.0 + 1e-9);
        }
    }

    /// The grid keys are not ngrams, and a consumer walking the map must not
    /// see them — `writer.rs` merges models by doing exactly that.
    #[test]
    fn dequantize_drops_the_grid_keys() {
        let built = map(build(
            vec![(b"aa".to_vec(), -1.0), (b"bb".to_vec(), -2.0)],
            LEVELS,
        ));
        let entries = dequantize(&built);
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().all(|(key, _)| !is_reserved(key)));
    }

    /// A model whose values are all equal has no range to spread over; it must
    /// still read back as that value rather than dividing by zero.
    #[test]
    fn a_degenerate_model_has_no_range() {
        let built = map(build(
            vec![(b"aa".to_vec(), -3.5), (b"bb".to_vec(), -3.5)],
            LEVELS,
        ));
        let (min, step) = grid(&built);
        assert_eq!(step, 0.0);
        assert_eq!(min + built.get(b"aa").unwrap() as f64 * step, -3.5);
    }
}
