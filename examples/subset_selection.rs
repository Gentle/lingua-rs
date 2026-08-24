//! What happens when the detector's language set is wider than the blobs loaded.
//!
//! With per-language blobs this is the obvious mistake: install three
//! languages, then build a detector from all 75. Whether that fails loudly or
//! degrades quietly decides how the wrapper must behave.

use lingua::{IsoCode639_1, Language, LanguageDetectorBuilder};
use std::str::FromStr;

fn main() {
    let dir = std::env::args().nth(1).expect("usage: subset_selection <dir>");
    let mut codes: Vec<String> = vec![];
    for entry in std::fs::read_dir(&dir).expect("read dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("lingua") {
            continue;
        }
        lingua::load_models_from_blob(&path).expect("load");
        codes.push(path.file_stem().unwrap().to_str().unwrap().to_string());
    }
    codes.sort();
    println!("loaded: {}", codes.join(", "));

    // 1. The supported case: detector set == loaded set.
    let langs: Vec<Language> = codes
        .iter()
        .map(|c| Language::from_iso_code_639_1(&IsoCode639_1::from_str(c).unwrap()))
        .collect();
    let ok = LanguageDetectorBuilder::from_languages(&langs)
        .with_preloaded_language_models()
        .build();
    println!(
        "from_languages(loaded): de -> {:?}",
        ok.detect_language_of("Der schnelle braune Hund springt über den faulen Fuchs")
    );

    // 2. The mistake: ask for every language when only a few are present.
    println!("from_all_languages() with a partial set:");
    let result = std::panic::catch_unwind(|| {
        let all = LanguageDetectorBuilder::from_all_languages()
            .with_preloaded_language_models()
            .build();
        all.detect_language_of("Der schnelle braune Hund springt über den faulen Fuchs")
    });
    match result {
        Ok(got) => println!("  did NOT fail — returned {got:?}"),
        Err(_) => println!("  panicked (loud failure)"),
    }

    // 3. The consequence: text in a language whose models are absent. If this
    //    comes back as one of the loaded languages, a caller who believes all
    //    75 are installed gets a confident wrong answer rather than a miss.
    let spanish = "El perro marrón rápido salta sobre el zorro perezoso cada día";
    let all = LanguageDetectorBuilder::from_all_languages()
        .with_preloaded_language_models()
        .build();
    println!("Spanish text, es NOT loaded:");
    println!("  from_all_languages() -> {:?}", all.detect_language_of(spanish));
    println!("  from_languages(loaded) -> {:?}", ok.detect_language_of(spanish));
}
