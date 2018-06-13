//! 連体語
//!
//! NOTE: 「連体語」(Attributive) という用語は『新論』には登場せず未確定である．
//! `AttributiveActionVerb`, `AttributiveQualitativeVerb`, `AttributiveNoun` という識別子は未確定である．

use clause;
use meaning_bear_sound;
use word;

/// 連体語を表す列挙型．
pub enum Attributive {
    /// 連体詞
    Attributie(meaning_bear_sound::Attribute),

    /// 動作動詞の連体形
    AttributiveActionVerb(AttributiveActionVerb),

    /// 形状動詞の連体形
    AttributiveQualitativeVerb(AttributiveQualitativeVerb),

    /// 名詞の連体形
    AttributiveNoun(AttributiveNoun),
}

/// 動作動詞の連体形を表す構造体．
///
/// 主要部は動作動詞終止形・連体形形成の文法接尾辞，従属部は動作動詞幹である．
// TODO: 既に似たようなものを定義しているのでマージすること．
pub struct AttributiveActionVerb {
    /// 動作動詞終止形・連体形形成の文法接尾辞
    // NOTE: これにあたる範疇はまだ作成されていない．仮に動詞文法接尾辞を取るものとする．
    pub grammatical_verbal_suffix: meaning_bear_sound::GrammaticalVerbalSuffix,

    /// 動作動詞幹
    pub action_verb_stem: word::action_verb::ActionVerbStem,
}

/// 形状動詞の連体形を表す構造体．
///
/// 主要部は形状動詞終止形・連体形形成の文法接尾辞，従属部は形状動詞幹である．
pub struct AttributiveQualitativeVerb {
    /// 形状動詞終止形・連体形形成の文法接尾辞
    // NOTE: これにあたる範疇はまだ作成されていない．仮に動詞文法接尾辞を取るものとする．
    pub grammatical_verbal_suffix: meaning_bear_sound::GrammaticalVerbalSuffix,

    /// 形状動詞幹
    pub qualitative_verb_stem: word::qualitative_verb::QualitativeVerbStem,
}

/// 名詞の連体形を表す構造体．
///
/// 主要部は繋辞連体形，従属部は名詞節である．たぶん．
pub struct AttributiveNoun {
    /// 繋辞連体形
    // NOTE: これにあたる範疇はまだ作成されていない．仮に繋辞を取るものとする．
    pub copulative_suffix: meaning_bear_sound::CopulativeSuffix,

    /// 名詞節
    pub nominal_clause: Box<clause::NominalClause>,
}
