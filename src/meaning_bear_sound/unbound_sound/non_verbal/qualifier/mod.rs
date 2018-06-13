//! 副用詞

pub mod adverb;
pub mod attribute;
pub use self::adverb::*;
pub use self::attribute::*;

/// 副用詞
pub enum Qualifier {
    /// 連体詞
    Attribute(Attribute),

    /// 副詞
    Adverb(Adverb),
}
