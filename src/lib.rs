#![feature(box_syntax)]
mod word;
pub use word::Word;

pub mod verb;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn verb() {
        use verb::*;

        let kak = VerbPrimaryStem::new("kak");
        let mi = VerbPrimaryStem::new("mi");
        let sase = DerivationalVerbalSuffix::new('s', "ase");
        let ana = DerivationalVerbalSuffix::new('a', "na");
        let ru = SyntacticalVerbalSuffix::new('r', "u");
        let i = SyntacticalVerbalSuffix::new(None, "i");

        {
            let kak = kak.clone();
            let ru = ru.clone();

            let kaku = Verb::new(kak, ru);
            assert_eq!("kaku", kaku.to_string());
        }

        {
            let kak = kak.clone();
            let ru = ru.clone();
            let sase = sase.clone();

            let kakase = VerbSecondaryStem::new(kak, sase);
            let kakaseru = Verb::new(kakase, ru);
            assert_eq!("kakaseru", kakaseru.to_string());
        }

        {
            let kak = kak.clone();
            let ana = ana.clone();
            let i = i.clone();

            let kakana = VerbSecondaryStem::new(kak, ana);
            let kakanai = Verb::new(kakana, i);
            assert_eq!("kakanai", kakanai.to_string());
        }

        {
            let mi = mi.clone();
            let ru = ru.clone();

            let miru = Verb::new(mi, ru);
            assert_eq!("miru", miru.to_string());
        }

        {
            let mi = mi.clone();
            let ru = ru.clone();
            let sase = sase.clone();

            let misase = VerbSecondaryStem::new(mi, sase);
            let misaseru = Verb::new(misase, ru);
            assert_eq!("misaseru", misaseru.to_string());
        }

        {
            let mi = mi.clone();
            let ana = ana.clone();
            let i = i.clone();

            let mina = VerbSecondaryStem::new(mi, ana);
            let minai = Verb::new(mina, i);
            assert_eq!("minai", minai.to_string());
        }

        {
            let kak = kak.clone();
            let sase = sase.clone();
            let rare = DerivationalVerbalSuffix::new('r', "are");
            let itagar = DerivationalVerbalSuffix::new('i', "tagar");
            let imas = DerivationalVerbalSuffix::new('i', "mas");
            let umai = SyntacticalVerbalSuffix::new('u', "mai");

            let kakase = VerbSecondaryStem::new(kak, sase);
            let kakaserare = VerbSecondaryStem::new(kakase, rare);
            let kakaseraretagar = VerbSecondaryStem::new(kakaserare, itagar);
            let kakaseraretagarimas = VerbSecondaryStem::new(kakaseraretagar, imas);
            let kakaseraretagarimasumai = Verb::new(kakaseraretagarimas, umai);
            assert_eq!(
                "kakaseraretagarimasumai",
                kakaseraretagarimasumai.to_string()
            );
        }

        {
            let kak = kak.clone();
            let ita = SyntacticalVerbalSuffix::new('i', "ta");

            let kaita = Verb::new(kak, ita);
            assert_eq!("kaita", kaita.to_string());
        }
    }
}
