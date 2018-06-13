//! 文

use clause;
use meaning_bear_sound;

/* TODO: To be written.
/// 文を表す構造体．
pub struct Sentence;

/// 客体文を表す構造体．
pub struct ObjectiveSentence;
*/

/// 主体文を表す構造体．
///
/// 主体文は主題部と陳述部からなる (p. 162).
///
/// > 日本語では、主題に引き続きその主体的解説を陳述部として展開する「主題-陳述」構造の形式を取る文が多い。この種の文は**主体文** (Subjective Sentence) と称せられるべきものである。
///
/// 主題部は省略されうる (p. 165).
///
/// > 提題助辞を以って主題化せられた主語も、勿論ここに言う「主語」に含まれる。日本語では、脈絡によって主語の省略 (ellipsis) が見られる事は極く普通であるから、  
/// >
/// > > 見事ニ咲ク。  
/// > > 確カニ広イ。  
/// > > 実ニ親切ダッタ。  
/// >
/// > の様に、……
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
/// > > _花ハ_ 植物ノ繁殖器官デアル。  
/// > > _月ハ_ 地球ノ衛生デアル。  
/// >
/// > の様な文では、「花」「月」が提題助辞「-ハ」によって文の主題として提出せられ、「-ハ」と共に**主題部** (Theme) を成し、……
pub struct Theme {
    /// 名詞節
    pub nominal_clause: clause::NominalClause,

    /// 提題助辞
    pub thematic_postposition: meaning_bear_sound::ThematicPostposition,
}

/// 陳述部を表す構造体．
///
/// 『新論』において陳述部の明確な定義なはされていない．
pub struct Rheme;
