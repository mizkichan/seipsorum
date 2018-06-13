//! 副用詞

use meaning_bear_sound::Adverb;
use meaning_bear_sound::Attribute;

/// 副用詞
pub enum Qualifier {
    /// 連体詞
    Attribute(Attribute),

    /// 副詞
    Adverb(Adverb),
}
