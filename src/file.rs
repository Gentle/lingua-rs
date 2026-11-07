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

use crate::Language;
use crate::detector::{CountModelFst, LanguageModelFst};
#[cfg(not(feature = "external-models"))]
use include_dir::Dir;
#[cfg(all(feature = "afrikaans", not(feature = "external-models")))]
use lingua_afrikaans_language_model::{AFRIKAANS_MODELS_DIRECTORY, AFRIKAANS_TESTDATA_DIRECTORY};
#[cfg(all(feature = "albanian", not(feature = "external-models")))]
use lingua_albanian_language_model::{ALBANIAN_MODELS_DIRECTORY, ALBANIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "arabic", not(feature = "external-models")))]
use lingua_arabic_language_model::{ARABIC_MODELS_DIRECTORY, ARABIC_TESTDATA_DIRECTORY};
#[cfg(all(feature = "armenian", not(feature = "external-models")))]
use lingua_armenian_language_model::{ARMENIAN_MODELS_DIRECTORY, ARMENIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "azerbaijani", not(feature = "external-models")))]
use lingua_azerbaijani_language_model::{
    AZERBAIJANI_MODELS_DIRECTORY, AZERBAIJANI_TESTDATA_DIRECTORY,
};
#[cfg(all(feature = "basque", not(feature = "external-models")))]
use lingua_basque_language_model::{BASQUE_MODELS_DIRECTORY, BASQUE_TESTDATA_DIRECTORY};
#[cfg(all(feature = "belarusian", not(feature = "external-models")))]
use lingua_belarusian_language_model::{
    BELARUSIAN_MODELS_DIRECTORY, BELARUSIAN_TESTDATA_DIRECTORY,
};
#[cfg(all(feature = "bengali", not(feature = "external-models")))]
use lingua_bengali_language_model::{BENGALI_MODELS_DIRECTORY, BENGALI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "bokmal", not(feature = "external-models")))]
use lingua_bokmal_language_model::{BOKMAL_MODELS_DIRECTORY, BOKMAL_TESTDATA_DIRECTORY};
#[cfg(all(feature = "bosnian", not(feature = "external-models")))]
use lingua_bosnian_language_model::{BOSNIAN_MODELS_DIRECTORY, BOSNIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "bulgarian", not(feature = "external-models")))]
use lingua_bulgarian_language_model::{BULGARIAN_MODELS_DIRECTORY, BULGARIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "catalan", not(feature = "external-models")))]
use lingua_catalan_language_model::{CATALAN_MODELS_DIRECTORY, CATALAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "chinese", not(feature = "external-models")))]
use lingua_chinese_language_model::{CHINESE_MODELS_DIRECTORY, CHINESE_TESTDATA_DIRECTORY};
#[cfg(all(feature = "croatian", not(feature = "external-models")))]
use lingua_croatian_language_model::{CROATIAN_MODELS_DIRECTORY, CROATIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "czech", not(feature = "external-models")))]
use lingua_czech_language_model::{CZECH_MODELS_DIRECTORY, CZECH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "danish", not(feature = "external-models")))]
use lingua_danish_language_model::{DANISH_MODELS_DIRECTORY, DANISH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "dutch", not(feature = "external-models")))]
use lingua_dutch_language_model::{DUTCH_MODELS_DIRECTORY, DUTCH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "english", not(feature = "external-models")))]
use lingua_english_language_model::{ENGLISH_MODELS_DIRECTORY, ENGLISH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "esperanto", not(feature = "external-models")))]
use lingua_esperanto_language_model::{ESPERANTO_MODELS_DIRECTORY, ESPERANTO_TESTDATA_DIRECTORY};
#[cfg(all(feature = "estonian", not(feature = "external-models")))]
use lingua_estonian_language_model::{ESTONIAN_MODELS_DIRECTORY, ESTONIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "finnish", not(feature = "external-models")))]
use lingua_finnish_language_model::{FINNISH_MODELS_DIRECTORY, FINNISH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "french", not(feature = "external-models")))]
use lingua_french_language_model::{FRENCH_MODELS_DIRECTORY, FRENCH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "ganda", not(feature = "external-models")))]
use lingua_ganda_language_model::{GANDA_MODELS_DIRECTORY, GANDA_TESTDATA_DIRECTORY};
#[cfg(all(feature = "georgian", not(feature = "external-models")))]
use lingua_georgian_language_model::{GEORGIAN_MODELS_DIRECTORY, GEORGIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "german", not(feature = "external-models")))]
use lingua_german_language_model::{GERMAN_MODELS_DIRECTORY, GERMAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "greek", not(feature = "external-models")))]
use lingua_greek_language_model::{GREEK_MODELS_DIRECTORY, GREEK_TESTDATA_DIRECTORY};
#[cfg(all(feature = "gujarati", not(feature = "external-models")))]
use lingua_gujarati_language_model::{GUJARATI_MODELS_DIRECTORY, GUJARATI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "hebrew", not(feature = "external-models")))]
use lingua_hebrew_language_model::{HEBREW_MODELS_DIRECTORY, HEBREW_TESTDATA_DIRECTORY};
#[cfg(all(feature = "hindi", not(feature = "external-models")))]
use lingua_hindi_language_model::{HINDI_MODELS_DIRECTORY, HINDI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "hungarian", not(feature = "external-models")))]
use lingua_hungarian_language_model::{HUNGARIAN_MODELS_DIRECTORY, HUNGARIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "icelandic", not(feature = "external-models")))]
use lingua_icelandic_language_model::{ICELANDIC_MODELS_DIRECTORY, ICELANDIC_TESTDATA_DIRECTORY};
#[cfg(all(feature = "indonesian", not(feature = "external-models")))]
use lingua_indonesian_language_model::{
    INDONESIAN_MODELS_DIRECTORY, INDONESIAN_TESTDATA_DIRECTORY,
};
#[cfg(all(feature = "irish", not(feature = "external-models")))]
use lingua_irish_language_model::{IRISH_MODELS_DIRECTORY, IRISH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "italian", not(feature = "external-models")))]
use lingua_italian_language_model::{ITALIAN_MODELS_DIRECTORY, ITALIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "japanese", not(feature = "external-models")))]
use lingua_japanese_language_model::{JAPANESE_MODELS_DIRECTORY, JAPANESE_TESTDATA_DIRECTORY};
#[cfg(all(feature = "kazakh", not(feature = "external-models")))]
use lingua_kazakh_language_model::{KAZAKH_MODELS_DIRECTORY, KAZAKH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "korean", not(feature = "external-models")))]
use lingua_korean_language_model::{KOREAN_MODELS_DIRECTORY, KOREAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "latin", not(feature = "external-models")))]
use lingua_latin_language_model::{LATIN_MODELS_DIRECTORY, LATIN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "latvian", not(feature = "external-models")))]
use lingua_latvian_language_model::{LATVIAN_MODELS_DIRECTORY, LATVIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "lithuanian", not(feature = "external-models")))]
use lingua_lithuanian_language_model::{
    LITHUANIAN_MODELS_DIRECTORY, LITHUANIAN_TESTDATA_DIRECTORY,
};
#[cfg(all(feature = "luxembourgish", not(feature = "external-models")))]
use lingua_luxembourgish_language_model::{
    LUXEMBOURGISH_MODELS_DIRECTORY, LUXEMBOURGISH_TESTDATA_DIRECTORY,
};
#[cfg(all(feature = "macedonian", not(feature = "external-models")))]
use lingua_macedonian_language_model::{
    MACEDONIAN_MODELS_DIRECTORY, MACEDONIAN_TESTDATA_DIRECTORY,
};
#[cfg(all(feature = "malay", not(feature = "external-models")))]
use lingua_malay_language_model::{MALAY_MODELS_DIRECTORY, MALAY_TESTDATA_DIRECTORY};
#[cfg(all(feature = "maori", not(feature = "external-models")))]
use lingua_maori_language_model::{MAORI_MODELS_DIRECTORY, MAORI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "marathi", not(feature = "external-models")))]
use lingua_marathi_language_model::{MARATHI_MODELS_DIRECTORY, MARATHI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "mongolian", not(feature = "external-models")))]
use lingua_mongolian_language_model::{MONGOLIAN_MODELS_DIRECTORY, MONGOLIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "nynorsk", not(feature = "external-models")))]
use lingua_nynorsk_language_model::{NYNORSK_MODELS_DIRECTORY, NYNORSK_TESTDATA_DIRECTORY};
#[cfg(all(feature = "persian", not(feature = "external-models")))]
use lingua_persian_language_model::{PERSIAN_MODELS_DIRECTORY, PERSIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "polish", not(feature = "external-models")))]
use lingua_polish_language_model::{POLISH_MODELS_DIRECTORY, POLISH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "portuguese", not(feature = "external-models")))]
use lingua_portuguese_language_model::{
    PORTUGUESE_MODELS_DIRECTORY, PORTUGUESE_TESTDATA_DIRECTORY,
};
#[cfg(all(feature = "punjabi", not(feature = "external-models")))]
use lingua_punjabi_language_model::{PUNJABI_MODELS_DIRECTORY, PUNJABI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "romanian", not(feature = "external-models")))]
use lingua_romanian_language_model::{ROMANIAN_MODELS_DIRECTORY, ROMANIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "russian", not(feature = "external-models")))]
use lingua_russian_language_model::{RUSSIAN_MODELS_DIRECTORY, RUSSIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "serbian", not(feature = "external-models")))]
use lingua_serbian_language_model::{SERBIAN_MODELS_DIRECTORY, SERBIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "shona", not(feature = "external-models")))]
use lingua_shona_language_model::{SHONA_MODELS_DIRECTORY, SHONA_TESTDATA_DIRECTORY};
#[cfg(all(feature = "slovak", not(feature = "external-models")))]
use lingua_slovak_language_model::{SLOVAK_MODELS_DIRECTORY, SLOVAK_TESTDATA_DIRECTORY};
#[cfg(all(feature = "slovene", not(feature = "external-models")))]
use lingua_slovene_language_model::{SLOVENE_MODELS_DIRECTORY, SLOVENE_TESTDATA_DIRECTORY};
#[cfg(all(feature = "somali", not(feature = "external-models")))]
use lingua_somali_language_model::{SOMALI_MODELS_DIRECTORY, SOMALI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "sotho", not(feature = "external-models")))]
use lingua_sotho_language_model::{SOTHO_MODELS_DIRECTORY, SOTHO_TESTDATA_DIRECTORY};
#[cfg(all(feature = "spanish", not(feature = "external-models")))]
use lingua_spanish_language_model::{SPANISH_MODELS_DIRECTORY, SPANISH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "swahili", not(feature = "external-models")))]
use lingua_swahili_language_model::{SWAHILI_MODELS_DIRECTORY, SWAHILI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "swedish", not(feature = "external-models")))]
use lingua_swedish_language_model::{SWEDISH_MODELS_DIRECTORY, SWEDISH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "tagalog", not(feature = "external-models")))]
use lingua_tagalog_language_model::{TAGALOG_MODELS_DIRECTORY, TAGALOG_TESTDATA_DIRECTORY};
#[cfg(all(feature = "tamil", not(feature = "external-models")))]
use lingua_tamil_language_model::{TAMIL_MODELS_DIRECTORY, TAMIL_TESTDATA_DIRECTORY};
#[cfg(all(feature = "telugu", not(feature = "external-models")))]
use lingua_telugu_language_model::{TELUGU_MODELS_DIRECTORY, TELUGU_TESTDATA_DIRECTORY};
#[cfg(all(feature = "thai", not(feature = "external-models")))]
use lingua_thai_language_model::{THAI_MODELS_DIRECTORY, THAI_TESTDATA_DIRECTORY};
#[cfg(all(feature = "tsonga", not(feature = "external-models")))]
use lingua_tsonga_language_model::{TSONGA_MODELS_DIRECTORY, TSONGA_TESTDATA_DIRECTORY};
#[cfg(all(feature = "tswana", not(feature = "external-models")))]
use lingua_tswana_language_model::{TSWANA_MODELS_DIRECTORY, TSWANA_TESTDATA_DIRECTORY};
#[cfg(all(feature = "turkish", not(feature = "external-models")))]
use lingua_turkish_language_model::{TURKISH_MODELS_DIRECTORY, TURKISH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "ukrainian", not(feature = "external-models")))]
use lingua_ukrainian_language_model::{UKRAINIAN_MODELS_DIRECTORY, UKRAINIAN_TESTDATA_DIRECTORY};
#[cfg(all(feature = "urdu", not(feature = "external-models")))]
use lingua_urdu_language_model::{URDU_MODELS_DIRECTORY, URDU_TESTDATA_DIRECTORY};
#[cfg(all(feature = "vietnamese", not(feature = "external-models")))]
use lingua_vietnamese_language_model::{
    VIETNAMESE_MODELS_DIRECTORY, VIETNAMESE_TESTDATA_DIRECTORY,
};
#[cfg(all(feature = "welsh", not(feature = "external-models")))]
use lingua_welsh_language_model::{WELSH_MODELS_DIRECTORY, WELSH_TESTDATA_DIRECTORY};
#[cfg(all(feature = "xhosa", not(feature = "external-models")))]
use lingua_xhosa_language_model::{XHOSA_MODELS_DIRECTORY, XHOSA_TESTDATA_DIRECTORY};
#[cfg(all(feature = "yoruba", not(feature = "external-models")))]
use lingua_yoruba_language_model::{YORUBA_MODELS_DIRECTORY, YORUBA_TESTDATA_DIRECTORY};
#[cfg(all(feature = "zulu", not(feature = "external-models")))]
use lingua_zulu_language_model::{ZULU_MODELS_DIRECTORY, ZULU_TESTDATA_DIRECTORY};
use std::borrow::Cow;
use std::io::ErrorKind;

