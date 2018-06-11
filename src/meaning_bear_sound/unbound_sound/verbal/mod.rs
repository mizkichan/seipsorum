//! 動詞

pub mod action_verb;
pub mod qualitative_verb;
use self::action_verb::ActionVerb;
use self::qualitative_verb::QualitativeVerb;

/// 動詞
pub enum Verbal {
    /// 動作動詞
    ActionVerb(ActionVerb),

    /// 形状動詞
    QualitativeVerb(QualitativeVerb),
}
