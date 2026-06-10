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

use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;

use crate::alphabet::CharSet;
use crate::language::Language;

pub(crate) static JAPANESE_CHARACTER_SET: LazyLock<CharSet> =
    LazyLock::new(|| CharSet::from_char_classes(&["Hiragana", "Katakana", "Han"]));
pub(crate) static MULTIPLE_WHITESPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("\\s+").unwrap());
pub(crate) static NUMBERS: LazyLock<Regex> = LazyLock::new(|| Regex::new("\\p{N}").unwrap());
pub(crate) static DIGITS_AT_BEGINNING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\\d+\\s+").unwrap());
pub(crate) static PUNCTUATION: LazyLock<Regex> = LazyLock::new(|| Regex::new("\\p{P}").unwrap());
pub(crate) static TOKENS_WITHOUT_WHITESPACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        "\\p{Bengali}+|\\p{Devanagari}+|\\p{Gujarati}+|\\p{Gurmukhi}+|\\p{Han}|\\p{Hangul}+|\\p{Hiragana}|\\p{Katakana}|\\p{Tamil}+|\\p{Telugu}+|\\p{Thai}+|\\p{L}+",
    )
    .unwrap()
});
pub(crate) static TOKENS_WITH_OPTIONAL_WHITESPACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        "\\s*(?:\\p{Bengali}+|\\p{Devanagari}+|\\p{Gujarati}+|\\p{Gurmukhi}+|\\p{Han}|\\p{Hangul}+|\\p{Hiragana}|\\p{Katakana}|\\p{Tamil}+|\\p{Telugu}+|\\p{Thai}+|[\\p{L}'-]+)[\\p{N}\\p{P}]*\\s*",
    )
    .unwrap()
});
pub(crate) static CHARS_TO_LANGUAGES_MAPPING: LazyLock<HashMap<&'static str, HashSet<Language>>> =
    LazyLock::new(|| {
        let mut mapping = hashmap!();

        if cfg!(any(feature = "portuguese", feature = "external-models")) || cfg!(any(feature = "vietnamese", feature = "external-models")) {
            mapping.insert("Ãã", {
                let mut languages = hashset!();
                if cfg!(any(feature = "portuguese", feature = "external-models")) {
                    languages.insert(Language::from_str("Portuguese").unwrap());
                }
                if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "lithuanian", feature = "external-models")) || cfg!(any(feature = "polish", feature = "external-models")) {
            mapping.insert("ĄąĘę", {
                let mut languages = hashset!();
                if cfg!(any(feature = "lithuanian", feature = "external-models")) {
                    languages.insert(Language::from_str("Lithuanian").unwrap());
                }
                if cfg!(any(feature = "polish", feature = "external-models")) {
                    languages.insert(Language::from_str("Polish").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "polish", feature = "external-models")) || cfg!(any(feature = "romanian", feature = "external-models")) {
            mapping.insert("Żż", {
                let mut languages = hashset!();
                if cfg!(any(feature = "polish", feature = "external-models")) {
                    languages.insert(Language::from_str("Polish").unwrap());
                }
                if cfg!(any(feature = "romanian", feature = "external-models")) {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "french", feature = "external-models")) || cfg!(any(feature = "luxembourgish", feature = "external-models")) || cfg!(any(feature = "romanian", feature = "external-models"))
        {
            mapping.insert("Îî", {
                let mut languages = hashset!();
                if cfg!(any(feature = "french", feature = "external-models")) {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                    languages.insert(Language::from_str("Luxembourgish").unwrap());
                }
                if cfg!(any(feature = "romanian", feature = "external-models")) {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "basque", feature = "external-models")) || cfg!(any(feature = "spanish", feature = "external-models")) {
            mapping.insert("Ññ", {
                let mut languages = hashset!();
                if cfg!(any(feature = "basque", feature = "external-models")) {
                    languages.insert(Language::from_str("Basque").unwrap());
                }
                if cfg!(any(feature = "spanish", feature = "external-models")) {
                    languages.insert(Language::from_str("Spanish").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "czech", feature = "external-models")) || cfg!(any(feature = "slovak", feature = "external-models")) {
            mapping.insert("ŇňŤť", {
                let mut languages = hashset!();
                if cfg!(any(feature = "czech", feature = "external-models")) {
                    languages.insert(Language::from_str("Czech").unwrap());
                }
                if cfg!(any(feature = "slovak", feature = "external-models")) {
                    languages.insert(Language::from_str("Slovak").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "romanian", feature = "external-models")) || cfg!(any(feature = "vietnamese", feature = "external-models")) {
            mapping.insert("Ăă", {
                let mut languages = hashset!();
                if cfg!(any(feature = "romanian", feature = "external-models")) {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "azerbaijani", feature = "external-models")) || cfg!(any(feature = "turkish", feature = "external-models")) {
            mapping.insert("İıĞğ", {
                let mut languages = hashset!();
                if cfg!(any(feature = "azerbaijani", feature = "external-models")) {
                    languages.insert(Language::from_str("Azerbaijani").unwrap());
                }
                if cfg!(any(feature = "turkish", feature = "external-models")) {
                    languages.insert(Language::from_str("Turkish").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "macedonian", feature = "external-models")) || cfg!(any(feature = "serbian", feature = "external-models")) {
            mapping.insert("ЈјЉљЊњ", {
                let mut languages = hashset!();
                if cfg!(any(feature = "macedonian", feature = "external-models")) {
                    languages.insert(Language::from_str("Macedonian").unwrap());
                }
                if cfg!(any(feature = "serbian", feature = "external-models")) {
                    languages.insert(Language::from_str("Serbian").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "vietnamese", feature = "external-models")) || cfg!(any(feature = "yoruba", feature = "external-models")) {
            mapping.insert("ẸẹỌọ", {
                let mut languages = hashset!();
                if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                if cfg!(any(feature = "yoruba", feature = "external-models")) {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "icelandic", feature = "external-models")) || cfg!(any(feature = "turkish", feature = "external-models")) {
            mapping.insert("ÐðÞþ", {
                let mut languages = hashset!();
                if cfg!(any(feature = "icelandic", feature = "external-models")) {
                    languages.insert(Language::from_str("Icelandic").unwrap());
                }
                if cfg!(any(feature = "turkish", feature = "external-models")) {
                    languages.insert(Language::from_str("Turkish").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "french", feature = "external-models"))
            || cfg!(any(feature = "hungarian", feature = "external-models"))
            || cfg!(any(feature = "luxembourgish", feature = "external-models"))
        {
            mapping.insert("Ûû", {
                let mut languages = hashset!();
                if cfg!(any(feature = "french", feature = "external-models")) {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(any(feature = "hungarian", feature = "external-models")) {
                    languages.insert(Language::from_str("Hungarian").unwrap());
                }
                if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                    languages.insert(Language::from_str("Luxembourgish").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "maori", feature = "external-models")) || cfg!(any(feature = "yoruba", feature = "external-models")) {
            mapping.insert("Ōō", {
                let mut languages = hashset!();
                if cfg!(any(feature = "maori", feature = "external-models")) {
                    languages.insert(Language::from_str("Maori").unwrap());
                }
                if cfg!(any(feature = "yoruba", feature = "external-models")) {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "kazakh", feature = "external-models")) || cfg!(any(feature = "mongolian", feature = "external-models")) {
            mapping.insert("ӨөҮү", {
                let mut languages = hashset!();
                if cfg!(any(feature = "kazakh", feature = "external-models")) {
                    languages.insert(Language::from_str("Kazakh").unwrap());
                }
                if cfg!(any(feature = "mongolian", feature = "external-models")) {
                    languages.insert(Language::from_str("Mongolian").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "latvian", feature = "external-models")) || cfg!(any(feature = "maori", feature = "external-models")) || cfg!(any(feature = "yoruba", feature = "external-models")) {
            mapping.insert("ĀāĒēĪī", {
                let mut languages = hashset!();
                if cfg!(any(feature = "latvian", feature = "external-models")) {
                    languages.insert(Language::from_str("Latvian").unwrap());
                }
                if cfg!(any(feature = "maori", feature = "external-models")) {
                    languages.insert(Language::from_str("Maori").unwrap());
                }
                if cfg!(any(feature = "yoruba", feature = "external-models")) {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "azerbaijani", feature = "external-models")) || cfg!(any(feature = "romanian", feature = "external-models")) || cfg!(any(feature = "turkish", feature = "external-models"))
        {
            mapping.insert("Şş", {
                let mut languages = hashset!();
                if cfg!(any(feature = "azerbaijani", feature = "external-models")) {
                    languages.insert(Language::from_str("Azerbaijani").unwrap());
                }
                if cfg!(any(feature = "romanian", feature = "external-models")) {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                if cfg!(any(feature = "turkish", feature = "external-models")) {
                    languages.insert(Language::from_str("Turkish").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "czech", feature = "external-models")) || cfg!(any(feature = "romanian", feature = "external-models")) || cfg!(any(feature = "slovak", feature = "external-models")) {
            mapping.insert("Ďď", {
                let mut languages = hashset!();
                if cfg!(any(feature = "czech", feature = "external-models")) {
                    languages.insert(Language::from_str("Czech").unwrap());
                }
                if cfg!(any(feature = "romanian", feature = "external-models")) {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                if cfg!(any(feature = "slovak", feature = "external-models")) {
                    languages.insert(Language::from_str("Slovak").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "bosnian", feature = "external-models")) || cfg!(any(feature = "croatian", feature = "external-models")) || cfg!(any(feature = "polish", feature = "external-models")) {
            mapping.insert("Ćć", {
                let mut languages = hashset!();
                if cfg!(any(feature = "bosnian", feature = "external-models")) {
                    languages.insert(Language::from_str("Bosnian").unwrap());
                }
                if cfg!(any(feature = "croatian", feature = "external-models")) {
                    languages.insert(Language::from_str("Croatian").unwrap());
                }
                if cfg!(any(feature = "polish", feature = "external-models")) {
                    languages.insert(Language::from_str("Polish").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "bosnian", feature = "external-models")) || cfg!(any(feature = "croatian", feature = "external-models")) || cfg!(any(feature = "vietnamese", feature = "external-models")) {
            mapping.insert("Đđ", {
                let mut languages = hashset!();
                if cfg!(any(feature = "bosnian", feature = "external-models")) {
                    languages.insert(Language::from_str("Bosnian").unwrap());
                }
                if cfg!(any(feature = "croatian", feature = "external-models")) {
                    languages.insert(Language::from_str("Croatian").unwrap());
                }
                if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "belarusian", feature = "external-models")) || cfg!(any(feature = "kazakh", feature = "external-models")) || cfg!(any(feature = "ukrainian", feature = "external-models")) {
            mapping.insert("Іі", {
                let mut languages = hashset!();
                if cfg!(any(feature = "belarusian", feature = "external-models")) {
                    languages.insert(Language::from_str("Belarusian").unwrap());
                }
                if cfg!(any(feature = "kazakh", feature = "external-models")) {
                    languages.insert(Language::from_str("Kazakh").unwrap());
                }
                if cfg!(any(feature = "ukrainian", feature = "external-models")) {
                    languages.insert(Language::from_str("Ukrainian").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "italian", feature = "external-models")) || cfg!(any(feature = "vietnamese", feature = "external-models")) || cfg!(any(feature = "yoruba", feature = "external-models")) {
            mapping.insert("Ìì", {
                let mut languages = hashset!();
                if cfg!(any(feature = "italian", feature = "external-models")) {
                    languages.insert(Language::from_str("Italian").unwrap());
                }
                if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                if cfg!(any(feature = "yoruba", feature = "external-models")) {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "bokmal", feature = "external-models")) || cfg!(any(feature = "danish", feature = "external-models")) || cfg!(any(feature = "nynorsk", feature = "external-models")) {
            mapping.insert("Øø", {
                let mut languages = hashset!();
                if cfg!(any(feature = "bokmal", feature = "external-models")) {
                    languages.insert(Language::from_str("Bokmal").unwrap());
                }
                if cfg!(any(feature = "danish", feature = "external-models")) {
                    languages.insert(Language::from_str("Danish").unwrap());
                }
                if cfg!(any(feature = "nynorsk", feature = "external-models")) {
                    languages.insert(Language::from_str("Nynorsk").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "latvian", feature = "external-models"))
            || cfg!(any(feature = "lithuanian", feature = "external-models"))
            || cfg!(any(feature = "maori", feature = "external-models"))
            || cfg!(any(feature = "yoruba", feature = "external-models"))
        {
            mapping.insert("Ūū", {
                let mut languages = hashset!();
                if cfg!(any(feature = "latvian", feature = "external-models")) {
                    languages.insert(Language::from_str("Latvian").unwrap());
                }
                if cfg!(any(feature = "lithuanian", feature = "external-models")) {
                    languages.insert(Language::from_str("Lithuanian").unwrap());
                }
                if cfg!(any(feature = "maori", feature = "external-models")) {
                    languages.insert(Language::from_str("Maori").unwrap());
                }
                if cfg!(any(feature = "yoruba", feature = "external-models")) {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "afrikaans", feature = "external-models"))
            || cfg!(any(feature = "albanian", feature = "external-models"))
            || cfg!(any(feature = "dutch", feature = "external-models"))
            || cfg!(any(feature = "french", feature = "external-models"))
            || cfg!(any(feature = "luxembourgish", feature = "external-models"))
        {
            mapping.insert("Ëë", {
                let mut languages = hashset!();
                if cfg!(any(feature = "afrikaans", feature = "external-models")) {
                    languages.insert(Language::from_str("Afrikaans").unwrap());
                }
                if cfg!(any(feature = "albanian", feature = "external-models")) {
                    languages.insert(Language::from_str("Albanian").unwrap());
                }
                if cfg!(any(feature = "dutch", feature = "external-models")) {
                    languages.insert(Language::from_str("Dutch").unwrap());
                }
                if cfg!(any(feature = "french", feature = "external-models")) {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                    languages.insert(Language::from_str("Luxembourgish").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "french", feature = "external-models"))
            || cfg!(any(feature = "italian", feature = "external-models"))
            || cfg!(any(feature = "vietnamese", feature = "external-models"))
            || cfg!(any(feature = "yoruba", feature = "external-models"))
        {
            mapping.insert("ÈèÙù", {
                let mut languages = hashset!();
                if cfg!(any(feature = "french", feature = "external-models")) {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(any(feature = "italian", feature = "external-models")) {
                    languages.insert(Language::from_str("Italian").unwrap());
                }
                if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                if cfg!(any(feature = "yoruba", feature = "external-models")) {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "afrikaans", feature = "external-models"))
            || cfg!(any(feature = "french", feature = "external-models"))
            || cfg!(any(feature = "luxembourgish", feature = "external-models"))
            || cfg!(any(feature = "portuguese", feature = "external-models"))
            || cfg!(any(feature = "vietnamese", feature = "external-models"))
        {
            mapping.insert("Êê", {
                let mut languages = hashset!();
                if cfg!(any(feature = "afrikaans", feature = "external-models")) {
                    languages.insert(Language::from_str("Afrikaans").unwrap());
                }
                if cfg!(any(feature = "french", feature = "external-models")) {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                    languages.insert(Language::from_str("Luxembourgish").unwrap());
                }
                if cfg!(any(feature = "portuguese", feature = "external-models")) {
                    languages.insert(Language::from_str("Portuguese").unwrap());
                }
                if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });
        }

        if cfg!(any(feature = "estonian", feature = "external-models"))
            || cfg!(any(feature = "hungarian", feature = "external-models"))
            || cfg!(any(feature = "portuguese", feature = "external-models"))
            || cfg!(any(feature = "vietnamese", feature = "external-models"))
        {
            mapping.insert("Õõ", {
                let mut languages = hashset!();
                if cfg!(any(feature = "estonian", feature = "external-models")) {
                    languages.insert(Language::from_str("Estonian").unwrap());
                }
                if cfg!(any(feature = "hungarian", feature = "external-models")) {
                    languages.insert(Language::from_str("Hungarian").unwrap());
                }
                if cfg!(any(feature = "portuguese", feature = "external-models")) {
                    languages.insert(Language::from_str("Portuguese").unwrap());
                }
                if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });

            if cfg!(any(feature = "french", feature = "external-models"))
                || cfg!(any(feature = "luxembourgish", feature = "external-models"))
                || cfg!(any(feature = "portuguese", feature = "external-models"))
                || cfg!(any(feature = "slovak", feature = "external-models"))
                || cfg!(any(feature = "vietnamese", feature = "external-models"))
            {
                mapping.insert("Ôô", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "french", feature = "external-models")) {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                        languages.insert(Language::from_str("Luxembourgish").unwrap());
                    }
                    if cfg!(any(feature = "portuguese", feature = "external-models")) {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(any(feature = "slovak", feature = "external-models")) {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "belarusian", feature = "external-models"))
                || cfg!(any(feature = "kazakh", feature = "external-models"))
                || cfg!(any(feature = "mongolian", feature = "external-models"))
                || cfg!(any(feature = "russian", feature = "external-models"))
            {
                mapping.insert("ЁёЫыЭэ", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "belarusian", feature = "external-models")) {
                        languages.insert(Language::from_str("Belarusian").unwrap());
                    }
                    if cfg!(any(feature = "kazakh", feature = "external-models")) {
                        languages.insert(Language::from_str("Kazakh").unwrap());
                    }
                    if cfg!(any(feature = "mongolian", feature = "external-models")) {
                        languages.insert(Language::from_str("Mongolian").unwrap());
                    }
                    if cfg!(any(feature = "russian", feature = "external-models")) {
                        languages.insert(Language::from_str("Russian").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "bulgarian", feature = "external-models"))
                || cfg!(any(feature = "kazakh", feature = "external-models"))
                || cfg!(any(feature = "mongolian", feature = "external-models"))
                || cfg!(any(feature = "russian", feature = "external-models"))
            {
                mapping.insert("ЩщЪъ", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "bulgarian", feature = "external-models")) {
                        languages.insert(Language::from_str("Bulgarian").unwrap());
                    }
                    if cfg!(any(feature = "kazakh", feature = "external-models")) {
                        languages.insert(Language::from_str("Kazakh").unwrap());
                    }
                    if cfg!(any(feature = "mongolian", feature = "external-models")) {
                        languages.insert(Language::from_str("Mongolian").unwrap());
                    }
                    if cfg!(any(feature = "russian", feature = "external-models")) {
                        languages.insert(Language::from_str("Russian").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "catalan", feature = "external-models"))
                || cfg!(any(feature = "italian", feature = "external-models"))
                || cfg!(any(feature = "vietnamese", feature = "external-models"))
                || cfg!(any(feature = "yoruba", feature = "external-models"))
            {
                mapping.insert("Òò", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "catalan", feature = "external-models")) {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(any(feature = "italian", feature = "external-models")) {
                        languages.insert(Language::from_str("Italian").unwrap());
                    }
                    if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    if cfg!(any(feature = "yoruba", feature = "external-models")) {
                        languages.insert(Language::from_str("Yoruba").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "french", feature = "external-models"))
                || cfg!(any(feature = "luxembourgish", feature = "external-models"))
                || cfg!(any(feature = "portuguese", feature = "external-models"))
                || cfg!(any(feature = "romanian", feature = "external-models"))
                || cfg!(any(feature = "turkish", feature = "external-models"))
                || cfg!(any(feature = "vietnamese", feature = "external-models"))
            {
                mapping.insert("Ââ", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "french", feature = "external-models")) {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                        languages.insert(Language::from_str("Luxembourgish").unwrap());
                    }
                    if cfg!(any(feature = "portuguese", feature = "external-models")) {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(any(feature = "romanian", feature = "external-models")) {
                        languages.insert(Language::from_str("Romanian").unwrap());
                    }
                    if cfg!(any(feature = "turkish", feature = "external-models")) {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "bokmal", feature = "external-models"))
                || cfg!(any(feature = "danish", feature = "external-models"))
                || cfg!(any(feature = "icelandic", feature = "external-models"))
                || cfg!(any(feature = "nynorsk", feature = "external-models"))
            {
                mapping.insert("Ææ", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "bokmal", feature = "external-models")) {
                        languages.insert(Language::from_str("Bokmal").unwrap());
                    }
                    if cfg!(any(feature = "danish", feature = "external-models")) {
                        languages.insert(Language::from_str("Danish").unwrap());
                    }
                    if cfg!(any(feature = "icelandic", feature = "external-models")) {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(any(feature = "nynorsk", feature = "external-models")) {
                        languages.insert(Language::from_str("Nynorsk").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "bokmal", feature = "external-models"))
                || cfg!(any(feature = "danish", feature = "external-models"))
                || cfg!(any(feature = "nynorsk", feature = "external-models"))
                || cfg!(any(feature = "swedish", feature = "external-models"))
            {
                mapping.insert("Åå", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "bokmal", feature = "external-models")) {
                        languages.insert(Language::from_str("Bokmal").unwrap());
                    }
                    if cfg!(any(feature = "danish", feature = "external-models")) {
                        languages.insert(Language::from_str("Danish").unwrap());
                    }
                    if cfg!(any(feature = "nynorsk", feature = "external-models")) {
                        languages.insert(Language::from_str("Nynorsk").unwrap());
                    }
                    if cfg!(any(feature = "swedish", feature = "external-models")) {
                        languages.insert(Language::from_str("Swedish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "czech", feature = "external-models"))
                || cfg!(any(feature = "icelandic", feature = "external-models"))
                || cfg!(any(feature = "slovak", feature = "external-models"))
                || cfg!(any(feature = "turkish", feature = "external-models"))
                || cfg!(any(feature = "vietnamese", feature = "external-models"))
            {
                mapping.insert("Ýý", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "czech", feature = "external-models")) {
                        languages.insert(Language::from_str("Czech").unwrap());
                    }
                    if cfg!(any(feature = "icelandic", feature = "external-models")) {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(any(feature = "slovak", feature = "external-models")) {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(any(feature = "turkish", feature = "external-models")) {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "estonian", feature = "external-models"))
                || cfg!(any(feature = "finnish", feature = "external-models"))
                || cfg!(any(feature = "german", feature = "external-models"))
                || cfg!(any(feature = "luxembourgish", feature = "external-models"))
                || cfg!(any(feature = "slovak", feature = "external-models"))
                || cfg!(any(feature = "swedish", feature = "external-models"))
            {
                mapping.insert("Ää", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "estonian", feature = "external-models")) {
                        languages.insert(Language::from_str("Estonian").unwrap());
                    }
                    if cfg!(any(feature = "finnish", feature = "external-models")) {
                        languages.insert(Language::from_str("Finnish").unwrap());
                    }
                    if cfg!(any(feature = "german", feature = "external-models")) {
                        languages.insert(Language::from_str("German").unwrap());
                    }
                    if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                        languages.insert(Language::from_str("Luxembourgish").unwrap());
                    }
                    if cfg!(any(feature = "slovak", feature = "external-models")) {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(any(feature = "swedish", feature = "external-models")) {
                        languages.insert(Language::from_str("Swedish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "catalan", feature = "external-models"))
                || cfg!(any(feature = "french", feature = "external-models"))
                || cfg!(any(feature = "italian", feature = "external-models"))
                || cfg!(any(feature = "portuguese", feature = "external-models"))
                || cfg!(any(feature = "vietnamese", feature = "external-models"))
            {
                mapping.insert("Àà", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "catalan", feature = "external-models")) {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(any(feature = "french", feature = "external-models")) {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(any(feature = "italian", feature = "external-models")) {
                        languages.insert(Language::from_str("Italian").unwrap());
                    }
                    if cfg!(any(feature = "portuguese", feature = "external-models")) {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "azerbaijani", feature = "external-models"))
                || cfg!(any(feature = "catalan", feature = "external-models"))
                || cfg!(any(feature = "estonian", feature = "external-models"))
                || cfg!(any(feature = "german", feature = "external-models"))
                || cfg!(any(feature = "hungarian", feature = "external-models"))
                || cfg!(any(feature = "luxembourgish", feature = "external-models"))
                || cfg!(any(feature = "spanish", feature = "external-models"))
                || cfg!(any(feature = "turkish", feature = "external-models"))
            {
                mapping.insert("Üü", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "azerbaijani", feature = "external-models")) {
                        languages.insert(Language::from_str("Azerbaijani").unwrap());
                    }
                    if cfg!(any(feature = "catalan", feature = "external-models")) {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(any(feature = "estonian", feature = "external-models")) {
                        languages.insert(Language::from_str("Estonian").unwrap());
                    }
                    if cfg!(any(feature = "german", feature = "external-models")) {
                        languages.insert(Language::from_str("German").unwrap());
                    }
                    if cfg!(any(feature = "hungarian", feature = "external-models")) {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                        languages.insert(Language::from_str("Luxembourgish").unwrap());
                    }
                    if cfg!(any(feature = "spanish", feature = "external-models")) {
                        languages.insert(Language::from_str("Spanish").unwrap());
                    }
                    if cfg!(any(feature = "turkish", feature = "external-models")) {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "bosnian", feature = "external-models"))
                || cfg!(any(feature = "czech", feature = "external-models"))
                || cfg!(any(feature = "croatian", feature = "external-models"))
                || cfg!(any(feature = "latvian", feature = "external-models"))
                || cfg!(any(feature = "lithuanian", feature = "external-models"))
                || cfg!(any(feature = "slovak", feature = "external-models"))
                || cfg!(any(feature = "slovene", feature = "external-models"))
            {
                mapping.insert("ČčŠšŽž", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "bosnian", feature = "external-models")) {
                        languages.insert(Language::from_str("Bosnian").unwrap());
                    }
                    if cfg!(any(feature = "czech", feature = "external-models")) {
                        languages.insert(Language::from_str("Czech").unwrap());
                    }
                    if cfg!(any(feature = "croatian", feature = "external-models")) {
                        languages.insert(Language::from_str("Croatian").unwrap());
                    }
                    if cfg!(any(feature = "latvian", feature = "external-models")) {
                        languages.insert(Language::from_str("Latvian").unwrap());
                    }
                    if cfg!(any(feature = "lithuanian", feature = "external-models")) {
                        languages.insert(Language::from_str("Lithuanian").unwrap());
                    }
                    if cfg!(any(feature = "slovak", feature = "external-models")) {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(any(feature = "slovene", feature = "external-models")) {
                        languages.insert(Language::from_str("Slovene").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "albanian", feature = "external-models"))
                || cfg!(any(feature = "azerbaijani", feature = "external-models"))
                || cfg!(any(feature = "basque", feature = "external-models"))
                || cfg!(any(feature = "catalan", feature = "external-models"))
                || cfg!(any(feature = "french", feature = "external-models"))
                || cfg!(any(feature = "portuguese", feature = "external-models"))
                || cfg!(any(feature = "turkish", feature = "external-models"))
            {
                mapping.insert("Çç", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "albanian", feature = "external-models")) {
                        languages.insert(Language::from_str("Albanian").unwrap());
                    }
                    if cfg!(any(feature = "azerbaijani", feature = "external-models")) {
                        languages.insert(Language::from_str("Azerbaijani").unwrap());
                    }
                    if cfg!(any(feature = "basque", feature = "external-models")) {
                        languages.insert(Language::from_str("Basque").unwrap());
                    }
                    if cfg!(any(feature = "catalan", feature = "external-models")) {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(any(feature = "french", feature = "external-models")) {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(any(feature = "portuguese", feature = "external-models")) {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(any(feature = "turkish", feature = "external-models")) {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "azerbaijani", feature = "external-models"))
                || cfg!(any(feature = "estonian", feature = "external-models"))
                || cfg!(any(feature = "finnish", feature = "external-models"))
                || cfg!(any(feature = "german", feature = "external-models"))
                || cfg!(any(feature = "hungarian", feature = "external-models"))
                || cfg!(any(feature = "icelandic", feature = "external-models"))
                || cfg!(any(feature = "luxembourgish", feature = "external-models"))
                || cfg!(any(feature = "swedish", feature = "external-models"))
                || cfg!(any(feature = "turkish", feature = "external-models"))
            {
                mapping.insert("Öö", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "azerbaijani", feature = "external-models")) {
                        languages.insert(Language::from_str("Azerbaijani").unwrap());
                    }
                    if cfg!(any(feature = "estonian", feature = "external-models")) {
                        languages.insert(Language::from_str("Estonian").unwrap());
                    }
                    if cfg!(any(feature = "finnish", feature = "external-models")) {
                        languages.insert(Language::from_str("Finnish").unwrap());
                    }
                    if cfg!(any(feature = "german", feature = "external-models")) {
                        languages.insert(Language::from_str("German").unwrap());
                    }
                    if cfg!(any(feature = "hungarian", feature = "external-models")) {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(any(feature = "icelandic", feature = "external-models")) {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                        languages.insert(Language::from_str("Luxembourgish").unwrap());
                    }
                    if cfg!(any(feature = "swedish", feature = "external-models")) {
                        languages.insert(Language::from_str("Swedish").unwrap());
                    }
                    if cfg!(any(feature = "turkish", feature = "external-models")) {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "catalan", feature = "external-models"))
                || cfg!(any(feature = "hungarian", feature = "external-models"))
                || cfg!(any(feature = "icelandic", feature = "external-models"))
                || cfg!(any(feature = "irish", feature = "external-models"))
                || cfg!(any(feature = "polish", feature = "external-models"))
                || cfg!(any(feature = "portuguese", feature = "external-models"))
                || cfg!(any(feature = "slovak", feature = "external-models"))
                || cfg!(any(feature = "spanish", feature = "external-models"))
                || cfg!(any(feature = "vietnamese", feature = "external-models"))
                || cfg!(any(feature = "yoruba", feature = "external-models"))
            {
                mapping.insert("Óó", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "catalan", feature = "external-models")) {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(any(feature = "hungarian", feature = "external-models")) {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(any(feature = "icelandic", feature = "external-models")) {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(any(feature = "irish", feature = "external-models")) {
                        languages.insert(Language::from_str("Irish").unwrap());
                    }
                    if cfg!(any(feature = "polish", feature = "external-models")) {
                        languages.insert(Language::from_str("Polish").unwrap());
                    }
                    if cfg!(any(feature = "portuguese", feature = "external-models")) {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(any(feature = "slovak", feature = "external-models")) {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(any(feature = "spanish", feature = "external-models")) {
                        languages.insert(Language::from_str("Spanish").unwrap());
                    }
                    if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    if cfg!(any(feature = "yoruba", feature = "external-models")) {
                        languages.insert(Language::from_str("Yoruba").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "catalan", feature = "external-models"))
                || cfg!(any(feature = "czech", feature = "external-models"))
                || cfg!(any(feature = "icelandic", feature = "external-models"))
                || cfg!(any(feature = "irish", feature = "external-models"))
                || cfg!(any(feature = "hungarian", feature = "external-models"))
                || cfg!(any(feature = "portuguese", feature = "external-models"))
                || cfg!(any(feature = "slovak", feature = "external-models"))
                || cfg!(any(feature = "spanish", feature = "external-models"))
                || cfg!(any(feature = "vietnamese", feature = "external-models"))
                || cfg!(any(feature = "yoruba", feature = "external-models"))
            {
                mapping.insert("ÁáÍíÚú", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "catalan", feature = "external-models")) {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(any(feature = "czech", feature = "external-models")) {
                        languages.insert(Language::from_str("Czech").unwrap());
                    }
                    if cfg!(any(feature = "icelandic", feature = "external-models")) {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(any(feature = "irish", feature = "external-models")) {
                        languages.insert(Language::from_str("Irish").unwrap());
                    }
                    if cfg!(any(feature = "hungarian", feature = "external-models")) {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(any(feature = "portuguese", feature = "external-models")) {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(any(feature = "slovak", feature = "external-models")) {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(any(feature = "spanish", feature = "external-models")) {
                        languages.insert(Language::from_str("Spanish").unwrap());
                    }
                    if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    if cfg!(any(feature = "yoruba", feature = "external-models")) {
                        languages.insert(Language::from_str("Yoruba").unwrap());
                    }
                    languages
                });
            }

            if cfg!(any(feature = "catalan", feature = "external-models"))
                || cfg!(any(feature = "czech", feature = "external-models"))
                || cfg!(any(feature = "french", feature = "external-models"))
                || cfg!(any(feature = "hungarian", feature = "external-models"))
                || cfg!(any(feature = "icelandic", feature = "external-models"))
                || cfg!(any(feature = "irish", feature = "external-models"))
                || cfg!(any(feature = "italian", feature = "external-models"))
                || cfg!(any(feature = "luxembourgish", feature = "external-models"))
                || cfg!(any(feature = "portuguese", feature = "external-models"))
                || cfg!(any(feature = "slovak", feature = "external-models"))
                || cfg!(any(feature = "spanish", feature = "external-models"))
                || cfg!(any(feature = "vietnamese", feature = "external-models"))
                || cfg!(any(feature = "yoruba", feature = "external-models"))
            {
                mapping.insert("Éé", {
                    let mut languages = hashset!();
                    if cfg!(any(feature = "catalan", feature = "external-models")) {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(any(feature = "czech", feature = "external-models")) {
                        languages.insert(Language::from_str("Czech").unwrap());
                    }
                    if cfg!(any(feature = "french", feature = "external-models")) {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(any(feature = "hungarian", feature = "external-models")) {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(any(feature = "icelandic", feature = "external-models")) {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(any(feature = "irish", feature = "external-models")) {
                        languages.insert(Language::from_str("Irish").unwrap());
                    }
                    if cfg!(any(feature = "italian", feature = "external-models")) {
                        languages.insert(Language::from_str("Italian").unwrap());
                    }
                    if cfg!(any(feature = "luxembourgish", feature = "external-models")) {
                        languages.insert(Language::from_str("Luxembourgish").unwrap());
                    }
                    if cfg!(any(feature = "portuguese", feature = "external-models")) {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(any(feature = "slovak", feature = "external-models")) {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(any(feature = "spanish", feature = "external-models")) {
                        languages.insert(Language::from_str("Spanish").unwrap());
                    }
                    if cfg!(any(feature = "vietnamese", feature = "external-models")) {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    if cfg!(any(feature = "yoruba", feature = "external-models")) {
                        languages.insert(Language::from_str("Yoruba").unwrap());
                    }
                    languages
                });
            }
        }

        mapping
    });
