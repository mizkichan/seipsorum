//! 文
use meaning_bear_sound::bound_sound::postposition::phrase_postposition::thematic_postposition::ThematicPostposition;
use meaning_bear_sound::bound_sound::suffix::nominal_suffix::copulative_suffix::CopulativeSuffix;
use meaning_bear_sound::bound_sound::suffix::verbal_suffix::grammatical_verbal_suffix::GrammaticalVerbalSuffix;
use meaning_bear_sound::unbound_sound::non_verbal::nominal::noun_qualitative::NounQualitative;
use meaning_bear_sound::unbound_sound::non_verbal::nominal::noun_substantive::NounSubstantive;
use meaning_bear_sound::unbound_sound::non_verbal::qualifier::attribute::Attribute;
use meaning_bear_sound::unbound_sound::verbal::Verbal;

/// 文を表す列挙型．
///
/// 文は客体文または主体文である (pp. 161-162)．
pub enum Sentence {
    /// 客体文
    ObjectiveSentence(ObjectiveSentence),

    /// 主体文
    SubjectiveSentence(SubjectiveSentence),
}

/// 客体文を表す構造体．
pub struct ObjectiveSentence;

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
    pub thematic_postposition: ThematicPostposition,
}

/// 名詞節を表す列挙型．
///
/// 名詞節は次のうちいずれかである:
///
/// 1. 実名詞
/// 2. 連体節
/// 3. 連体節と名詞節
///
/// これは p.166 における議論を元にしたものではあるものの，本文とはやや異なる．
/// 本文においては 1. の場合は含まれず，2. の場合は動名詞である言われるのみであり，3. の場合は連体形が直後に実名詞を伴う場合に限定されている．
///
/// 本文と異なる定義とした理由は，
///
/// 1. については，実名詞が単独で名詞節を構成できないとすると不便であること
/// 2. については，動名詞とは連体形，および連体形に導かれる節すなわち連体節に他ならないこと
/// 3. については，「波の極めて静かな青い海」は [波の極めて静かな][[青い]海] と解されるべきであると思われること
///
/// である．3.  に関しては「波の極めて静かな青い」という連体節が構成できるのかも知れず，今後変更がありうる．
pub enum NominalClause {
    /// 実名詞
    NounSubstantive(NounSubstantive),

    /// 連体節
    AttributiveClause(AttributiveClause),

    /// 連体節と名詞節
    // NOTE: 名称要検討
    NominalWithAttributive {
        /// 連体節
        attributive_clause: AttributiveClause,

        /// 名詞節
        nominal_clause: Box<NominalClause>,
    },
}

/// 連体節を表す構造体．
pub struct AttributiveClause;

/// 陳述部を表す構造体．
///
/// 『新論』において陳述部の明確な定義なはされていない．
pub struct Rheme;
