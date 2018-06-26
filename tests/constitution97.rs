extern crate seipsorum;
use seipsorum::meaning_bear_sound::{
    ActionVerb, CaseSuffix, DerivationalVerbalSuffix, GrammaticalVerbalSuffix, NounQualitative,
    NounSubstantive, ThematicPostposition,
};

/*
この憲法が日本国民に保障する基本的人権は、人類の多年にわたる自由獲得の努力の成果であって、これらの権利は、過去幾多の試錬に堪え、現在及び将来の国民に対し、侵すことのできない永久の権利として信託されたものである。
*/

#[test]
fn constitution97() {
    // TODO "この"

    let kenpoo = NounSubstantive::new("kenpoo");
    let ga = CaseSuffix::new("ga");
    let nihonkokumin = NounSubstantive::new("nihonkokumin");
    let ni = CaseSuffix::new("ni");
    let hosyoo = NounSubstantive::new("hosyoo");
    let su = ActionVerb::new("su"); // NOTE 変格活用
    let ru = GrammaticalVerbalSuffix::new('r', "u");
    let kihontekizinken = NounSubstantive::new("kihontekizinken");
    let wa = ThematicPostposition::new("wa");
    let zinrui = NounSubstantive::new("zinrui");
    let no = CaseSuffix::new("no");
    let tanen = NounSubstantive::new("tanen");
    let wata = ActionVerb::new("wata");
    let ziyuukakutoku = NounSubstantive::new("ziyuukakutoku");
    let doryoku = NounSubstantive::new("doryoku");
    let seeka = NounSubstantive::new("seeka");

    // TODO "であって"
    // TODO "これら"

    let kenri = NounSubstantive::new("kenri");
    let kako = NounSubstantive::new("kako");
    let ikuta = NounSubstantive::new("ikuta");
    let siren = NounSubstantive::new("siren");
    let tae = ActionVerb::new("tae");
    let genzai = NounSubstantive::new("genzai");

    // TODO "及び"

    let mirai = NounSubstantive::new("mirai");
    let kokumin = NounSubstantive::new("kokumin");

    // TODO "対し"

    let okas = ActionVerb::new("okas");

    // TODO "こと"

    let deki = ActionVerb::new("deki");
    let ana = DerivationalVerbalSuffix::new('a', "na");
    let i = GrammaticalVerbalSuffix::new('i', "");
    let eekyuu = NounQualitative::new("eekyuu"); // NOTE 本当に形状名詞か？

    // TODO "として"

    let sintaku = NounSubstantive::new("sintaku");
    let are = DerivationalVerbalSuffix::new('a', "re");
    let ita = GrammaticalVerbalSuffix::new('i', "ta");

    // TODO "もの"
    // TODO "である"

    unimplemented!();
}
