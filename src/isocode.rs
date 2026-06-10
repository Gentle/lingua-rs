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

use std::fmt::{Debug, Display, Formatter, Result};

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

/// This enum specifies the ISO 639-1 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
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
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord, module = "lingua")
)]
pub enum IsoCode639_1 {
    #[cfg(any(feature = "afrikaans", feature = "external-models"))]
    /// The ISO 639-1 code for [`Afrikaans`](crate::language::Language::Afrikaans)
    AF,

    #[cfg(any(feature = "arabic", feature = "external-models"))]
    /// The ISO 639-1 code for [`Arabic`](crate::language::Language::Arabic)
    AR,

    #[cfg(any(feature = "azerbaijani", feature = "external-models"))]
    /// The ISO 639-1 code for [`Azerbaijani`](crate::language::Language::Azerbaijani)
    AZ,

    #[cfg(any(feature = "belarusian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Belarusian`](crate::language::Language::Belarusian)
    BE,

    #[cfg(any(feature = "bulgarian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Bulgarian`](crate::language::Language::Bulgarian)
    BG,

    #[cfg(any(feature = "bengali", feature = "external-models"))]
    /// The ISO 639-1 code for [`Bengali`](crate::language::Language::Bengali)
    BN,

    #[cfg(any(feature = "bosnian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Bosnian`](crate::language::Language::Bosnian)
    BS,

    #[cfg(any(feature = "catalan", feature = "external-models"))]
    /// The ISO 639-1 code for [`Catalan`](crate::language::Language::Catalan)
    CA,

    #[cfg(any(feature = "czech", feature = "external-models"))]
    /// The ISO 639-1 code for [`Czech`](crate::language::Language::Czech)
    CS,

    #[cfg(any(feature = "welsh", feature = "external-models"))]
    /// The ISO 639-1 code for [`Welsh`](crate::language::Language::Welsh)
    CY,

    #[cfg(any(feature = "danish", feature = "external-models"))]
    /// The ISO 639-1 code for [`Danish`](crate::language::Language::Danish)
    DA,

    #[cfg(any(feature = "german", feature = "external-models"))]
    /// The ISO 639-1 code for [`German`](crate::language::Language::German)
    DE,

    #[cfg(any(feature = "greek", feature = "external-models"))]
    /// The ISO 639-1 code for [`Greek`](crate::language::Language::Greek)
    EL,

    #[cfg(any(feature = "english", feature = "external-models"))]
    /// The ISO 639-1 code for [`English`](crate::language::Language::English)
    EN,

    #[cfg(any(feature = "esperanto", feature = "external-models"))]
    /// The ISO 639-1 code for [`Esperanto`](crate::language::Language::Esperanto)
    EO,

    #[cfg(any(feature = "spanish", feature = "external-models"))]
    /// The ISO 639-1 code for [`Spanish`](crate::language::Language::Spanish)
    ES,

    #[cfg(any(feature = "estonian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Estonian`](crate::language::Language::Estonian)
    ET,

    #[cfg(any(feature = "basque", feature = "external-models"))]
    /// The ISO 639-1 code for [`Basque`](crate::language::Language::Basque)
    EU,

    #[cfg(any(feature = "persian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Persian`](crate::language::Language::Persian)
    FA,

    #[cfg(any(feature = "finnish", feature = "external-models"))]
    /// The ISO 639-1 code for [`Finnish`](crate::language::Language::Finnish)
    FI,

    #[cfg(any(feature = "french", feature = "external-models"))]
    /// The ISO 639-1 code for [`French`](crate::language::Language::French)
    FR,

    #[cfg(any(feature = "irish", feature = "external-models"))]
    /// The ISO 639-1 code for [`Irish`](crate::language::Language::Irish)
    GA,

    #[cfg(any(feature = "gujarati", feature = "external-models"))]
    /// The ISO 639-1 code for [`Gujarati`](crate::language::Language::Gujarati)
    GU,

    #[cfg(any(feature = "hebrew", feature = "external-models"))]
    /// The ISO 639-1 code for [`Hebrew`](crate::language::Language::Hebrew)
    HE,

    #[cfg(any(feature = "hindi", feature = "external-models"))]
    /// The ISO 639-1 code for [`Hindi`](crate::language::Language::Hindi)
    HI,

