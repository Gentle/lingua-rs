//! Load several per-language blobs and detect across them.
//!
//! The layout `ax-models lingua-split` produces: one `<iso>.lingua` per
//! language, each its own mapping. Upstream held a single blob in a `OnceLock`
//! and refused a second, so this is the case the registry exists for.
//!
//!     cargo run --release --example split_blobs --features external-models -- dir/

use lingua::{IsoCode639_1, Language, LanguageDetectorBuilder};
use std::str::FromStr;

fn main() {
    let dir = std::env::args().nth(1).expect("usage: split_blobs <dir>");

    let mut codes: Vec<String> = vec![];
    for entry in std::fs::read_dir(&dir).expect("read dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("lingua") {
            continue;
        }
        lingua::load_models_from_blob(&path)
            .unwrap_or_else(|e| panic!("load {}: {e}", path.display()));
        codes.push(path.file_stem().unwrap().to_str().unwrap().to_string());
    }
    codes.sort();
    assert!(!codes.is_empty(), "no .lingua blobs in {dir}");
    println!("loaded {} blobs: {}", codes.len(), codes.join(", "));

    // Loading the same path again must be a no-op, not an error.
    let first = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().and_then(|x| x.to_str()) == Some("lingua"))
        .unwrap();
    lingua::load_models_from_blob(&first).expect("re-loading the same blob must be a no-op");
    println!("re-load of the same blob: ok");

    let languages: Vec<Language> = codes
        .iter()
        .map(|c| Language::from_iso_code_639_1(&IsoCode639_1::from_str(c).unwrap()))
        .collect();
    let detector = LanguageDetectorBuilder::from_languages(&languages)
        .with_preloaded_language_models()
        .build();

    // One sentence per language we loaded; each must come back as itself.
    let samples: &[(&str, &str)] = &[
        ("de", "Der schnelle braune Hund springt über den faulen Fuchs"),
        ("en", "The quick brown dog jumps over the lazy fox today"),
        ("fr", "Le chien brun rapide saute par dessus le renard paresseux"),
        ("nl", "De snelle bruine hond springt over de luie vos heen"),
        ("es", "El perro marrón rápido salta sobre el zorro perezoso"),
    ];
    let mut checked = 0;
    for (iso, text) in samples {
        if !codes.iter().any(|c| c == iso) {
            continue;
        }
        let want = Language::from_iso_code_639_1(&IsoCode639_1::from_str(iso).unwrap());
        let got = detector.detect_language_of(*text);
        println!("  {iso}: {got:?} (want {want:?})");
        assert_eq!(got, Some(want), "misdetected {iso}");
        checked += 1;
    }
    assert!(checked >= 2, "only {checked} languages checked");
    println!("{checked} languages detected correctly from separate blobs");
}
