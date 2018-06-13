//! 動詞

use meaning_bear_sound::ActionVerb;
use meaning_bear_sound::QualitativeVerb;

/// 動詞
pub enum Verbal {
    /// 動作動詞
    ActionVerb(ActionVerb),

    /// 形状動詞
    QualitativeVerb(QualitativeVerb),
}