    #[cfg(any(feature = "croatian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Croatian`](crate::language::Language::Croatian)
    HR,

    #[cfg(any(feature = "hungarian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Hungarian`](crate::language::Language::Hungarian)
    HU,

    #[cfg(any(feature = "armenian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Armenian`](crate::language::Language::Armenian)
    HY,

    #[cfg(any(feature = "indonesian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Indonesian`](crate::language::Language::Indonesian)
    ID,

    #[cfg(any(feature = "icelandic", feature = "external-models"))]
    /// The ISO 639-1 code for [`Icelandic`](crate::language::Language::Icelandic)
    IS,

    #[cfg(any(feature = "italian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Italian`](crate::language::Language::Italian)
    IT,

    #[cfg(any(feature = "japanese", feature = "external-models"))]
    /// The ISO 639-1 code for [`Japanese`](crate::language::Language::Japanese)
    JA,

    #[cfg(any(feature = "georgian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Georgian`](crate::language::Language::Georgian)
    KA,

    #[cfg(any(feature = "kazakh", feature = "external-models"))]
    /// The ISO 639-1 code for [`Kazakh`](crate::language::Language::Kazakh)
    KK,

    #[cfg(any(feature = "korean", feature = "external-models"))]
    /// The ISO 639-1 code for [`Korean`](crate::language::Language::Korean)
    KO,

    #[cfg(any(feature = "latin", feature = "external-models"))]
    /// The ISO 639-1 code for [`Latin`](crate::language::Language::Latin)
    LA,

    #[cfg(any(feature = "ganda", feature = "external-models"))]
    /// The ISO 639-1 code for [`Ganda`](crate::language::Language::Ganda)
    LG,

    #[cfg(any(feature = "lithuanian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Lithuanian`](crate::language::Language::Lithuanian)
    LT,

    #[cfg(any(feature = "latvian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Latvian`](crate::language::Language::Latvian)
    LV,

    #[cfg(any(feature = "maori", feature = "external-models"))]
    /// The ISO 639-1 code for [`Maori`](crate::language::Language::Maori)
    MI,

    #[cfg(any(feature = "macedonian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Macedonian`](crate::language::Language::Macedonian)
    MK,

    #[cfg(any(feature = "mongolian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Mongolian`](crate::language::Language::Mongolian)
    MN,

    #[cfg(any(feature = "marathi", feature = "external-models"))]
    /// The ISO 639-1 code for [`Marathi`](crate::language::Language::Marathi)
    MR,

    #[cfg(any(feature = "malay", feature = "external-models"))]
    /// The ISO 639-1 code for [`Malay`](crate::language::Language::Malay)
    MS,

    #[cfg(any(feature = "bokmal", feature = "external-models"))]
    /// The ISO 639-1 code for [`Norwegian Bokmal`](crate::language::Language::Bokmal)
    NB,

    #[cfg(any(feature = "dutch", feature = "external-models"))]
    /// The ISO 639-1 code for [`Dutch`](crate::language::Language::Dutch)
    NL,

    #[cfg(any(feature = "nynorsk", feature = "external-models"))]
    /// The ISO 639-1 code for [`Norwegian Nynorsk`](crate::language::Language::Nynorsk)
    NN,

    #[cfg(any(feature = "punjabi", feature = "external-models"))]
    /// The ISO 639-1 code for [`Punjabi`](crate::language::Language::Punjabi)
    PA,

    #[cfg(any(feature = "polish", feature = "external-models"))]
    /// The ISO 639-1 code for [`Polish`](crate::language::Language::Polish)
    PL,

    #[cfg(any(feature = "portuguese", feature = "external-models"))]
    /// The ISO 639-1 code for [`Portuguese`](crate::language::Language::Portuguese)
    PT,

    #[cfg(any(feature = "romanian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Romanian`](crate::language::Language::Romanian)
    RO,

    #[cfg(any(feature = "russian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Russian`](crate::language::Language::Russian)
    RU,

    #[cfg(any(feature = "slovak", feature = "external-models"))]
    /// The ISO 639-1 code for [`Slovak`](crate::language::Language::Slovak)
    SK,

