//! 非動詞

pub use meaning_bear_sound::Nominal;
pub use meaning_bear_sound::Qualifier;

/// 非動詞
pub enum NonVerbal {
    /// 名詞
    Nominal(Nominal),

    /// 副用詞
    Qualifier(Qualifier),
}