#[cfg(not(feature = "external-models"))]
pub(crate) fn read_probability_model_data_file(
    language: Language,
    file_name: &str,
) -> std::io::Result<LanguageModelFst> {
    let directory = get_language_models_directory(language);
    let fst_file = directory.get_file(file_name).ok_or(ErrorKind::NotFound)?;
    let fst_map = fst::Map::new(Cow::Borrowed(fst_file.contents())).unwrap();
    Ok(LanguageModelFst::from_fst_map(fst_map))
}

#[cfg(not(feature = "external-models"))]
pub(crate) fn read_count_model_data_file(
    language: Language,
    file_name: &str,
) -> std::io::Result<CountModelFst> {
    let directory = get_language_models_directory(language);
    let fst_file = directory.get_file(file_name).ok_or(ErrorKind::NotFound)?;
    let fst_set = fst::Set::new(Cow::Borrowed(fst_file.contents())).unwrap();
    Ok(fst_set)
}

#[cfg(not(feature = "external-models"))]
pub(crate) fn read_test_data_file(language: Language, file_name: &str) -> std::io::Result<&str> {
    let directory = get_test_data_directory(language);
    let test_data_file = directory.get_file(file_name).ok_or(ErrorKind::NotFound)?;
    Ok(test_data_file.contents_utf8().unwrap())
}

