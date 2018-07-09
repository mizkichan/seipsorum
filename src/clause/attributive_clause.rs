//! 連体節

use clause::NominalClause;
use meaning_bear_sound::{
    Attribute, AttributiveCopulativeSuffix, FiniteSuffixForActionVerb,
    FiniteSuffixForQualicativeVerb,
};
use word;

/// 連体節を表す列挙型．
/// 「この」「私の」「買った」「赤い」「綺麗な」など．
pub enum AttributiveClause {
    /// 連体詞
    Attribute(Attribute),

    /// 動作動詞の連体形
    AttributiveActionVerb(AttributiveActionVerb),

    /// 形状動詞の連体形
    AttributiveQualitativeVerb(AttributiveQualitativeVerb),

    /// 名詞の連体形
    AttributiveNoun(AttributiveNoun),
}

impl From<Attribute> for AttributiveClause {
    fn from(value: Attribute) -> AttributiveClause {
        AttributiveClause::Attribute(value)
    }
}

/// 動作動詞の連体形を表す構造体．
// TODO: 既に似たようなものを定義しているのでマージすること．
pub struct AttributiveActionVerb {
    /// 動作動詞幹
    pub action_verb_stem: word::action_verb::ActionVerbStem,

    /// 動作動詞終止形・連体形形成の文法接尾辞
    pub grammatical_verbal_suffix: FiniteSuffixForActionVerb,
}

/// 形状動詞の連体形を表す構造体．
pub struct AttributiveQualitativeVerb {
    /// 形状動詞幹
    pub qualitative_verb_stem: word::qualitative_verb::QualitativeVerbStem,

    /// 形状動詞終止形・連体形形成の文法接尾辞
    pub grammatical_verbal_suffix: FiniteSuffixForQualicativeVerb,
}

/// 名詞の連体形を表す構造体．
pub struct AttributiveNoun {
    /// 名詞節
    pub nominal_clause: Box<NominalClause>,

    /// 繋辞連体形
    pub copulative_suffix: AttributiveCopulativeSuffix,
}
