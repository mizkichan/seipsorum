use std::fmt;
use std::fmt::{Display, Formatter};
use verb::suffix::DerivationalVerbalSuffix;

/// 動詞幹
#[derive(Clone)]
pub enum VerbStem {
    Primary(VerbPrimaryStem),
    Secondary(Box<VerbSecondaryStem>),
}

impl VerbStem {
    pub fn is_consonant_stem(&self) -> bool {
        match self {
            VerbStem::Primary(primary) => primary.is_consonant_stem(),
            VerbStem::Secondary(secondary) => secondary.is_consonant_stem(),
        }
    }

    pub fn is_vowel_stem(&self) -> bool {
        match self {
            VerbStem::Primary(primary) => primary.is_vowel_stem(),
            VerbStem::Secondary(secondary) => secondary.is_vowel_stem(),
        }
    }
}

impl Display for VerbStem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            VerbStem::Primary(primary) => primary.fmt(f),
            VerbStem::Secondary(secondary) => secondary.fmt(f),
        }
    }
}

impl From<VerbPrimaryStem> for VerbStem {
    fn from(primary_stem: VerbPrimaryStem) -> VerbStem {
        VerbStem::Primary(primary_stem)
    }
}

impl From<VerbSecondaryStem> for VerbStem {
    fn from(secondary_stem: VerbSecondaryStem) -> VerbStem {
        VerbStem::Secondary(box secondary_stem)
    }
}

/// 動詞の一次語幹
#[derive(Clone)]
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
#[derive(Clone)]
pub struct VerbSecondaryStem {
    /// 一次語幹
    stem: VerbStem,

    /// 派生接尾辞
    derivational_suffix: DerivationalVerbalSuffix,
}

impl VerbSecondaryStem {
    pub fn new<IntoStem, IntoSuffix>(
        stem: IntoStem,
        derivational_suffix: IntoSuffix,
    ) -> VerbSecondaryStem
    where
        IntoStem: Into<VerbStem>,
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
            if self.derivational_suffix.has_juncture_consonant() {
                self.derivational_suffix.without_juncture()
            } else if self.derivational_suffix.has_juncture_vowel() {
                self.derivational_suffix.with_juncture()
            } else {
                unreachable!();
            }
        } else if self.stem.is_vowel_stem() {
            if self.derivational_suffix.has_juncture_consonant() {
                self.derivational_suffix.with_juncture()
            } else if self.derivational_suffix.has_juncture_vowel() {
                self.derivational_suffix.without_juncture()
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        };

        write!(f, "{}{}", self.stem, suffix)
    }
}
