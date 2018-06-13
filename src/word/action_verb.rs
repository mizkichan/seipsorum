//! 動作動詞

use meaning_bear_sound::GrammaticalVerbalSuffix;
use std::fmt;
use std::fmt::{Display, Formatter};
use word::stem::ActionVerbStem;

/// 動作動詞
#[derive(Debug, Clone)]
pub struct ActionVerb {
    /// 語幹
    stem: ActionVerbStem,

    /// 統語接尾辞
    syntactical_suffix: GrammaticalVerbalSuffix,
}

impl ActionVerb {
    /// 語幹と動詞接尾辞から動作動詞を作成する．
    pub fn new<IntoStem, IntoSuffix>(stem: IntoStem, suffix: IntoSuffix) -> ActionVerb
    where
        IntoStem: Into<ActionVerbStem>,
        IntoSuffix: Into<GrammaticalVerbalSuffix>,
    {
        ActionVerb {
            stem: stem.into(),
            syntactical_suffix: suffix.into(),
        }
    }
}

impl Display for ActionVerb {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // 連声の処理
        fn ita(stem: &ActionVerbStem) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn verb() {
        use self::stem::{VerbPrimaryStem, VerbSecondaryStem};
        use super::*;
        use meaning_bear_sound::bound_sound::suffix::verbal_suffix::derivational_verbal_suffix::DerivationalVerbalSuffix;

        let kak = VerbPrimaryStem::new("kak");
        let mi = VerbPrimaryStem::new("mi");
        let sase = DerivationalVerbalSuffix::new('s', "ase");
        let ana = DerivationalVerbalSuffix::new('a', "na");
        let ru = GrammaticalVerbalSuffix::new('r', "u");
        let i = GrammaticalVerbalSuffix::new(None, "i");

        {
            let kak = kak.clone();
            let ru = ru.clone();

            let kaku = ActionVerb::new(kak, ru);
            assert_eq!("kaku", kaku.to_string());
        }

        {
            let kak = kak.clone();
            let ru = ru.clone();
            let sase = sase.clone();

            let kakase = VerbSecondaryStem::new(kak, sase);
            let kakaseru = ActionVerb::new(kakase, ru);
            assert_eq!("kakaseru", kakaseru.to_string());
        }

        {
            let kak = kak.clone();
            let ana = ana.clone();
            let i = i.clone();

            let kakana = VerbSecondaryStem::new(kak, ana);
            let kakanai = ActionVerb::new(kakana, i);
            assert_eq!("kakanai", kakanai.to_string());
        }

        {
            let mi = mi.clone();
            let ru = ru.clone();

            let miru = ActionVerb::new(mi, ru);
            assert_eq!("miru", miru.to_string());
        }

        {
            let mi = mi.clone();
            let ru = ru.clone();
            let sase = sase.clone();

            let misase = VerbSecondaryStem::new(mi, sase);
            let misaseru = ActionVerb::new(misase, ru);
            assert_eq!("misaseru", misaseru.to_string());
        }

        {
            let mi = mi.clone();
            let ana = ana.clone();
            let i = i.clone();

            let mina = VerbSecondaryStem::new(mi, ana);
            let minai = ActionVerb::new(mina, i);
            assert_eq!("minai", minai.to_string());
        }

        {
            let kak = kak.clone();
            let sase = sase.clone();
            let rare = DerivationalVerbalSuffix::new('r', "are");
            let itagar = DerivationalVerbalSuffix::new('i', "tagar");
            let imas = DerivationalVerbalSuffix::new('i', "mas");
            let umai = GrammaticalVerbalSuffix::new('u', "mai");

            let kakase = VerbSecondaryStem::new(kak, sase);
            let kakaserare = VerbSecondaryStem::new(kakase, rare);
            let kakaseraretagar = VerbSecondaryStem::new(kakaserare, itagar);
            let kakaseraretagarimas = VerbSecondaryStem::new(kakaseraretagar, imas);
            let kakaseraretagarimasumai = ActionVerb::new(kakaseraretagarimas, umai);
            assert_eq!(
                "kakaseraretagarimasumai",
                kakaseraretagarimasumai.to_string()
            );
        }

        {
            let kak = kak.clone();
            let ita = GrammaticalVerbalSuffix::new('i', "ta");

            let kaita = ActionVerb::new(kak, ita);
            assert_eq!("kaita", kaita.to_string());
        }
    }
}
