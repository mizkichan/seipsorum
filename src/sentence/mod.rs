//! 文
use meaning_bear_sound;
use word::stem::ActionVerbStem;

/* TODO: To be written.
/// 文を表す構造体．
pub struct Sentence;

/// 客体文を表す構造体．
pub struct ObjectiveSentence;
*/

/// 主体文を表す構造体．
///
/// 主体文は主題部と陳述部からなる (p. 162)．
///
/// > 日本語では、主題に引き続きその主体的解説を陳述部として展開する「主題-陳述」構造の形式を取る文が多い。この種の文は**主体文** (Subjective Sentence) と称せられるべきものである。
pub struct SubjectiveSentence {
    /// 主題部
    pub theme: Theme,

    /// 陳述部
    pub rheme: Rheme,
}

/// 主題部を表す構造体．
///
/// 主題部は名詞節と提題助辞からなる (p.161)．
///
/// > _花ハ_植物ノ繁殖器官デアル。  
/// > _月ハ_地球ノ衛生デアル。  
/// > の様な文では、「花」「月」が提題助辞「-ハ」によって文の主題として提出せられ、「-ハ」と共に**主題部** (Theme) を成し、……
pub struct Theme {
    /// 名詞節
    pub nominal_clause: NominalClause,

    /// 提題助辞
    pub thematic_postposition: meaning_bear_sound::ThematicPostposition,
}

/// 名詞節を表す構造体．
///
/// 主要部は実名詞，従属部は連体節である．連体節は省略されうる．
pub struct NominalClause {
    /// 実名詞
    pub noun_substantive: meaning_bear_sound::NounSubstantive,

    /// 連体節
    pub attributive_clause: Option<AttributiveClause>,
}

/// 連体節を表す構造体．
///
/// 主要部は連体語，従属部は？
pub struct AttributiveClause {
    /// 連体語
    // NOTE: 名称要検討
    pub attributive: Attributive,
}

/// 連体語を表す列挙型．
///
// NOTE: 「連体語」，`Attributive`, `AttributiveActionVerb`, `AttributiveQualitativeVerb`, `AttributiveNoun` という用語は未確定である．
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
    pub action_verb_stem: ActionVerbStem,
}

/// 形状動詞の連体形を表す構造体．
///
/// 主要部は形状動詞終止形・連体形形成の文法接尾辞，従属部は形状動詞幹である．
pub struct AttributiveQualitativeVerb {
    /// 形状動詞終止形・連体形形成の文法接尾辞
    // NOTE: これにあたる範疇はまだ作成されていない．仮に動詞文法接尾辞を取るものとする．
    pub grammatical_verbal_suffix: meaning_bear_sound::GrammaticalVerbalSuffix,

    /// 形状動詞幹
    pub qualitative_verb_stem: QualitativeVerbStem,
}

/// 形状動詞幹を表す構造体．
pub struct QualitativeVerbStem;

/// 名詞の連用形を表す構造体．
///
/// 主要部は繋辞連体形，従属部は名詞節である．たぶん．
pub struct AttributiveNoun {
    /// 繋辞連体形
    // NOTE: これにあたる範疇はまだ作成されていない．仮に繋辞を取るものとする．
    pub copulative_suffix: meaning_bear_sound::CopulativeSuffix,

    /// 名詞節
    pub nominal_clause: Box<NominalClause>,
}

/// 陳述部を表す構造体．
///
/// 『新論』において陳述部の明確な定義なはされていない．
pub struct Rheme;