#[cfg(feature = "external-models")]
pub(crate) fn read_probability_model_data_file(
    language: Language,
    file_name: &str,
) -> std::io::Result<LanguageModelFst> {
    let bytes = external::model_bytes(language, file_name)?;
    let fst_map = fst::Map::new(Cow::Borrowed(bytes)).unwrap();
    Ok(LanguageModelFst::from_fst_map(fst_map))
}

#[cfg(feature = "external-models")]
pub(crate) fn read_count_model_data_file(
    language: Language,
    file_name: &str,
) -> std::io::Result<CountModelFst> {
    let bytes = external::model_bytes(language, file_name)?;
    let fst_set = fst::Set::new(Cow::Borrowed(bytes)).unwrap();
    Ok(fst_set)
}

#[cfg(feature = "external-models")]
pub(crate) fn read_test_data_file(_language: Language, _file_name: &str) -> std::io::Result<&str> {
    Err(std::io::Error::new(
        ErrorKind::NotFound,
        "test data files are not part of external model blobs",
    ))
}

#[cfg(feature = "external-models")]
pub use external::load_models_from_blob;

/// Runtime loading of language models from a single blob file via mmap.
///
/// Blob layout (all integers little-endian):
/// - 8 bytes magic: `LINGUA\0\x01`
/// - u32 entry count
/// - per entry: u8 ISO 639-1 code length, code bytes, u8 file name length,
///   file name bytes, u64 payload offset, u64 payload length
/// - payloads (each aligned to 16 bytes)
#[cfg(feature = "external-models")]
mod external {
    use crate::Language;
    use crate::isocode::IsoCode639_1;
    use memmap2::Mmap;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{Error, ErrorKind, Result};
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use std::sync::{Mutex, OnceLock};

