//! 動詞

pub mod action_verb;
pub mod qualitative_verb;
pub use self::action_verb::*;
pub use self::qualitative_verb::*;

/// 動詞
pub enum Verbal {
    /// 動作動詞
    ActionVerb(ActionVerb),

    /// 形状動詞
    QualitativeVerb(QualitativeVerb),
}