    #[cfg(any(feature = "slovene", feature = "external-models"))]
    /// The ISO 639-1 code for [`Slovene`](crate::language::Language::Slovene)
    SL,

    #[cfg(any(feature = "shona", feature = "external-models"))]
    /// The ISO 639-1 code for [`Shona`](crate::language::Language::Shona)
    SN,

    #[cfg(any(feature = "somali", feature = "external-models"))]
    /// The ISO 639-1 code for [`Somali`](crate::language::Language::Somali)
    SO,

    #[cfg(any(feature = "albanian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Albanian`](crate::language::Language::Albanian)
    SQ,

    #[cfg(any(feature = "serbian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Serbian`](crate::language::Language::Serbian)
    SR,

    #[cfg(any(feature = "sotho", feature = "external-models"))]
    /// The ISO 639-1 code for [`Sotho`](crate::language::Language::Sotho)
    ST,

    #[cfg(any(feature = "swedish", feature = "external-models"))]
    /// The ISO 639-1 code for [`Swedish`](crate::language::Language::Swedish)
    SV,

    #[cfg(any(feature = "swahili", feature = "external-models"))]
    /// The ISO 639-1 code for [`Swahili`](crate::language::Language::Swahili)
    SW,

    #[cfg(any(feature = "tamil", feature = "external-models"))]
    /// The ISO 639-1 code for [`Tamil`](crate::language::Language::Tamil)
    TA,

    #[cfg(any(feature = "telugu", feature = "external-models"))]
    /// The ISO 639-1 code for [`Telugu`](crate::language::Language::Telugu)
    TE,

    #[cfg(any(feature = "thai", feature = "external-models"))]
    /// The ISO 639-1 code for [`Thai`](crate::language::Language::Thai)
    TH,

    #[cfg(any(feature = "tagalog", feature = "external-models"))]
    /// The ISO 639-1 code for [`Tagalog`](crate::language::Language::Tagalog)
    TL,

    #[cfg(any(feature = "tswana", feature = "external-models"))]
    /// The ISO 639-1 code for [`Tswana`](crate::language::Language::Tswana)
    TN,

    #[cfg(any(feature = "turkish", feature = "external-models"))]
    /// The ISO 639-1 code for [`Turkish`](crate::language::Language::Turkish)
    TR,

    #[cfg(any(feature = "tsonga", feature = "external-models"))]
    /// The ISO 639-1 code for [`Tsonga`](crate::language::Language::Tsonga)
    TS,

    #[cfg(any(feature = "ukrainian", feature = "external-models"))]
    /// The ISO 639-1 code for [`Ukrainian`](crate::language::Language::Ukrainian)
    UK,

    #[cfg(any(feature = "urdu", feature = "external-models"))]
    /// The ISO 639-1 code for [`Urdu`](crate::language::Language::Urdu)
    UR,

    #[cfg(any(feature = "vietnamese", feature = "external-models"))]
    /// The ISO 639-1 code for [`Vietnamese`](crate::language::Language::Vietnamese)
    VI,

    #[cfg(any(feature = "xhosa", feature = "external-models"))]
    /// The ISO 639-1 code for [`Xhosa`](crate::language::Language::Xhosa)
    XH,

    #[cfg(any(feature = "yoruba", feature = "external-models"))]
    /// The ISO 639-1 code for [`Yoruba`](crate::language::Language::Yoruba)
    YO,

    #[cfg(any(feature = "chinese", feature = "external-models"))]
    /// The ISO 639-1 code for [`Chinese`](crate::language::Language::Chinese)
    ZH,

    #[cfg(any(feature = "zulu", feature = "external-models"))]
    /// The ISO 639-1 code for [`Zulu`](crate::language::Language::Zulu)
    ZU,
}

/// This enum specifies the ISO 639-3 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
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
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord, module = "lingua")
)]
pub enum IsoCode639_3 {
    #[cfg(any(feature = "afrikaans", feature = "external-models"))]
    /// The ISO 639-3 code for [`Afrikaans`](crate::language::Language::Afrikaans)
    AFR,

    #[cfg(any(feature = "arabic", feature = "external-models"))]
    /// The ISO 639-3 code for [`Arabic`](crate::language::Language::Arabic)
    ARA,