    pub(crate) const BLOB_MAGIC: &[u8; 8] = b"LINGUA\0\x01";

    /// Every loaded blob's models, merged.
    ///
    /// Several blobs rather than one, because the natural unit is one file per
    /// language: the full 75-language blob is 136.8 MB while a single language
    /// is ~2 MB, and a deployment that detects among six has no use for the
    /// other 69. Per-language files also match how every other model format
    /// here is distributed, so they bind through an asset set keyed by language
    /// and the deployment picks languages by picking keys — instead of baking a
    /// language choice into a filtered artifact.
    ///
    /// Each blob keeps its own mapping; nothing is concatenated or copied.
    #[derive(Default)]
    struct Registry {
        models: HashMap<(Language, &'static str), &'static [u8]>,
        /// Canonicalized paths already loaded, with the languages each carries,
        /// so loading the same file twice is a no-op instead of a
        /// duplicate-entry error — and still answers what it holds.
        loaded: Vec<(PathBuf, Vec<IsoCode639_1>)>,
    }

    static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();

    fn registry() -> &'static Mutex<Registry> {
        REGISTRY.get_or_init(|| Mutex::new(Registry::default()))
    }

    /// Memory-maps a language models blob and adds its models to the set
    /// available for the rest of the program.
    ///
    /// May be called more than once, with different blobs — that is the point,
    /// since the intended layout is one blob per language. Loading the same
    /// path twice is a no-op. Two blobs claiming the same language is an error:
    /// it means a deployment bound overlapping sets, and silently preferring
    /// one would make which models are in use depend on load order.
    ///
    /// Must be called before the first language detection. Alternatively, the
    /// path can be provided in the `LINGUA_MODELS_BLOB` environment variable,
    /// which accepts several paths separated by the platform path separator.
    ///
    /// Returns the languages this blob carries, sorted. A caller needs that to
    /// know what it may build a detector over, and the blob is the only honest
    /// source: a file name says whatever someone called it, and a blob holding
    /// 75 languages is as likely to be called `models.lingua` as anything else.
    pub fn load_models_from_blob(path: impl AsRef<Path>) -> Result<Vec<IsoCode639_1>> {
        let path = path.as_ref();
        let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let mut reg = registry()
            .lock()
            .map_err(|_| Error::new(ErrorKind::Other, "models registry poisoned"))?;
        if let Some((_, codes)) = reg.loaded.iter().find(|(p, _)| p == &canonical) {
            return Ok(codes.clone());
        }

        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let index = parse_index(&mmap)?;

        // Leaked deliberately: the models are borrowed as `&'static [u8]` for
        // the process lifetime, which the previous `OnceLock` guaranteed
        // implicitly. Leaking makes that lifetime true rather than assumed, and
        // a blob is never unloaded anyway.
        let mmap: &'static Mmap = Box::leak(Box::new(mmap));

        for (key, (offset, length)) in &index {
            if reg.models.contains_key(key) {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!(
                        "{}: models for {:?} are already loaded from another blob",
                        path.display(),
                        key.0
                    ),
                ));
            }
            reg.models.insert(*key, &mmap[*offset..*offset + *length]);
        }
        let mut codes: Vec<IsoCode639_1> = index
            .keys()
            .map(|(language, _)| language.iso_code_639_1())
            .collect();
        codes.sort_by_key(|c| c.to_string());
        codes.dedup();
        reg.loaded.push((canonical, codes.clone()));
        Ok(codes)
    }

    /// Load whatever `LINGUA_MODELS_BLOB` names, once.
    fn load_from_env() {
        static DONE: OnceLock<()> = OnceLock::new();
        DONE.get_or_init(|| {
            if let Ok(paths) = std::env::var("LINGUA_MODELS_BLOB") {
                for path in std::env::split_paths(&paths) {
                    // Best effort: a bad entry surfaces as a NotFound for the
                    // language that needed it, naming the language rather than
                    // the path — the same as any other missing model.
                    let _ = load_models_from_blob(path);
                }
            }
        });
    }

    pub(super) fn model_bytes(language: Language, file_name: &str) -> Result<&'static [u8]> {
        load_from_env();
        let reg = registry()
            .lock()
            .map_err(|_| Error::new(ErrorKind::Other, "models registry poisoned"))?;
        if reg.models.is_empty() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "no language models loaded; call lingua::load_models_from_blob() \
                 or set the LINGUA_MODELS_BLOB environment variable",
            ));
        }
        // Copying the slice reference out is what lets the lock drop here: the
        // bytes are `&'static`, so they outlive the guard. Called once per
        // (language, file) while building models, not per detection.
        reg.models
            .get(&(language, file_name))
            .copied()
            .ok_or_else(|| ErrorKind::NotFound.into())
    }

    /// File names are interned to canonical static strings so the index
    /// can be keyed without allocating on lookup.
    fn intern_file_name(name: &str) -> Option<&'static str> {
        ["ngrams.fst", "unique-ngrams.fst", "mostcommon-ngrams.fst"]
            .into_iter()
            .find(|known| *known == name)
    }

    fn parse_index(data: &[u8]) -> Result<HashMap<(Language, &'static str), (usize, usize)>> {
        let invalid = |message: &str| Error::new(ErrorKind::InvalidData, message.to_string());
        if data.len() < 12 || &data[0..8] != BLOB_MAGIC {
            return Err(invalid("not a lingua models blob"));
        }
        let entry_count = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;
        let mut index = HashMap::with_capacity(entry_count);
        let mut pos = 12;

        let mut take = |len: usize| -> Result<&[u8]> {
            let slice = data
                .get(pos..pos + len)
                .ok_or_else(|| invalid("truncated models blob"))?;
            pos += len;
            Ok(slice)
        };

        for _ in 0..entry_count {
            let code_len = take(1)?[0] as usize;
            let code = std::str::from_utf8(take(code_len)?)
                .map_err(|_| invalid("invalid language code in models blob"))?
                .to_string();
            let name_len = take(1)?[0] as usize;
            let name = std::str::from_utf8(take(name_len)?)
                .map_err(|_| invalid("invalid file name in models blob"))?
                .to_string();
            let offset = u64::from_le_bytes(take(8)?.try_into().unwrap()) as usize;
            let length = u64::from_le_bytes(take(8)?.try_into().unwrap()) as usize;

            if offset + length > data.len() {
                return Err(invalid("models blob entry out of bounds"));
            }
            let iso_code = IsoCode639_1::from_str(&code)
                .map_err(|_| invalid("unsupported language code in models blob"))?;
            let language = Language::from_iso_code_639_1(&iso_code);
            let file_name = intern_file_name(&name)
                .ok_or_else(|| invalid("unsupported file name in models blob"))?;
            index.insert((language, file_name), (offset, length));
        }
        Ok(index)
    }
}

