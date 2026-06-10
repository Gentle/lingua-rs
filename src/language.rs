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

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Result};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use crate::alphabet::Alphabet;
use crate::isocode::{IsoCode639_1, IsoCode639_3};

/// This enum specifies the so far 75 supported languages which can be detected by *Lingua*.
#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(feature = "accuracy-reports", derive(clap::ValueEnum))]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(
        eq,
        eq_int,
        frozen,
        hash,
        ord,
        rename_all = "UPPERCASE",
        module = "lingua"
    )
)]
pub enum Language {
    #[cfg(any(feature = "afrikaans", feature = "external-models"))]
    Afrikaans,

    #[cfg(any(feature = "albanian", feature = "external-models"))]
    Albanian,

    #[cfg(any(feature = "arabic", feature = "external-models"))]
    Arabic,

    #[cfg(any(feature = "armenian", feature = "external-models"))]
    Armenian,

    #[cfg(any(feature = "azerbaijani", feature = "external-models"))]
    Azerbaijani,

    #[cfg(any(feature = "basque", feature = "external-models"))]
    Basque,

    #[cfg(any(feature = "belarusian", feature = "external-models"))]
    Belarusian,

    #[cfg(any(feature = "bengali", feature = "external-models"))]
    Bengali,

    #[cfg(any(feature = "bokmal", feature = "external-models"))]
    Bokmal,

    #[cfg(any(feature = "bosnian", feature = "external-models"))]
    Bosnian,

    #[cfg(any(feature = "bulgarian", feature = "external-models"))]
    Bulgarian,

    #[cfg(any(feature = "catalan", feature = "external-models"))]
    Catalan,

    #[cfg(any(feature = "chinese", feature = "external-models"))]
    Chinese,

    #[cfg(any(feature = "croatian", feature = "external-models"))]
    Croatian,

    #[cfg(any(feature = "czech", feature = "external-models"))]
    Czech,

    #[cfg(any(feature = "danish", feature = "external-models"))]
    Danish,

    #[cfg(any(feature = "dutch", feature = "external-models"))]
    Dutch,

    #[cfg(any(feature = "english", feature = "external-models"))]
    English,

    #[cfg(any(feature = "esperanto", feature = "external-models"))]
    Esperanto,

    #[cfg(any(feature = "estonian", feature = "external-models"))]
    Estonian,

    #[cfg(any(feature = "finnish", feature = "external-models"))]
    Finnish,

    #[cfg(any(feature = "french", feature = "external-models"))]
    French,

    #[cfg(any(feature = "ganda", feature = "external-models"))]
    Ganda,

    #[cfg(any(feature = "georgian", feature = "external-models"))]
    Georgian,

    #[cfg(any(feature = "german", feature = "external-models"))]
    German,

    #[cfg(any(feature = "greek", feature = "external-models"))]
    Greek,

    #[cfg(any(feature = "gujarati", feature = "external-models"))]
    Gujarati,

    #[cfg(any(feature = "hebrew", feature = "external-models"))]
    Hebrew,

    #[cfg(any(feature = "hindi", feature = "external-models"))]
    Hindi,

    #[cfg(any(feature = "hungarian", feature = "external-models"))]
    Hungarian,

    #[cfg(any(feature = "icelandic", feature = "external-models"))]
    Icelandic,

    #[cfg(any(feature = "indonesian", feature = "external-models"))]
    Indonesian,

    #[cfg(any(feature = "irish", feature = "external-models"))]
    Irish,

    #[cfg(any(feature = "italian", feature = "external-models"))]
    Italian,

    #[cfg(any(feature = "japanese", feature = "external-models"))]
    Japanese,

    #[cfg(any(feature = "kazakh", feature = "external-models"))]
    Kazakh,

    #[cfg(any(feature = "korean", feature = "external-models"))]
    Korean,

    #[cfg(any(feature = "latin", feature = "external-models"))]
    Latin,

    #[cfg(any(feature = "latvian", feature = "external-models"))]
    Latvian,

    #[cfg(any(feature = "lithuanian", feature = "external-models"))]
    Lithuanian,

    #[cfg(any(feature = "macedonian", feature = "external-models"))]
    Macedonian,

    #[cfg(any(feature = "malay", feature = "external-models"))]
    Malay,

    #[cfg(any(feature = "maori", feature = "external-models"))]
    Maori,

    #[cfg(any(feature = "marathi", feature = "external-models"))]
    Marathi,

    #[cfg(any(feature = "mongolian", feature = "external-models"))]
    Mongolian,

    #[cfg(any(feature = "nynorsk", feature = "external-models"))]
    Nynorsk,

    #[cfg(any(feature = "persian", feature = "external-models"))]
    Persian,

    #[cfg(any(feature = "polish", feature = "external-models"))]
    Polish,

    #[cfg(any(feature = "portuguese", feature = "external-models"))]
    Portuguese,

    #[cfg(any(feature = "punjabi", feature = "external-models"))]
    Punjabi,

    #[cfg(any(feature = "romanian", feature = "external-models"))]
    Romanian,

    #[cfg(any(feature = "russian", feature = "external-models"))]
    Russian,

    #[cfg(any(feature = "serbian", feature = "external-models"))]
    Serbian,

    #[cfg(any(feature = "shona", feature = "external-models"))]
    Shona,