    #[cfg(any(feature = "azerbaijani", feature = "external-models"))]
    /// The ISO 639-3 code for [`Azerbaijani`](crate::language::Language::Azerbaijani)
    AZE,

    #[cfg(any(feature = "belarusian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Belarusian`](crate::language::Language::Belarusian)
    BEL,

    #[cfg(any(feature = "bengali", feature = "external-models"))]
    /// The ISO 639-3 code for [`Bengali`](crate::language::Language::Bengali)
    BEN,

    #[cfg(any(feature = "bosnian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Bosnian`](crate::language::Language::Bosnian)
    BOS,

    #[cfg(any(feature = "bulgarian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Bulgarian`](crate::language::Language::Bulgarian)
    BUL,

    #[cfg(any(feature = "catalan", feature = "external-models"))]
    /// The ISO 639-3 code for [`Catalan`](crate::language::Language::Catalan)
    CAT,

    #[cfg(any(feature = "czech", feature = "external-models"))]
    /// The ISO 639-3 code for [`Czech`](crate::language::Language::Czech)
    CES,

    #[cfg(any(feature = "welsh", feature = "external-models"))]
    /// The ISO 639-3 code for [`Welsh`](crate::language::Language::Welsh)
    CYM,

    #[cfg(any(feature = "danish", feature = "external-models"))]
    /// The ISO 639-3 code for [`Danish`](crate::language::Language::Danish)
    DAN,

    #[cfg(any(feature = "german", feature = "external-models"))]
    /// The ISO 639-3 code for [`German`](crate::language::Language::German)
    DEU,

    #[cfg(any(feature = "greek", feature = "external-models"))]
    /// The ISO 639-3 code for [`Greek`](crate::language::Language::Greek)
    ELL,

    #[cfg(any(feature = "english", feature = "external-models"))]
    /// The ISO 639-3 code for [`English`](crate::language::Language::English)
    ENG,

    #[cfg(any(feature = "esperanto", feature = "external-models"))]
    /// The ISO 639-3 code for [`Esperanto`](crate::language::Language::Esperanto)
    EPO,

    #[cfg(any(feature = "estonian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Estonian`](crate::language::Language::Estonian)
    EST,

    #[cfg(any(feature = "basque", feature = "external-models"))]
    /// The ISO 639-3 code for [`Basque`](crate::language::Language::Basque)
    EUS,

    #[cfg(any(feature = "persian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Persian`](crate::language::Language::Persian)
    FAS,

    #[cfg(any(feature = "finnish", feature = "external-models"))]
    /// The ISO 639-3 code for [`Finnish`](crate::language::Language::Finnish)
    FIN,

    #[cfg(any(feature = "french", feature = "external-models"))]
    /// The ISO 639-3 code for [`French`](crate::language::Language::French)
    FRA,

    #[cfg(any(feature = "irish", feature = "external-models"))]
    /// The ISO 639-3 code for [`Irish`](crate::language::Language::Irish)
    GLE,

    #[cfg(any(feature = "gujarati", feature = "external-models"))]
    /// The ISO 639-3 code for [`Gujarati`](crate::language::Language::Gujarati)
    GUJ,

    #[cfg(any(feature = "hebrew", feature = "external-models"))]
    /// The ISO 639-3 code for [`Hebrew`](crate::language::Language::Hebrew)
    HEB,

    #[cfg(any(feature = "hindi", feature = "external-models"))]
    /// The ISO 639-3 code for [`Hindi`](crate::language::Language::Hindi)
    HIN,

    #[cfg(any(feature = "croatian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Croatian`](crate::language::Language::Croatian)
    HRV,

    #[cfg(any(feature = "hungarian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Hungarian`](crate::language::Language::Hungarian)
    HUN,

    #[cfg(any(feature = "armenian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Armenian`](crate::language::Language::Armenian)
    HYE,

    #[cfg(any(feature = "indonesian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Indonesian`](crate::language::Language::Indonesian)
    IND,

    #[cfg(any(feature = "icelandic", feature = "external-models"))]
    /// The ISO 639-3 code for [`Icelandic`](crate::language::Language::Icelandic)
    ISL,

    #[cfg(any(feature = "italian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Italian`](crate::language::Language::Italian)
    ITA,

