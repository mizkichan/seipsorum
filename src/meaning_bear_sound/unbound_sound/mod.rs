//! 自立音

pub mod non_verbal;
pub mod verbal;
pub use self::non_verbal::*;
pub use self::verbal::*;

/// 自立音
pub enum UnboundSound {
    /// 動詞
    Verbal(Verbal),

    /// 非動詞
    NonVerbal(NonVerbal),
}
