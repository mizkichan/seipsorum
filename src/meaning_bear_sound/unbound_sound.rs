//! 自立音

use meaning_bear_sound::NonVerbal;
use meaning_bear_sound::Verbal;

/// 自立音
pub enum UnboundSound {
    /// 動詞
    Verbal(Verbal),

    /// 非動詞
    NonVerbal(NonVerbal),
}