    #[cfg(any(feature = "japanese", feature = "external-models"))]
    /// The ISO 639-3 code for [`Japanese`](crate::language::Language::Japanese)
    JPN,

    #[cfg(any(feature = "georgian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Georgian`](crate::language::Language::Georgian)
    KAT,

    #[cfg(any(feature = "kazakh", feature = "external-models"))]
    /// The ISO 639-3 code for [`Kazakh`](crate::language::Language::Kazakh)
    KAZ,

    #[cfg(any(feature = "korean", feature = "external-models"))]
    /// The ISO 639-3 code for [`Korean`](crate::language::Language::Korean)
    KOR,

    #[cfg(any(feature = "latin", feature = "external-models"))]
    /// The ISO 639-3 code for [`Latin`](crate::language::Language::Latin)
    LAT,

    #[cfg(any(feature = "latvian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Latvian`](crate::language::Language::Latvian)
    LAV,

    #[cfg(any(feature = "lithuanian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Lithuanian`](crate::language::Language::Lithuanian)
    LIT,

    #[cfg(any(feature = "ganda", feature = "external-models"))]
    /// The ISO 639-3 code for [`Ganda`](crate::language::Language::Ganda)
    LUG,

    #[cfg(any(feature = "marathi", feature = "external-models"))]
    /// The ISO 639-3 code for [`Marathi`](crate::language::Language::Marathi)
    MAR,

    #[cfg(any(feature = "macedonian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Macedonian`](crate::language::Language::Macedonian)
    MKD,

    #[cfg(any(feature = "mongolian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Mongolian`](crate::language::Language::Mongolian)
    MON,

    #[cfg(any(feature = "maori", feature = "external-models"))]
    /// The ISO 639-3 code for [`Maori`](crate::language::Language::Maori)
    MRI,

    #[cfg(any(feature = "malay", feature = "external-models"))]
    /// The ISO 639-3 code for [`Malay`](crate::language::Language::Malay)
    MSA,

    #[cfg(any(feature = "dutch", feature = "external-models"))]
    /// The ISO 639-3 code for [`Dutch`](crate::language::Language::Dutch)
    NLD,

    #[cfg(any(feature = "nynorsk", feature = "external-models"))]
    /// The ISO 639-3 code for [`Norwegian Nynorsk`](crate::language::Language::Nynorsk)
    NNO,

    #[cfg(any(feature = "bokmal", feature = "external-models"))]
    /// The ISO 639-3 code for [`Norwegian Bokmal`](crate::language::Language::Bokmal)
    NOB,

    #[cfg(any(feature = "punjabi", feature = "external-models"))]
    /// The ISO 639-3 code for [`Punjabi`](crate::language::Language::Punjabi)
    PAN,

    #[cfg(any(feature = "polish", feature = "external-models"))]
    /// The ISO 639-3 code for [`Polish`](crate::language::Language::Polish)
    POL,

    #[cfg(any(feature = "portuguese", feature = "external-models"))]
    /// The ISO 639-3 code for [`Portuguese`](crate::language::Language::Portuguese)
    POR,

    #[cfg(any(feature = "romanian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Romanian`](crate::language::Language::Romanian)
    RON,

    #[cfg(any(feature = "russian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Russian`](crate::language::Language::Russian)
    RUS,

    #[cfg(any(feature = "slovak", feature = "external-models"))]
    /// The ISO 639-3 code for [`Slovak`](crate::language::Language::Slovak)
    SLK,

    #[cfg(any(feature = "slovene", feature = "external-models"))]
    /// The ISO 639-3 code for [`Slovene`](crate::language::Language::Slovene)
    SLV,

    #[cfg(any(feature = "shona", feature = "external-models"))]
    /// The ISO 639-3 code for [`Shona`](crate::language::Language::Shona)
    SNA,

    #[cfg(any(feature = "somali", feature = "external-models"))]
    /// The ISO 639-3 code for [`Somali`](crate::language::Language::Somali)
    SOM,

    #[cfg(any(feature = "sotho", feature = "external-models"))]
    /// The ISO 639-3 code for [`Sotho`](crate::language::Language::Sotho)
    SOT,

