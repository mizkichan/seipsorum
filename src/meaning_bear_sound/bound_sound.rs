//! 従属音

pub use meaning_bear_sound::Postposition;
pub use meaning_bear_sound::Suffix;

/// 従属音
pub enum BoundSound {
    /// 接尾辞
    Suffix(Suffix),

    /// 助辞
    Postposition(Postposition),
}
