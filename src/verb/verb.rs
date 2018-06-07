use std::fmt;
use std::fmt::{Display, Formatter};
use verb::stem::VerbStem;
use verb::suffix::SyntacticalVerbalSuffix;
use Word;

/// 動詞
#[derive(Clone)]
pub struct Verb {
    /// 語幹
    stem: VerbStem,

    /// 統語接尾辞
    syntactical_suffix: SyntacticalVerbalSuffix,
}

impl Verb {
    pub fn new<IntoStem, IntoSuffix>(stem: IntoStem, suffix: IntoSuffix) -> Verb
    where
        IntoStem: Into<VerbStem>,
        IntoSuffix: Into<SyntacticalVerbalSuffix>,
    {
        Verb {
            stem: stem.into(),
            syntactical_suffix: suffix.into(),
        }
    }
}

impl Display for Verb {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // FIXME: following code is getting to be stupid!
        let suffix = if self.stem.is_consonant_stem() {
            if self.syntactical_suffix.has_juncture_vowel() {
                self.syntactical_suffix.with_juncture()
            } else {
                self.syntactical_suffix.without_juncture()
            }
        } else if self.stem.is_vowel_stem() {
            if self.syntactical_suffix.has_juncture_consonant() {
                self.syntactical_suffix.with_juncture()
            } else {
                self.syntactical_suffix.without_juncture()
            }
        } else {
            unreachable!();
        };

        write!(f, "{}{}", self.stem, suffix)
    }
}

impl Word for Verb {}