/// Writes the language models of all languages enabled at compile time
/// into a single blob file for use with the `external-models` feature.
#[cfg(all(feature = "pack-models", not(feature = "external-models")))]
pub fn write_models_blob(output_path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    use std::io::Write;
    use strum::IntoEnumIterator;

    const BLOB_MAGIC: &[u8; 8] = b"LINGUA\0\x01";
    const FILE_NAMES: [&str; 3] = ["ngrams.fst", "unique-ngrams.fst", "mostcommon-ngrams.fst"];

    // A language may ship without unique ngrams: upstream's Luxembourgish has
    // no `unique-ngrams.fst`, since none of its ngrams are unique to it. The
    // embedded reader takes the missing file as "no unique-ngram model", which
    // matches nothing. An empty set matches nothing too, so the blob carries
    // one: detection is unchanged, and every language keeps all three entries,
    // which readers of the format (ours and axite-lingua's tools) rely on.
    let empty_set: Vec<u8> = fst::SetBuilder::memory().into_inner().unwrap();

    let mut entries: Vec<(String, &str, &[u8])> = vec![];
    for language in Language::iter() {
        let directory = get_language_models_directory(language);
        for file_name in FILE_NAMES {
            let contents = match directory.get_file(file_name) {
                Some(file) => file.contents(),
                None if file_name == "unique-ngrams.fst" => empty_set.as_slice(),
                None => return Err(ErrorKind::NotFound.into()),
            };
            entries.push((language.iso_code_639_1().to_string(), file_name, contents));
        }
    }

    let header_size: usize = 12
        + entries
            .iter()
            .map(|(code, name, _)| 1 + code.len() + 1 + name.len() + 16)
            .sum::<usize>();
    let align = |offset: usize| offset.div_ceil(16) * 16;

    let mut header = Vec::with_capacity(header_size);
    header.extend_from_slice(BLOB_MAGIC);
    header.extend_from_slice(&(entries.len() as u32).to_le_bytes());

    let mut offset = align(header_size);
    for (code, name, contents) in &entries {
        header.push(code.len() as u8);
        header.extend_from_slice(code.as_bytes());
        header.push(name.len() as u8);
        header.extend_from_slice(name.as_bytes());
        header.extend_from_slice(&(offset as u64).to_le_bytes());
        header.extend_from_slice(&(contents.len() as u64).to_le_bytes());
        offset = align(offset + contents.len());
    }
    assert_eq!(header.len(), header_size);

    let file = std::fs::File::create(output_path)?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(&header)?;
    let mut written = header_size;
    for (_, _, contents) in &entries {
        let padding = align(written) - written;
        writer.write_all(&vec![0; padding])?;
        writer.write_all(contents)?;
        written = align(written) + contents.len();
    }
    writer.flush()
}