    #[cfg(any(feature = "slovak", feature = "external-models"))]
    Slovak,

    #[cfg(any(feature = "slovene", feature = "external-models"))]
    Slovene,

    #[cfg(any(feature = "somali", feature = "external-models"))]
    Somali,

    #[cfg(any(feature = "sotho", feature = "external-models"))]
    Sotho,

    #[cfg(any(feature = "spanish", feature = "external-models"))]
    Spanish,

    #[cfg(any(feature = "swahili", feature = "external-models"))]
    Swahili,

    #[cfg(any(feature = "swedish", feature = "external-models"))]
    Swedish,

    #[cfg(any(feature = "tagalog", feature = "external-models"))]
    Tagalog,

    #[cfg(any(feature = "tamil", feature = "external-models"))]
    Tamil,

    #[cfg(any(feature = "telugu", feature = "external-models"))]
    Telugu,

    #[cfg(any(feature = "thai", feature = "external-models"))]
    Thai,

    #[cfg(any(feature = "tsonga", feature = "external-models"))]
    Tsonga,

    #[cfg(any(feature = "tswana", feature = "external-models"))]
    Tswana,

    #[cfg(any(feature = "turkish", feature = "external-models"))]
    Turkish,

    #[cfg(any(feature = "ukrainian", feature = "external-models"))]
    Ukrainian,

    #[cfg(any(feature = "urdu", feature = "external-models"))]
    Urdu,

    #[cfg(any(feature = "vietnamese", feature = "external-models"))]
    Vietnamese,

    #[cfg(any(feature = "welsh", feature = "external-models"))]
    Welsh,

    #[cfg(any(feature = "xhosa", feature = "external-models"))]
    Xhosa,

    #[cfg(any(feature = "yoruba", feature = "external-models"))]
    Yoruba,

    #[cfg(any(feature = "zulu", feature = "external-models"))]
    Zulu,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr)
    }
}

impl Language {
    /// Returns a set of all supported languages.
    pub fn all() -> HashSet<Language> {
        Language::iter().collect()
    }

    /// Returns a set of all supported spoken languages.
    pub fn all_spoken_ones() -> HashSet<Language> {
        Language::iter()
            .filter(|it| {
                if cfg!(any(feature = "latin", feature = "external-models")) {
                    it != &<Language as FromStr>::from_str("Latin").unwrap()
                } else {
                    true
                }
            })
            .collect()
    }

