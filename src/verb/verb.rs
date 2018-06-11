use std::fmt;
use std::fmt::{Display, Formatter};
use verb::stem::VerbStem;
use verb::suffix::SyntacticalVerbalSuffix;

/// 動詞
#[derive(Debug, Clone)]
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
        // 連声の処理
        fn ita(stem: &VerbStem) -> Option<String> {
            let mut result = stem.to_string();
            match result.pop().unwrap() {
                'k' => result.push_str("ita"),
                'g' => result.push_str("ida"),
                't' | 'r' | 'w' => result.push_str("tta"),
                'n' | 'm' => result.push_str("nda"),
                's' => result.push_str("sita"),
                _ => return None,
            }
            Some(result)
        }

        if self.syntactical_suffix.with_juncture() == "ita" {
            if let Some(result) = ita(&self.stem) {
                return f.write_str(&result);
            }
        }

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
            unreachable!()
        };

        write!(f, "{}{}", self.stem, suffix)
    }
}
