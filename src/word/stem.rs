#![allow(missing_docs)]
use meaning_bear_sound::DerivationalVerbalSuffix;
use std::fmt;
use std::fmt::{Display, Formatter};

/// 動詞幹
#[derive(Debug, Clone)]
pub enum ActionVerbStem {
    Primary(VerbPrimaryStem),
    Secondary(Box<VerbSecondaryStem>),
}

impl ActionVerbStem {
    pub fn is_consonant_stem(&self) -> bool {
        match self {
            ActionVerbStem::Primary(primary) => primary.is_consonant_stem(),
            ActionVerbStem::Secondary(secondary) => secondary.is_consonant_stem(),
        }
    }

    pub fn is_vowel_stem(&self) -> bool {
        match self {
            ActionVerbStem::Primary(primary) => primary.is_vowel_stem(),
            ActionVerbStem::Secondary(secondary) => secondary.is_vowel_stem(),
        }
    }
}

impl Display for ActionVerbStem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ActionVerbStem::Primary(primary) => primary.fmt(f),
            ActionVerbStem::Secondary(secondary) => secondary.fmt(f),
        }
    }
}

impl From<VerbPrimaryStem> for ActionVerbStem {
    fn from(primary_stem: VerbPrimaryStem) -> ActionVerbStem {
        ActionVerbStem::Primary(primary_stem)
    }
}

impl From<VerbSecondaryStem> for ActionVerbStem {
    fn from(secondary_stem: VerbSecondaryStem) -> ActionVerbStem {
        ActionVerbStem::Secondary(box secondary_stem)
    }
}

/// 動詞の一次語幹
#[derive(Debug, Clone)]
pub struct VerbPrimaryStem {
    /// 語幹
    stem: String,
}

impl VerbPrimaryStem {
    pub fn new<T>(stem: T) -> VerbPrimaryStem
    where
        T: Into<String>,
    {
        VerbPrimaryStem { stem: stem.into() }
    }

    pub fn is_consonant_stem(&self) -> bool {
        let last_characher = self.stem.chars().last().unwrap(); // NOTE: ひどいので `String` は止めて `Vec<u8>` にしたほうがいいかも
        assert!(match last_characher {
            'c' | 'f' | 'j' | 'l' | 'q' | 'v' | 'x' => false,
            other => other.is_ascii_lowercase(),
        });

        match last_characher {
            'k' | 'g' | 's' | 'z' | 't' | 'd' | 'n' | 'h' | 'p' | 'b' | 'm' | 'r' | 'w' => true,
            'a' | 'i' | 'u' | 'e' | 'o' => false,
            _ => unreachable!(),
        }
    }

    pub fn is_vowel_stem(&self) -> bool {
        !self.is_consonant_stem()
    }
}

impl Display for VerbPrimaryStem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.stem.fmt(f)
    }
}

/// 動詞の二次語幹
#[derive(Debug, Clone)]
pub struct VerbSecondaryStem {
    /// 一次語幹
    stem: ActionVerbStem,

    /// 派生接尾辞
    derivational_suffix: DerivationalVerbalSuffix,
}

impl VerbSecondaryStem {
    pub fn new<IntoStem, IntoSuffix>(
        stem: IntoStem,
        derivational_suffix: IntoSuffix,
    ) -> VerbSecondaryStem
    where
        IntoStem: Into<ActionVerbStem>,
        IntoSuffix: Into<DerivationalVerbalSuffix>,
    {
        VerbSecondaryStem {
            stem: stem.into(),
            derivational_suffix: derivational_suffix.into(),
        }
    }

    pub fn is_consonant_stem(&self) -> bool {
        self.derivational_suffix.is_consonant_stem()
    }

    pub fn is_vowel_stem(&self) -> bool {
        !self.is_consonant_stem()
    }
}

impl Display for VerbSecondaryStem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let suffix = if self.stem.is_consonant_stem() {
            if self.derivational_suffix.has_juncture_vowel() {
                self.derivational_suffix.with_juncture()
            } else {
                self.derivational_suffix.without_juncture()
            }
        } else if self.stem.is_vowel_stem() {
            if self.derivational_suffix.has_juncture_consonant() {
                self.derivational_suffix.with_juncture()
            } else {
                self.derivational_suffix.without_juncture()
            }
        } else {
            unreachable!()
        };

        write!(f, "{}{}", self.stem, suffix)
    }
}
