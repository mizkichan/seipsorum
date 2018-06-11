pub mod adverb;
pub mod attribute;
use self::adverb::Adverb;
use self::attribute::Attribute;

/// 副用詞
pub enum Qualifier {
    /// 連体詞
    Attribute(Attribute),

    /// 副詞
    Adverb(Adverb),
}
