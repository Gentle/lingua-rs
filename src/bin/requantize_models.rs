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

//! Rewrites every shipped `ngrams.fst` onto a different quantization grid:
//!
//!     cargo run --release --bin requantize_models --features pack-models \
//!         -- language-models [levels]
//!
//! The models are committed artifacts, so changing the resolution is a rewrite
//! rather than a retrain. Each file is read at whatever resolution it records,
//! which is what lets a 16-bit model become an 8-bit one with no corpus.

fn main() {
    let mut args = std::env::args().skip(1);
    let root = args.next().unwrap_or_else(|| "language-models".to_string());
    let levels = match args.next() {
        Some(value) => value.parse().expect("levels must be a positive integer"),
        None => lingua::DEFAULT_QUANTIZATION_LEVELS,
    };
    assert!(levels > 0, "levels must be a positive integer");

    let report = lingua::requantize_model_files(&root, levels).expect("failed to requantize");
    if report.is_empty() {
        eprintln!("no <language>/models/ngrams.fst found under {root}");
        std::process::exit(1);
    }

    let (mut before, mut after) = (0u64, 0u64);
    for (file, was, now) in &report {
        before += was;
        after += now;
        println!("{} {was} -> {now}", file.display());
    }
    println!(
        "{} models at {levels} levels: {before} -> {after} bytes ({:.1}% smaller)",
        report.len(),
        100.0 * (1.0 - after as f64 / before as f64)
    );
}
