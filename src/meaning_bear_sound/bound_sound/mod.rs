//! 従属音

pub mod postposition;
pub mod suffix;
pub use self::postposition::*;
pub use self::suffix::*;

/// 従属音
pub enum BoundSound {
    /// 接尾辞
    Suffix(Suffix),

    /// 助辞
    Postposition(Postposition),
}