    #[cfg(any(feature = "spanish", feature = "external-models"))]
    /// The ISO 639-3 code for [`Spanish`](crate::language::Language::Spanish)
    SPA,

    #[cfg(any(feature = "albanian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Albanian`](crate::language::Language::Albanian)
    SQI,

    #[cfg(any(feature = "serbian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Serbian`](crate::language::Language::Serbian)
    SRP,

    #[cfg(any(feature = "swahili", feature = "external-models"))]
    /// The ISO 639-3 code for [`Swahili`](crate::language::Language::Swahili)
    SWA,

    #[cfg(any(feature = "swedish", feature = "external-models"))]
    /// The ISO 639-3 code for [`Swedish`](crate::language::Language::Swedish)
    SWE,

    #[cfg(any(feature = "tamil", feature = "external-models"))]
    /// The ISO 639-3 code for [`Tamil`](crate::language::Language::Tamil)
    TAM,

    #[cfg(any(feature = "telugu", feature = "external-models"))]
    /// The ISO 639-3 code for [`Telugu`](crate::language::Language::Telugu)
    TEL,

    #[cfg(any(feature = "tagalog", feature = "external-models"))]
    /// The ISO 639-3 code for [`Tagalog`](crate::language::Language::Tagalog)
    TGL,

    #[cfg(any(feature = "thai", feature = "external-models"))]
    /// The ISO 639-3 code for [`Thai`](crate::language::Language::Thai)
    THA,

    #[cfg(any(feature = "tswana", feature = "external-models"))]
    /// The ISO 639-3 code for [`Tswana`](crate::language::Language::Tswana)
    TSN,

    #[cfg(any(feature = "tsonga", feature = "external-models"))]
    /// The ISO 639-3 code for [`Tsonga`](crate::language::Language::Tsonga)
    TSO,

    #[cfg(any(feature = "turkish", feature = "external-models"))]
    /// The ISO 639-3 code for [`Turkish`](crate::language::Language::Turkish)
    TUR,

    #[cfg(any(feature = "ukrainian", feature = "external-models"))]
    /// The ISO 639-3 code for [`Ukrainian`](crate::language::Language::Ukrainian)
    UKR,

    #[cfg(any(feature = "urdu", feature = "external-models"))]
    /// The ISO 639-3 code for [`Urdu`](crate::language::Language::Urdu)
    URD,

    #[cfg(any(feature = "vietnamese", feature = "external-models"))]
    /// The ISO 639-3 code for [`Vietnamese`](crate::language::Language::Vietnamese)
    VIE,

    #[cfg(any(feature = "xhosa", feature = "external-models"))]
    /// The ISO 639-3 code for [`Xhosa`](crate::language::Language::Xhosa)
    XHO,

    #[cfg(any(feature = "yoruba", feature = "external-models"))]
    /// The ISO 639-3 code for [`Yoruba`](crate::language::Language::Yoruba)
    YOR,

    #[cfg(any(feature = "chinese", feature = "external-models"))]
    /// The ISO 639-3 code for [`Chinese`](crate::language::Language::Chinese)
    ZHO,

    #[cfg(any(feature = "zulu", feature = "external-models"))]
    /// The ISO 639-3 code for [`Zulu`](crate::language::Language::Zulu)
    ZUL,
}

impl Display for IsoCode639_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

impl Display for IsoCode639_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use strum::ParseError::VariantNotFound;

    #[test]
    fn assert_iso_code_639_1_string_representation_is_correct() {
        assert_eq!(IsoCode639_1::EN.to_string(), "en");
    }

    #[test]
    fn assert_iso_code_639_3_string_representation_is_correct() {
        assert_eq!(IsoCode639_3::ENG.to_string(), "eng");
    }

    #[test]
    fn assert_string_to_iso_code_639_1_is_correct() {
        assert_eq!(IsoCode639_1::from_str("en"), Ok(IsoCode639_1::EN));
        assert_eq!(IsoCode639_1::from_str("12"), Err(VariantNotFound));
    }

    #[test]
    fn assert_string_to_iso_code_639_3_is_correct() {
        assert_eq!(IsoCode639_3::from_str("eng"), Ok(IsoCode639_3::ENG));
        assert_eq!(IsoCode639_3::from_str("123"), Err(VariantNotFound));
    }
}