    /// Returns a set of all languages supporting the Arabic script.
    pub fn all_with_arabic_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Arabic))
            .collect()
    }

    /// Returns a set of all languages supporting the Cyrillic script.
    pub fn all_with_cyrillic_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Cyrillic))
            .collect()
    }

    /// Returns a set of all languages supporting the Devanagari script.
    pub fn all_with_devanagari_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Devanagari))
            .collect()
    }

    /// Returns a set of all languages supporting the Latin script.
    pub fn all_with_latin_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Latin))
            .collect()
    }

    /// Returns a set of all languages supporting a single unique script.
    pub fn all_with_single_unique_script() -> HashSet<Language> {
        let single_language_alphabets = Alphabet::all_supporting_single_language();
        Language::iter()
            .filter(|it| {
                it.alphabets().len() == 1
                    && single_language_alphabets.contains_key(it.alphabets().iter().next().unwrap())
            })
            .collect()
    }

    /// Returns the language associated with the ISO 639-1 code
    /// passed to this method.
    pub fn from_iso_code_639_1(iso_code: &IsoCode639_1) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_1() == iso_code)
            .unwrap()
    }

    /// Returns the language associated with the ISO 639-3 code
    /// passed to this method.
    pub fn from_iso_code_639_3(iso_code: &IsoCode639_3) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_3() == iso_code)
            .unwrap()
    }

    /// Returns the ISO 639-1 code of this language.
    pub fn iso_code_639_1(&self) -> IsoCode639_1 {
        match self {
            #[cfg(any(feature = "afrikaans", feature = "external-models"))]
            Language::Afrikaans => IsoCode639_1::AF,

            #[cfg(any(feature = "albanian", feature = "external-models"))]
            Language::Albanian => IsoCode639_1::SQ,

            #[cfg(any(feature = "arabic", feature = "external-models"))]
            Language::Arabic => IsoCode639_1::AR,

            #[cfg(any(feature = "armenian", feature = "external-models"))]
            Language::Armenian => IsoCode639_1::HY,

            #[cfg(any(feature = "azerbaijani", feature = "external-models"))]
            Language::Azerbaijani => IsoCode639_1::AZ,

            #[cfg(any(feature = "basque", feature = "external-models"))]
            Language::Basque => IsoCode639_1::EU,

            #[cfg(any(feature = "belarusian", feature = "external-models"))]
            Language::Belarusian => IsoCode639_1::BE,

            #[cfg(any(feature = "bengali", feature = "external-models"))]
            Language::Bengali => IsoCode639_1::BN,

            #[cfg(any(feature = "bokmal", feature = "external-models"))]
            Language::Bokmal => IsoCode639_1::NB,

            #[cfg(any(feature = "bosnian", feature = "external-models"))]
            Language::Bosnian => IsoCode639_1::BS,

            #[cfg(any(feature = "bulgarian", feature = "external-models"))]
            Language::Bulgarian => IsoCode639_1::BG,

            #[cfg(any(feature = "catalan", feature = "external-models"))]
            Language::Catalan => IsoCode639_1::CA,

            #[cfg(any(feature = "chinese", feature = "external-models"))]
            Language::Chinese => IsoCode639_1::ZH,

            #[cfg(any(feature = "croatian", feature = "external-models"))]
            Language::Croatian => IsoCode639_1::HR,

            #[cfg(any(feature = "czech", feature = "external-models"))]
            Language::Czech => IsoCode639_1::CS,

            #[cfg(any(feature = "danish", feature = "external-models"))]
            Language::Danish => IsoCode639_1::DA,

            #[cfg(any(feature = "dutch", feature = "external-models"))]
            Language::Dutch => IsoCode639_1::NL,

            #[cfg(any(feature = "english", feature = "external-models"))]
            Language::English => IsoCode639_1::EN,

            #[cfg(any(feature = "esperanto", feature = "external-models"))]
            Language::Esperanto => IsoCode639_1::EO,

            #[cfg(any(feature = "estonian", feature = "external-models"))]
            Language::Estonian => IsoCode639_1::ET,

            #[cfg(any(feature = "finnish", feature = "external-models"))]
            Language::Finnish => IsoCode639_1::FI,

            #[cfg(any(feature = "french", feature = "external-models"))]
            Language::French => IsoCode639_1::FR,

            #[cfg(any(feature = "ganda", feature = "external-models"))]
            Language::Ganda => IsoCode639_1::LG,

            #[cfg(any(feature = "georgian", feature = "external-models"))]
            Language::Georgian => IsoCode639_1::KA,

            #[cfg(any(feature = "german", feature = "external-models"))]
            Language::German => IsoCode639_1::DE,

            #[cfg(any(feature = "greek", feature = "external-models"))]
            Language::Greek => IsoCode639_1::EL,

            #[cfg(any(feature = "gujarati", feature = "external-models"))]
            Language::Gujarati => IsoCode639_1::GU,

            #[cfg(any(feature = "hebrew", feature = "external-models"))]
            Language::Hebrew => IsoCode639_1::HE,

            #[cfg(any(feature = "hindi", feature = "external-models"))]
            Language::Hindi => IsoCode639_1::HI,

            #[cfg(any(feature = "hungarian", feature = "external-models"))]
            Language::Hungarian => IsoCode639_1::HU,

            #[cfg(any(feature = "icelandic", feature = "external-models"))]
            Language::Icelandic => IsoCode639_1::IS,

            #[cfg(any(feature = "indonesian", feature = "external-models"))]
            Language::Indonesian => IsoCode639_1::ID,

            #[cfg(any(feature = "irish", feature = "external-models"))]
            Language::Irish => IsoCode639_1::GA,

            #[cfg(any(feature = "italian", feature = "external-models"))]
            Language::Italian => IsoCode639_1::IT,

            #[cfg(any(feature = "japanese", feature = "external-models"))]
            Language::Japanese => IsoCode639_1::JA,

            #[cfg(any(feature = "kazakh", feature = "external-models"))]
            Language::Kazakh => IsoCode639_1::KK,

            #[cfg(any(feature = "korean", feature = "external-models"))]
            Language::Korean => IsoCode639_1::KO,

            #[cfg(any(feature = "latin", feature = "external-models"))]
            Language::Latin => IsoCode639_1::LA,

            #[cfg(any(feature = "latvian", feature = "external-models"))]
            Language::Latvian => IsoCode639_1::LV,

            #[cfg(any(feature = "lithuanian", feature = "external-models"))]
            Language::Lithuanian => IsoCode639_1::LT,

            #[cfg(any(feature = "macedonian", feature = "external-models"))]
            Language::Macedonian => IsoCode639_1::MK,

            #[cfg(any(feature = "malay", feature = "external-models"))]
            Language::Malay => IsoCode639_1::MS,

            #[cfg(any(feature = "maori", feature = "external-models"))]
            Language::Maori => IsoCode639_1::MI,

            #[cfg(any(feature = "marathi", feature = "external-models"))]
            Language::Marathi => IsoCode639_1::MR,

            #[cfg(any(feature = "mongolian", feature = "external-models"))]
            Language::Mongolian => IsoCode639_1::MN,

            #[cfg(any(feature = "nynorsk", feature = "external-models"))]
            Language::Nynorsk => IsoCode639_1::NN,

            #[cfg(any(feature = "persian", feature = "external-models"))]
            Language::Persian => IsoCode639_1::FA,

            #[cfg(any(feature = "polish", feature = "external-models"))]
            Language::Polish => IsoCode639_1::PL,

            #[cfg(any(feature = "portuguese", feature = "external-models"))]
            Language::Portuguese => IsoCode639_1::PT,

            #[cfg(any(feature = "punjabi", feature = "external-models"))]
            Language::Punjabi => IsoCode639_1::PA,

            #[cfg(any(feature = "romanian", feature = "external-models"))]
            Language::Romanian => IsoCode639_1::RO,

            #[cfg(any(feature = "russian", feature = "external-models"))]
            Language::Russian => IsoCode639_1::RU,

            #[cfg(any(feature = "serbian", feature = "external-models"))]
            Language::Serbian => IsoCode639_1::SR,

            #[cfg(any(feature = "shona", feature = "external-models"))]
            Language::Shona => IsoCode639_1::SN,

            #[cfg(any(feature = "slovak", feature = "external-models"))]
            Language::Slovak => IsoCode639_1::SK,

            #[cfg(any(feature = "slovene", feature = "external-models"))]
            Language::Slovene => IsoCode639_1::SL,

            #[cfg(any(feature = "somali", feature = "external-models"))]
            Language::Somali => IsoCode639_1::SO,

            #[cfg(any(feature = "sotho", feature = "external-models"))]
            Language::Sotho => IsoCode639_1::ST,

            #[cfg(any(feature = "spanish", feature = "external-models"))]
            Language::Spanish => IsoCode639_1::ES,

            #[cfg(any(feature = "swahili", feature = "external-models"))]
            Language::Swahili => IsoCode639_1::SW,

            #[cfg(any(feature = "swedish", feature = "external-models"))]
            Language::Swedish => IsoCode639_1::SV,

            #[cfg(any(feature = "tagalog", feature = "external-models"))]
            Language::Tagalog => IsoCode639_1::TL,

            #[cfg(any(feature = "tamil", feature = "external-models"))]
            Language::Tamil => IsoCode639_1::TA,

            #[cfg(any(feature = "telugu", feature = "external-models"))]
            Language::Telugu => IsoCode639_1::TE,

            #[cfg(any(feature = "thai", feature = "external-models"))]
            Language::Thai => IsoCode639_1::TH,

            #[cfg(any(feature = "tsonga", feature = "external-models"))]
            Language::Tsonga => IsoCode639_1::TS,

            #[cfg(any(feature = "tswana", feature = "external-models"))]
            Language::Tswana => IsoCode639_1::TN,

            #[cfg(any(feature = "turkish", feature = "external-models"))]
            Language::Turkish => IsoCode639_1::TR,

            #[cfg(any(feature = "ukrainian", feature = "external-models"))]
            Language::Ukrainian => IsoCode639_1::UK,

            #[cfg(any(feature = "urdu", feature = "external-models"))]
            Language::Urdu => IsoCode639_1::UR,

            #[cfg(any(feature = "vietnamese", feature = "external-models"))]
            Language::Vietnamese => IsoCode639_1::VI,

            #[cfg(any(feature = "welsh", feature = "external-models"))]
            Language::Welsh => IsoCode639_1::CY,

            #[cfg(any(feature = "xhosa", feature = "external-models"))]
            Language::Xhosa => IsoCode639_1::XH,

            #[cfg(any(feature = "yoruba", feature = "external-models"))]
            Language::Yoruba => IsoCode639_1::YO,

            #[cfg(any(feature = "zulu", feature = "external-models"))]
            Language::Zulu => IsoCode639_1::ZU,
        }
    }

    /// Returns the ISO 639-3 code of this language.
    pub fn iso_code_639_3(&self) -> IsoCode639_3 {
        match self {
            #[cfg(any(feature = "afrikaans", feature = "external-models"))]
            Language::Afrikaans => IsoCode639_3::AFR,

            #[cfg(any(feature = "albanian", feature = "external-models"))]
            Language::Albanian => IsoCode639_3::SQI,

            #[cfg(any(feature = "arabic", feature = "external-models"))]
            Language::Arabic => IsoCode639_3::ARA,

            #[cfg(any(feature = "armenian", feature = "external-models"))]
            Language::Armenian => IsoCode639_3::HYE,

            #[cfg(any(feature = "azerbaijani", feature = "external-models"))]
            Language::Azerbaijani => IsoCode639_3::AZE,

            #[cfg(any(feature = "basque", feature = "external-models"))]
            Language::Basque => IsoCode639_3::EUS,

            #[cfg(any(feature = "belarusian", feature = "external-models"))]
            Language::Belarusian => IsoCode639_3::BEL,

            #[cfg(any(feature = "bengali", feature = "external-models"))]
            Language::Bengali => IsoCode639_3::BEN,

            #[cfg(any(feature = "bokmal", feature = "external-models"))]
            Language::Bokmal => IsoCode639_3::NOB,

            #[cfg(any(feature = "bosnian", feature = "external-models"))]
            Language::Bosnian => IsoCode639_3::BOS,

            #[cfg(any(feature = "bulgarian", feature = "external-models"))]
            Language::Bulgarian => IsoCode639_3::BUL,

            #[cfg(any(feature = "catalan", feature = "external-models"))]
            Language::Catalan => IsoCode639_3::CAT,

            #[cfg(any(feature = "chinese", feature = "external-models"))]
            Language::Chinese => IsoCode639_3::ZHO,

            #[cfg(any(feature = "croatian", feature = "external-models"))]
            Language::Croatian => IsoCode639_3::HRV,

            #[cfg(any(feature = "czech", feature = "external-models"))]
            Language::Czech => IsoCode639_3::CES,

            #[cfg(any(feature = "danish", feature = "external-models"))]
            Language::Danish => IsoCode639_3::DAN,

            #[cfg(any(feature = "dutch", feature = "external-models"))]
            Language::Dutch => IsoCode639_3::NLD,

            #[cfg(any(feature = "english", feature = "external-models"))]
            Language::English => IsoCode639_3::ENG,

            #[cfg(any(feature = "esperanto", feature = "external-models"))]
            Language::Esperanto => IsoCode639_3::EPO,

            #[cfg(any(feature = "estonian", feature = "external-models"))]
            Language::Estonian => IsoCode639_3::EST,

            #[cfg(any(feature = "finnish", feature = "external-models"))]
            Language::Finnish => IsoCode639_3::FIN,

            #[cfg(any(feature = "french", feature = "external-models"))]
            Language::French => IsoCode639_3::FRA,

            #[cfg(any(feature = "ganda", feature = "external-models"))]
            Language::Ganda => IsoCode639_3::LUG,

            #[cfg(any(feature = "georgian", feature = "external-models"))]
            Language::Georgian => IsoCode639_3::KAT,

            #[cfg(any(feature = "german", feature = "external-models"))]
            Language::German => IsoCode639_3::DEU,

            #[cfg(any(feature = "greek", feature = "external-models"))]
            Language::Greek => IsoCode639_3::ELL,

            #[cfg(any(feature = "gujarati", feature = "external-models"))]
            Language::Gujarati => IsoCode639_3::GUJ,

            #[cfg(any(feature = "hebrew", feature = "external-models"))]
            Language::Hebrew => IsoCode639_3::HEB,

            #[cfg(any(feature = "hindi", feature = "external-models"))]
            Language::Hindi => IsoCode639_3::HIN,

            #[cfg(any(feature = "hungarian", feature = "external-models"))]
            Language::Hungarian => IsoCode639_3::HUN,

            #[cfg(any(feature = "icelandic", feature = "external-models"))]
            Language::Icelandic => IsoCode639_3::ISL,

            #[cfg(any(feature = "indonesian", feature = "external-models"))]
            Language::Indonesian => IsoCode639_3::IND,

            #[cfg(any(feature = "irish", feature = "external-models"))]
            Language::Irish => IsoCode639_3::GLE,

            #[cfg(any(feature = "italian", feature = "external-models"))]
            Language::Italian => IsoCode639_3::ITA,

            #[cfg(any(feature = "japanese", feature = "external-models"))]
            Language::Japanese => IsoCode639_3::JPN,

            #[cfg(any(feature = "kazakh", feature = "external-models"))]
            Language::Kazakh => IsoCode639_3::KAZ,

            #[cfg(any(feature = "korean", feature = "external-models"))]
            Language::Korean => IsoCode639_3::KOR,

            #[cfg(any(feature = "latin", feature = "external-models"))]
            Language::Latin => IsoCode639_3::LAT,

            #[cfg(any(feature = "latvian", feature = "external-models"))]
            Language::Latvian => IsoCode639_3::LAV,

            #[cfg(any(feature = "lithuanian", feature = "external-models"))]
            Language::Lithuanian => IsoCode639_3::LIT,

            #[cfg(any(feature = "macedonian", feature = "external-models"))]
            Language::Macedonian => IsoCode639_3::MKD,

            #[cfg(any(feature = "malay", feature = "external-models"))]
            Language::Malay => IsoCode639_3::MSA,

            #[cfg(any(feature = "maori", feature = "external-models"))]
            Language::Maori => IsoCode639_3::MRI,

            #[cfg(any(feature = "marathi", feature = "external-models"))]
            Language::Marathi => IsoCode639_3::MAR,

            #[cfg(any(feature = "mongolian", feature = "external-models"))]
            Language::Mongolian => IsoCode639_3::MON,

            #[cfg(any(feature = "nynorsk", feature = "external-models"))]
            Language::Nynorsk => IsoCode639_3::NNO,

            #[cfg(any(feature = "persian", feature = "external-models"))]
            Language::Persian => IsoCode639_3::FAS,

            #[cfg(any(feature = "polish", feature = "external-models"))]
            Language::Polish => IsoCode639_3::POL,

            #[cfg(any(feature = "portuguese", feature = "external-models"))]
            Language::Portuguese => IsoCode639_3::POR,

            #[cfg(any(feature = "punjabi", feature = "external-models"))]
            Language::Punjabi => IsoCode639_3::PAN,

            #[cfg(any(feature = "romanian", feature = "external-models"))]
            Language::Romanian => IsoCode639_3::RON,

            #[cfg(any(feature = "russian", feature = "external-models"))]
            Language::Russian => IsoCode639_3::RUS,

            #[cfg(any(feature = "serbian", feature = "external-models"))]
            Language::Serbian => IsoCode639_3::SRP,

            #[cfg(any(feature = "shona", feature = "external-models"))]
            Language::Shona => IsoCode639_3::SNA,

            #[cfg(any(feature = "slovak", feature = "external-models"))]
            Language::Slovak => IsoCode639_3::SLK,

            #[cfg(any(feature = "slovene", feature = "external-models"))]
            Language::Slovene => IsoCode639_3::SLV,

            #[cfg(any(feature = "somali", feature = "external-models"))]
            Language::Somali => IsoCode639_3::SOM,

            #[cfg(any(feature = "sotho", feature = "external-models"))]
            Language::Sotho => IsoCode639_3::SOT,

            #[cfg(any(feature = "spanish", feature = "external-models"))]
            Language::Spanish => IsoCode639_3::SPA,

            #[cfg(any(feature = "swahili", feature = "external-models"))]
            Language::Swahili => IsoCode639_3::SWA,

            #[cfg(any(feature = "swedish", feature = "external-models"))]
            Language::Swedish => IsoCode639_3::SWE,

            #[cfg(any(feature = "tagalog", feature = "external-models"))]
            Language::Tagalog => IsoCode639_3::TGL,

            #[cfg(any(feature = "tamil", feature = "external-models"))]
            Language::Tamil => IsoCode639_3::TAM,

            #[cfg(any(feature = "telugu", feature = "external-models"))]
            Language::Telugu => IsoCode639_3::TEL,

            #[cfg(any(feature = "thai", feature = "external-models"))]
            Language::Thai => IsoCode639_3::THA,

            #[cfg(any(feature = "tsonga", feature = "external-models"))]
            Language::Tsonga => IsoCode639_3::TSO,

            #[cfg(any(feature = "tswana", feature = "external-models"))]
            Language::Tswana => IsoCode639_3::TSN,

            #[cfg(any(feature = "turkish", feature = "external-models"))]
            Language::Turkish => IsoCode639_3::TUR,

            #[cfg(any(feature = "ukrainian", feature = "external-models"))]
            Language::Ukrainian => IsoCode639_3::UKR,

            #[cfg(any(feature = "urdu", feature = "external-models"))]
            Language::Urdu => IsoCode639_3::URD,

            #[cfg(any(feature = "vietnamese", feature = "external-models"))]
            Language::Vietnamese => IsoCode639_3::VIE,

            #[cfg(any(feature = "welsh", feature = "external-models"))]
            Language::Welsh => IsoCode639_3::CYM,

            #[cfg(any(feature = "xhosa", feature = "external-models"))]
            Language::Xhosa => IsoCode639_3::XHO,

            #[cfg(any(feature = "yoruba", feature = "external-models"))]
            Language::Yoruba => IsoCode639_3::YOR,

            #[cfg(any(feature = "zulu", feature = "external-models"))]
            Language::Zulu => IsoCode639_3::ZUL,
        }
    }

    pub(crate) fn alphabets(&self) -> HashSet<Alphabet> {
        match self {
            #[cfg(any(feature = "afrikaans", feature = "external-models"))]
            Language::Afrikaans => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "albanian", feature = "external-models"))]
            Language::Albanian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "azerbaijani", feature = "external-models"))]
            Language::Azerbaijani => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "basque", feature = "external-models"))]
            Language::Basque => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "bokmal", feature = "external-models"))]
            Language::Bokmal => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "bosnian", feature = "external-models"))]
            Language::Bosnian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "catalan", feature = "external-models"))]
            Language::Catalan => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "croatian", feature = "external-models"))]
            Language::Croatian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "czech", feature = "external-models"))]
            Language::Czech => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "danish", feature = "external-models"))]
            Language::Danish => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "dutch", feature = "external-models"))]
            Language::Dutch => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "english", feature = "external-models"))]
            Language::English => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "esperanto", feature = "external-models"))]
            Language::Esperanto => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "estonian", feature = "external-models"))]
            Language::Estonian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "finnish", feature = "external-models"))]
            Language::Finnish => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "french", feature = "external-models"))]
            Language::French => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "ganda", feature = "external-models"))]
            Language::Ganda => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "german", feature = "external-models"))]
            Language::German => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "hungarian", feature = "external-models"))]
            Language::Hungarian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "icelandic", feature = "external-models"))]
            Language::Icelandic => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "indonesian", feature = "external-models"))]
            Language::Indonesian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "irish", feature = "external-models"))]
            Language::Irish => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "italian", feature = "external-models"))]
            Language::Italian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "latin", feature = "external-models"))]
            Language::Latin => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "latvian", feature = "external-models"))]
            Language::Latvian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "lithuanian", feature = "external-models"))]
            Language::Lithuanian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "malay", feature = "external-models"))]
            Language::Malay => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "maori", feature = "external-models"))]
            Language::Maori => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "nynorsk", feature = "external-models"))]
            Language::Nynorsk => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "polish", feature = "external-models"))]
            Language::Polish => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "portuguese", feature = "external-models"))]
            Language::Portuguese => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "romanian", feature = "external-models"))]
            Language::Romanian => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "shona", feature = "external-models"))]
            Language::Shona => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "slovak", feature = "external-models"))]
            Language::Slovak => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "slovene", feature = "external-models"))]
            Language::Slovene => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "somali", feature = "external-models"))]
            Language::Somali => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "sotho", feature = "external-models"))]
            Language::Sotho => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "spanish", feature = "external-models"))]
            Language::Spanish => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "swahili", feature = "external-models"))]
            Language::Swahili => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "swedish", feature = "external-models"))]
            Language::Swedish => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "tagalog", feature = "external-models"))]
            Language::Tagalog => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "tsonga", feature = "external-models"))]
            Language::Tsonga => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "tswana", feature = "external-models"))]
            Language::Tswana => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "turkish", feature = "external-models"))]
            Language::Turkish => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "vietnamese", feature = "external-models"))]
            Language::Vietnamese => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "welsh", feature = "external-models"))]
            Language::Welsh => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "xhosa", feature = "external-models"))]
            Language::Xhosa => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "yoruba", feature = "external-models"))]
            Language::Yoruba => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "zulu", feature = "external-models"))]
            Language::Zulu => hashset!(Alphabet::Latin),

            #[cfg(any(feature = "belarusian", feature = "external-models"))]
            Language::Belarusian => hashset!(Alphabet::Cyrillic),

            #[cfg(any(feature = "bulgarian", feature = "external-models"))]
            Language::Bulgarian => hashset!(Alphabet::Cyrillic),

            #[cfg(any(feature = "kazakh", feature = "external-models"))]
            Language::Kazakh => hashset!(Alphabet::Cyrillic),

            #[cfg(any(feature = "macedonian", feature = "external-models"))]
            Language::Macedonian => hashset!(Alphabet::Cyrillic),

            #[cfg(any(feature = "mongolian", feature = "external-models"))]
            Language::Mongolian => hashset!(Alphabet::Cyrillic),

            #[cfg(any(feature = "russian", feature = "external-models"))]
            Language::Russian => hashset!(Alphabet::Cyrillic),

            #[cfg(any(feature = "serbian", feature = "external-models"))]
            Language::Serbian => hashset!(Alphabet::Cyrillic),

            #[cfg(any(feature = "ukrainian", feature = "external-models"))]
            Language::Ukrainian => hashset!(Alphabet::Cyrillic),

            #[cfg(any(feature = "arabic", feature = "external-models"))]
            Language::Arabic => hashset!(Alphabet::Arabic),

            #[cfg(any(feature = "persian", feature = "external-models"))]
            Language::Persian => hashset!(Alphabet::Arabic),

            #[cfg(any(feature = "urdu", feature = "external-models"))]
            Language::Urdu => hashset!(Alphabet::Arabic),

            #[cfg(any(feature = "hindi", feature = "external-models"))]
            Language::Hindi => hashset!(Alphabet::Devanagari),

            #[cfg(any(feature = "marathi", feature = "external-models"))]
            Language::Marathi => hashset!(Alphabet::Devanagari),

            #[cfg(any(feature = "armenian", feature = "external-models"))]
            Language::Armenian => hashset!(Alphabet::Armenian),

            #[cfg(any(feature = "bengali", feature = "external-models"))]
            Language::Bengali => hashset!(Alphabet::Bengali),

            #[cfg(any(feature = "chinese", feature = "external-models"))]
            Language::Chinese => hashset!(Alphabet::Han),

            #[cfg(any(feature = "georgian", feature = "external-models"))]
            Language::Georgian => hashset!(Alphabet::Georgian),

            #[cfg(any(feature = "greek", feature = "external-models"))]
            Language::Greek => hashset!(Alphabet::Greek),

            #[cfg(any(feature = "gujarati", feature = "external-models"))]
            Language::Gujarati => hashset!(Alphabet::Gujarati),

            #[cfg(any(feature = "hebrew", feature = "external-models"))]
            Language::Hebrew => hashset!(Alphabet::Hebrew),

            #[cfg(any(feature = "japanese", feature = "external-models"))]
            Language::Japanese => hashset!(Alphabet::Hiragana, Alphabet::Katakana, Alphabet::Han),

            #[cfg(any(feature = "korean", feature = "external-models"))]
            Language::Korean => hashset!(Alphabet::Hangul),

            #[cfg(any(feature = "punjabi", feature = "external-models"))]
            Language::Punjabi => hashset!(Alphabet::Gurmukhi),

            #[cfg(any(feature = "tamil", feature = "external-models"))]
            Language::Tamil => hashset!(Alphabet::Tamil),

            #[cfg(any(feature = "telugu", feature = "external-models"))]
            Language::Telugu => hashset!(Alphabet::Telugu),

            #[cfg(any(feature = "thai", feature = "external-models"))]
            Language::Thai => hashset!(Alphabet::Thai),
        }
    }

    #[allow(unreachable_patterns)]
    pub(crate) fn unique_characters(&self) -> Option<&str> {
        match self {
            #[cfg(any(feature = "azerbaijani", feature = "external-models"))]
            Language::Azerbaijani => Some("Əə"),

            #[cfg(any(feature = "catalan", feature = "external-models"))]
            Language::Catalan => Some("Ïï"),

            #[cfg(any(feature = "czech", feature = "external-models"))]
            Language::Czech => Some("ĚěŘřŮů"),

            #[cfg(any(feature = "esperanto", feature = "external-models"))]
            Language::Esperanto => Some("ĈĉĜĝĤĥĴĵŜŝŬŭ"),

            #[cfg(any(feature = "german", feature = "external-models"))]
            Language::German => Some("ß"),

            #[cfg(any(feature = "hungarian", feature = "external-models"))]
            Language::Hungarian => Some("ŐőŰű"),

            #[cfg(any(feature = "kazakh", feature = "external-models"))]
            Language::Kazakh => Some("ӘәҒғҚқҢңҰұ"),

            #[cfg(any(feature = "latvian", feature = "external-models"))]
            Language::Latvian => Some("ĢģĶķĻļŅņ"),

            #[cfg(any(feature = "lithuanian", feature = "external-models"))]
            Language::Lithuanian => Some("ĖėĮįŲų"),

            #[cfg(any(feature = "macedonian", feature = "external-models"))]
            Language::Macedonian => Some("ЃѓЅѕЌќЏџ"),

            #[cfg(any(feature = "marathi", feature = "external-models"))]
            Language::Marathi => Some("ळ"),

            #[cfg(any(feature = "polish", feature = "external-models"))]
            Language::Polish => Some("ŁłŃńŚśŹź"),

            #[cfg(any(feature = "romanian", feature = "external-models"))]
            Language::Romanian => Some("Țţ"),

            #[cfg(any(feature = "serbian", feature = "external-models"))]
            Language::Serbian => Some("ЂђЋћ"),

            #[cfg(any(feature = "slovak", feature = "external-models"))]
            Language::Slovak => Some("ĹĺĽľŔŕ"),

            #[cfg(any(feature = "spanish", feature = "external-models"))]
            Language::Spanish => Some("¿¡"),

            #[cfg(any(feature = "ukrainian", feature = "external-models"))]
            Language::Ukrainian => Some("ҐґЄєЇї"),

            #[cfg(any(feature = "vietnamese", feature = "external-models"))]
            Language::Vietnamese => Some(
                "ẰằẦầẲẳẨẩẴẵẪẫẮắẤấẠạẶặẬậỀềẺẻỂểẼẽỄễẾếỆệỈỉĨĩỊịƠơỒồỜờỎỏỔổỞởỖỗỠỡỐốỚớỘộỢợƯưỪừỦủỬửŨũỮữỨứỤụỰựỲỳỶỷỸỹỴỵ",
            ),

            #[cfg(any(feature = "yoruba", feature = "external-models"))]
            Language::Yoruba => Some("Ṣṣ"),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::language::Language::*;
    use std::str::FromStr;
    use strum::ParseError::VariantNotFound;

    use super::*;

    #[test]
    fn assert_language_string_representation_is_correct() {
        assert_eq!(English.to_string(), "English");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Language::from_str("english"), Ok(English));
        assert_eq!(Language::from_str("foo"), Err(VariantNotFound));
    }

    #[test]
    fn assert_all_languages_are_available() {
        assert_eq!(
            Language::all(),
            hashset!(
                Afrikaans,
                Albanian,
                Arabic,
                Armenian,
                Azerbaijani,
                Basque,
                Belarusian,
                Bengali,
                Bokmal,
                Bosnian,
                Bulgarian,
                Catalan,
                Chinese,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                Georgian,
                German,
                Greek,
                Gujarati,
                Hebrew,
                Hindi,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Japanese,
                Kazakh,
                Korean,
                Latin,
                Latvian,
                Lithuanian,
                Macedonian,
                Malay,
                Maori,
                Marathi,
                Mongolian,
                Nynorsk,
                Persian,
                Polish,
                Portuguese,
                Punjabi,
                Romanian,
                Russian,
                Serbian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tamil,
                Telugu,
                Thai,
                Tsonga,
                Tswana,
                Turkish,
                Ukrainian,
                Urdu,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }

    #[test]
    fn assert_all_spoken_languages_are_available() {
        assert_eq!(
            Language::all_spoken_ones(),
            hashset!(
                Afrikaans,
                Albanian,
                Arabic,
                Armenian,
                Azerbaijani,
                Basque,
                Belarusian,
                Bengali,
                Bokmal,
                Bosnian,
                Bulgarian,
                Catalan,
                Chinese,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                Georgian,
                German,
                Greek,
                Gujarati,
                Hebrew,
                Hindi,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Japanese,
                Kazakh,
                Korean,
                Latvian,
                Lithuanian,
                Macedonian,
                Malay,
                Maori,
                Marathi,
                Mongolian,
                Nynorsk,
                Persian,
                Polish,
                Portuguese,
                Punjabi,
                Romanian,
                Russian,
                Serbian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tamil,
                Telugu,
                Thai,
                Tsonga,
                Tswana,
                Turkish,
                Ukrainian,
                Urdu,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }

    #[test]
    fn assert_certain_languages_support_arabic_script() {
        assert_eq!(
            Language::all_with_arabic_script(),
            hashset!(Arabic, Persian, Urdu)
        );
    }

    #[test]
    fn assert_certain_languages_support_cyrillic_script() {
        assert_eq!(
            Language::all_with_cyrillic_script(),
            hashset!(
                Belarusian, Bulgarian, Kazakh, Macedonian, Mongolian, Russian, Serbian, Ukrainian
            )
        );
    }

    #[test]
    fn assert_certain_languages_support_devanagari_script() {
        assert_eq!(
            Language::all_with_devanagari_script(),
            hashset!(Hindi, Marathi)
        );
    }

    #[test]
    fn assert_certain_languages_support_latin_script() {
        assert_eq!(
            Language::all_with_latin_script(),
            hashset!(
                Afrikaans,
                Albanian,
                Azerbaijani,
                Basque,
                Bokmal,
                Bosnian,
                Catalan,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                German,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Latin,
                Latvian,
                Lithuanian,
                Malay,
                Maori,
                Nynorsk,
                Polish,
                Portuguese,
                Romanian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tsonga,
                Tswana,
                Turkish,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }

    #[test]
    fn test_languages_with_single_unique_script() {
        assert_eq!(
            Language::all_with_single_unique_script(),
            hashset!(
                Armenian, Bengali, Georgian, Greek, Gujarati, Punjabi, Korean, Hebrew, Tamil,
                Telugu, Thai
            )
        );
    }
}
