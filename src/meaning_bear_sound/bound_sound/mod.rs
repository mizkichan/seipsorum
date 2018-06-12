//! 従属音

pub mod postposition;
pub mod suffix;
use self::postposition::Postposition;
use self::suffix::Suffix;

/// 従属音
pub enum BoundSound {
    /// 接尾辞
    Suffix(Suffix),

    /// 助辞
    Postposition(Postposition),
}
