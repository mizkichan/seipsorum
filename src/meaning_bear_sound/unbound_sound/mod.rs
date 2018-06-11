//! 自立音

pub mod non_verbal;
pub mod verbal;
use self::non_verbal::NonVerbal;
use self::verbal::Verbal;

/// 自立音
pub enum UnboundSound {
    /// 動詞
    Verbal(Verbal),

    /// 非動詞
    NonVerbal(NonVerbal),
}