#[cfg(not(feature = "external-models"))]
fn get_language_models_directory(language: Language) -> Dir<'static> {
    match language {
        #[cfg(all(feature = "afrikaans", not(feature = "external-models")))]
        Language::Afrikaans => AFRIKAANS_MODELS_DIRECTORY,

        #[cfg(all(feature = "albanian", not(feature = "external-models")))]
        Language::Albanian => ALBANIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "arabic", not(feature = "external-models")))]
        Language::Arabic => ARABIC_MODELS_DIRECTORY,

        #[cfg(all(feature = "armenian", not(feature = "external-models")))]
        Language::Armenian => ARMENIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "azerbaijani", not(feature = "external-models")))]
        Language::Azerbaijani => AZERBAIJANI_MODELS_DIRECTORY,

        #[cfg(all(feature = "basque", not(feature = "external-models")))]
        Language::Basque => BASQUE_MODELS_DIRECTORY,

        #[cfg(all(feature = "belarusian", not(feature = "external-models")))]
        Language::Belarusian => BELARUSIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "bengali", not(feature = "external-models")))]
        Language::Bengali => BENGALI_MODELS_DIRECTORY,

        #[cfg(all(feature = "bokmal", not(feature = "external-models")))]
        Language::Bokmal => BOKMAL_MODELS_DIRECTORY,

        #[cfg(all(feature = "bosnian", not(feature = "external-models")))]
        Language::Bosnian => BOSNIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "bulgarian", not(feature = "external-models")))]
        Language::Bulgarian => BULGARIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "catalan", not(feature = "external-models")))]
        Language::Catalan => CATALAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "chinese", not(feature = "external-models")))]
        Language::Chinese => CHINESE_MODELS_DIRECTORY,

        #[cfg(all(feature = "croatian", not(feature = "external-models")))]
        Language::Croatian => CROATIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "czech", not(feature = "external-models")))]
        Language::Czech => CZECH_MODELS_DIRECTORY,

        #[cfg(all(feature = "danish", not(feature = "external-models")))]
        Language::Danish => DANISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "dutch", not(feature = "external-models")))]
        Language::Dutch => DUTCH_MODELS_DIRECTORY,

        #[cfg(all(feature = "english", not(feature = "external-models")))]
        Language::English => ENGLISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "esperanto", not(feature = "external-models")))]
        Language::Esperanto => ESPERANTO_MODELS_DIRECTORY,

        #[cfg(all(feature = "estonian", not(feature = "external-models")))]
        Language::Estonian => ESTONIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "finnish", not(feature = "external-models")))]
        Language::Finnish => FINNISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "french", not(feature = "external-models")))]
        Language::French => FRENCH_MODELS_DIRECTORY,

        #[cfg(all(feature = "ganda", not(feature = "external-models")))]
        Language::Ganda => GANDA_MODELS_DIRECTORY,

        #[cfg(all(feature = "georgian", not(feature = "external-models")))]
        Language::Georgian => GEORGIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "german", not(feature = "external-models")))]
        Language::German => GERMAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "greek", not(feature = "external-models")))]
        Language::Greek => GREEK_MODELS_DIRECTORY,

        #[cfg(all(feature = "gujarati", not(feature = "external-models")))]
        Language::Gujarati => GUJARATI_MODELS_DIRECTORY,

        #[cfg(all(feature = "hebrew", not(feature = "external-models")))]
        Language::Hebrew => HEBREW_MODELS_DIRECTORY,

        #[cfg(all(feature = "hindi", not(feature = "external-models")))]
        Language::Hindi => HINDI_MODELS_DIRECTORY,

        #[cfg(all(feature = "hungarian", not(feature = "external-models")))]
        Language::Hungarian => HUNGARIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "icelandic", not(feature = "external-models")))]
        Language::Icelandic => ICELANDIC_MODELS_DIRECTORY,

        #[cfg(all(feature = "indonesian", not(feature = "external-models")))]
        Language::Indonesian => INDONESIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "irish", not(feature = "external-models")))]
        Language::Irish => IRISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "italian", not(feature = "external-models")))]
        Language::Italian => ITALIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "japanese", not(feature = "external-models")))]
        Language::Japanese => JAPANESE_MODELS_DIRECTORY,

        #[cfg(all(feature = "kazakh", not(feature = "external-models")))]
        Language::Kazakh => KAZAKH_MODELS_DIRECTORY,

        #[cfg(all(feature = "korean", not(feature = "external-models")))]
        Language::Korean => KOREAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "latin", not(feature = "external-models")))]
        Language::Latin => LATIN_MODELS_DIRECTORY,

        #[cfg(all(feature = "latvian", not(feature = "external-models")))]
        Language::Latvian => LATVIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "lithuanian", not(feature = "external-models")))]
        Language::Lithuanian => LITHUANIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "luxembourgish", not(feature = "external-models")))]
        Language::Luxembourgish => LUXEMBOURGISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "macedonian", not(feature = "external-models")))]
        Language::Macedonian => MACEDONIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "malay", not(feature = "external-models")))]
        Language::Malay => MALAY_MODELS_DIRECTORY,

        #[cfg(all(feature = "maori", not(feature = "external-models")))]
        Language::Maori => MAORI_MODELS_DIRECTORY,

        #[cfg(all(feature = "marathi", not(feature = "external-models")))]
        Language::Marathi => MARATHI_MODELS_DIRECTORY,

        #[cfg(all(feature = "mongolian", not(feature = "external-models")))]
        Language::Mongolian => MONGOLIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "nynorsk", not(feature = "external-models")))]
        Language::Nynorsk => NYNORSK_MODELS_DIRECTORY,

        #[cfg(all(feature = "persian", not(feature = "external-models")))]
        Language::Persian => PERSIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "polish", not(feature = "external-models")))]
        Language::Polish => POLISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "portuguese", not(feature = "external-models")))]
        Language::Portuguese => PORTUGUESE_MODELS_DIRECTORY,

        #[cfg(all(feature = "punjabi", not(feature = "external-models")))]
        Language::Punjabi => PUNJABI_MODELS_DIRECTORY,

        #[cfg(all(feature = "romanian", not(feature = "external-models")))]
        Language::Romanian => ROMANIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "russian", not(feature = "external-models")))]
        Language::Russian => RUSSIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "serbian", not(feature = "external-models")))]
        Language::Serbian => SERBIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "shona", not(feature = "external-models")))]
        Language::Shona => SHONA_MODELS_DIRECTORY,

        #[cfg(all(feature = "slovak", not(feature = "external-models")))]
        Language::Slovak => SLOVAK_MODELS_DIRECTORY,

        #[cfg(all(feature = "slovene", not(feature = "external-models")))]
        Language::Slovene => SLOVENE_MODELS_DIRECTORY,

        #[cfg(all(feature = "somali", not(feature = "external-models")))]
        Language::Somali => SOMALI_MODELS_DIRECTORY,

        #[cfg(all(feature = "sotho", not(feature = "external-models")))]
        Language::Sotho => SOTHO_MODELS_DIRECTORY,

        #[cfg(all(feature = "spanish", not(feature = "external-models")))]
        Language::Spanish => SPANISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "swahili", not(feature = "external-models")))]
        Language::Swahili => SWAHILI_MODELS_DIRECTORY,

        #[cfg(all(feature = "swedish", not(feature = "external-models")))]
        Language::Swedish => SWEDISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "tagalog", not(feature = "external-models")))]
        Language::Tagalog => TAGALOG_MODELS_DIRECTORY,

        #[cfg(all(feature = "tamil", not(feature = "external-models")))]
        Language::Tamil => TAMIL_MODELS_DIRECTORY,

        #[cfg(all(feature = "telugu", not(feature = "external-models")))]
        Language::Telugu => TELUGU_MODELS_DIRECTORY,

        #[cfg(all(feature = "thai", not(feature = "external-models")))]
        Language::Thai => THAI_MODELS_DIRECTORY,

        #[cfg(all(feature = "tsonga", not(feature = "external-models")))]
        Language::Tsonga => TSONGA_MODELS_DIRECTORY,

        #[cfg(all(feature = "tswana", not(feature = "external-models")))]
        Language::Tswana => TSWANA_MODELS_DIRECTORY,

        #[cfg(all(feature = "turkish", not(feature = "external-models")))]
        Language::Turkish => TURKISH_MODELS_DIRECTORY,

        #[cfg(all(feature = "ukrainian", not(feature = "external-models")))]
        Language::Ukrainian => UKRAINIAN_MODELS_DIRECTORY,

        #[cfg(all(feature = "urdu", not(feature = "external-models")))]
        Language::Urdu => URDU_MODELS_DIRECTORY,

        #[cfg(all(feature = "vietnamese", not(feature = "external-models")))]
        Language::Vietnamese => VIETNAMESE_MODELS_DIRECTORY,

        #[cfg(all(feature = "welsh", not(feature = "external-models")))]
        Language::Welsh => WELSH_MODELS_DIRECTORY,

        #[cfg(all(feature = "xhosa", not(feature = "external-models")))]
        Language::Xhosa => XHOSA_MODELS_DIRECTORY,

        #[cfg(all(feature = "yoruba", not(feature = "external-models")))]
        Language::Yoruba => YORUBA_MODELS_DIRECTORY,

        #[cfg(all(feature = "zulu", not(feature = "external-models")))]
        Language::Zulu => ZULU_MODELS_DIRECTORY,
    }
}

