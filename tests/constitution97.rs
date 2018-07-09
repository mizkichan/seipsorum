extern crate seipsorum;
use seipsorum::clause::{AttributiveClause, NominalClause, NominalClauseWithCase};
use seipsorum::meaning_bear_sound::{
    ActionVerb, Adverb, Attribute, CaseSuffix, ConverbSuffixForActionVerb,
    ConverbalCopulativeSuffix, DerivationalVerbalSuffix, FiniteSuffixForActionVerb,
    NounQualitative, NounSubstantive, ThematicPostposition,
};

/*

*/

#[test]
fn constitution97() {
    let kono = Attribute::new("kono");
    let kenpoo = NounSubstantive::new("kenpoo");
    let kk = NominalClause::new(AttributiveClause::from(kono), kenpoo);

    let ga = CaseSuffix::new("ga");
    let kkg = NominalClauseWithCase::new(kk, ga);

    let nihonkokumin = NounSubstantive::new("nihonkokumin");
    let ni = CaseSuffix::new("ni");
    let nn = NominalClauseWithCase::new(nihonkokumin, ni);

    let hosyoosu = NounSubstantive::new("hosyoosu");
    let ru = FiniteSuffixForActionVerb::NonPerfective;
    let kihontekizinken = NounSubstantive::new("kihontekizinken");
    let wa = ThematicPostposition::new("wa");
    let zinrui = NounSubstantive::new("zinrui");
    let no = CaseSuffix::new("no");
    let tanen = NounSubstantive::new("tanen");
    let wata = ActionVerb::new("wata");
    let ziyuukakutoku = NounSubstantive::new("ziyuukakutoku");
    let doryoku = NounSubstantive::new("doryoku");
    let seeka = NounSubstantive::new("seeka");
    let de = ConverbalCopulativeSuffix;
    let ar = ActionVerb::new("ar");
    let ite = ConverbSuffixForActionVerb::Perfective;
    let korera = Attribute::new("korera");
    let kenri = NounSubstantive::new("kenri");
    let kako = NounSubstantive::new("kako");
    let ikuta = NounSubstantive::new("ikuta");
    let siren = NounSubstantive::new("siren");
    let tae = ActionVerb::new("tae");
    let genzai = NounSubstantive::new("genzai");
    let oyobi = Adverb::new("oyobi");
    let syoorai = NounSubstantive::new("syoorai");
    let kokumin = NounSubstantive::new("kokumin");
    let tais = ActionVerb::new("tais");
    let i = ConverbSuffixForActionVerb::Copulative;
    let okas = ActionVerb::new("okas");
    let koto = NounSubstantive::new("koto");
    let deki = ActionVerb::new("deki");
    let ana = DerivationalVerbalSuffix::new('a', "na");
    let eekyuu = NounQualitative::new("eekyuu");
    let to = CaseSuffix::new("to");
    let sintaku = NounSubstantive::new("sintaku");
    let are = DerivationalVerbalSuffix::new('a', "re");
    let ita = FiniteSuffixForActionVerb::Perfective;
    let mono = NounSubstantive::new("mono");

    // assert_eq!(unimplemented!().into(), "この憲法が日本国民に保障する基本的人権は人類の多年にわたる自由獲得の努力の成果であってこれらの権利は過去幾多の試錬に堪え現在及び将来の国民に対し侵すことのできない永久の権利として信託されたものである");
}
