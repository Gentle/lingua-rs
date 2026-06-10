use lingua::{Language, LanguageDetectorBuilder};

fn main() {
    let blob_path = std::env::args().nth(1).expect("usage: external_blob <blob>");
    lingua::load_models_from_blob(&blob_path).expect("failed to load models blob");

    let detector = LanguageDetectorBuilder::from_all_languages()
        .with_preloaded_language_models()
        .build();

    let cases = [
        ("en", Language::English),
        ("de", Language::German),
        ("fr", Language::French),
        ("es", Language::Spanish),
        ("pt", Language::Portuguese),
        ("da", Language::Danish),
        ("nb", Language::Bokmal),
        ("zh", Language::Chinese),
    ];

    for (iso, language) in cases {
        for file in ["sentences.txt", "single-words.txt"] {
            let path = format!("language-models/{iso}/testdata/{file}");
            if !std::path::Path::new(&path).exists() {
                continue;
            }
            let content = std::fs::read_to_string(&path).unwrap();
            let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
            let correct = lines
                .iter()
                .filter(|line| detector.detect_language_of(**line) == Some(language))
                .count();
            println!(
                "{language:?} {file}: {correct}/{} ({:.2}%)",
                lines.len(),
                correct as f64 * 100.0 / lines.len() as f64
            );
        }
    }
}