#[cfg(not(feature = "external-models"))]
fn get_test_data_directory(language: Language) -> Dir<'static> {
    match language {
        #[cfg(all(feature = "afrikaans", not(feature = "external-models")))]
        Language::Afrikaans => AFRIKAANS_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "albanian", not(feature = "external-models")))]
        Language::Albanian => ALBANIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "arabic", not(feature = "external-models")))]
        Language::Arabic => ARABIC_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "armenian", not(feature = "external-models")))]
        Language::Armenian => ARMENIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "azerbaijani", not(feature = "external-models")))]
        Language::Azerbaijani => AZERBAIJANI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "basque", not(feature = "external-models")))]
        Language::Basque => BASQUE_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "belarusian", not(feature = "external-models")))]
        Language::Belarusian => BELARUSIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "bengali", not(feature = "external-models")))]
        Language::Bengali => BENGALI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "bokmal", not(feature = "external-models")))]
        Language::Bokmal => BOKMAL_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "bosnian", not(feature = "external-models")))]
        Language::Bosnian => BOSNIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "bulgarian", not(feature = "external-models")))]
        Language::Bulgarian => BULGARIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "catalan", not(feature = "external-models")))]
        Language::Catalan => CATALAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "chinese", not(feature = "external-models")))]
        Language::Chinese => CHINESE_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "croatian", not(feature = "external-models")))]
        Language::Croatian => CROATIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "czech", not(feature = "external-models")))]
        Language::Czech => CZECH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "danish", not(feature = "external-models")))]
        Language::Danish => DANISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "dutch", not(feature = "external-models")))]
        Language::Dutch => DUTCH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "english", not(feature = "external-models")))]
        Language::English => ENGLISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "esperanto", not(feature = "external-models")))]
        Language::Esperanto => ESPERANTO_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "estonian", not(feature = "external-models")))]
        Language::Estonian => ESTONIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "finnish", not(feature = "external-models")))]
        Language::Finnish => FINNISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "french", not(feature = "external-models")))]
        Language::French => FRENCH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "ganda", not(feature = "external-models")))]
        Language::Ganda => GANDA_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "georgian", not(feature = "external-models")))]
        Language::Georgian => GEORGIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "german", not(feature = "external-models")))]
        Language::German => GERMAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "greek", not(feature = "external-models")))]
        Language::Greek => GREEK_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "gujarati", not(feature = "external-models")))]
        Language::Gujarati => GUJARATI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "hebrew", not(feature = "external-models")))]
        Language::Hebrew => HEBREW_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "hindi", not(feature = "external-models")))]
        Language::Hindi => HINDI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "hungarian", not(feature = "external-models")))]
        Language::Hungarian => HUNGARIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "icelandic", not(feature = "external-models")))]
        Language::Icelandic => ICELANDIC_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "indonesian", not(feature = "external-models")))]
        Language::Indonesian => INDONESIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "irish", not(feature = "external-models")))]
        Language::Irish => IRISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "italian", not(feature = "external-models")))]
        Language::Italian => ITALIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "japanese", not(feature = "external-models")))]
        Language::Japanese => JAPANESE_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "kazakh", not(feature = "external-models")))]
        Language::Kazakh => KAZAKH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "korean", not(feature = "external-models")))]
        Language::Korean => KOREAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "latin", not(feature = "external-models")))]
        Language::Latin => LATIN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "latvian", not(feature = "external-models")))]
        Language::Latvian => LATVIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "lithuanian", not(feature = "external-models")))]
        Language::Lithuanian => LITHUANIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "luxembourgish", not(feature = "external-models")))]
        Language::Luxembourgish => LUXEMBOURGISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "macedonian", not(feature = "external-models")))]
        Language::Macedonian => MACEDONIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "malay", not(feature = "external-models")))]
        Language::Malay => MALAY_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "maori", not(feature = "external-models")))]
        Language::Maori => MAORI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "marathi", not(feature = "external-models")))]
        Language::Marathi => MARATHI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "mongolian", not(feature = "external-models")))]
        Language::Mongolian => MONGOLIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "nynorsk", not(feature = "external-models")))]
        Language::Nynorsk => NYNORSK_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "persian", not(feature = "external-models")))]
        Language::Persian => PERSIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "polish", not(feature = "external-models")))]
        Language::Polish => POLISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "portuguese", not(feature = "external-models")))]
        Language::Portuguese => PORTUGUESE_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "punjabi", not(feature = "external-models")))]
        Language::Punjabi => PUNJABI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "romanian", not(feature = "external-models")))]
        Language::Romanian => ROMANIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "russian", not(feature = "external-models")))]
        Language::Russian => RUSSIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "serbian", not(feature = "external-models")))]
        Language::Serbian => SERBIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "shona", not(feature = "external-models")))]
        Language::Shona => SHONA_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "slovak", not(feature = "external-models")))]
        Language::Slovak => SLOVAK_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "slovene", not(feature = "external-models")))]
        Language::Slovene => SLOVENE_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "somali", not(feature = "external-models")))]
        Language::Somali => SOMALI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "sotho", not(feature = "external-models")))]
        Language::Sotho => SOTHO_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "spanish", not(feature = "external-models")))]
        Language::Spanish => SPANISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "swahili", not(feature = "external-models")))]
        Language::Swahili => SWAHILI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "swedish", not(feature = "external-models")))]
        Language::Swedish => SWEDISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "tagalog", not(feature = "external-models")))]
        Language::Tagalog => TAGALOG_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "tamil", not(feature = "external-models")))]
        Language::Tamil => TAMIL_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "telugu", not(feature = "external-models")))]
        Language::Telugu => TELUGU_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "thai", not(feature = "external-models")))]
        Language::Thai => THAI_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "tsonga", not(feature = "external-models")))]
        Language::Tsonga => TSONGA_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "tswana", not(feature = "external-models")))]
        Language::Tswana => TSWANA_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "turkish", not(feature = "external-models")))]
        Language::Turkish => TURKISH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "ukrainian", not(feature = "external-models")))]
        Language::Ukrainian => UKRAINIAN_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "urdu", not(feature = "external-models")))]
        Language::Urdu => URDU_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "vietnamese", not(feature = "external-models")))]
        Language::Vietnamese => VIETNAMESE_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "welsh", not(feature = "external-models")))]
        Language::Welsh => WELSH_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "xhosa", not(feature = "external-models")))]
        Language::Xhosa => XHOSA_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "yoruba", not(feature = "external-models")))]
        Language::Yoruba => YORUBA_TESTDATA_DIRECTORY,

        #[cfg(all(feature = "zulu", not(feature = "external-models")))]
        Language::Zulu => ZULU_TESTDATA_DIRECTORY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{NGRAM_PROBABILITY_MODEL_FILE_NAME, NgramCountModelType};

    #[test]
    fn test_read_probability_model_data_file() {
        let result =
            read_probability_model_data_file(Language::English, NGRAM_PROBABILITY_MODEL_FILE_NAME);
        assert!(result.is_ok());

        let fst_map = result.unwrap();
        assert!(fst_map.contains_key(b"that".to_vec()));
    }

    #[test]
    fn test_read_count_model_data_file() {
        let result = read_count_model_data_file(
            Language::English,
            &NgramCountModelType::MostCommon.file_name(),
        );
        assert!(result.is_ok());
    }
}
