//! 非動詞

pub mod nominal;
pub mod qualifier;
pub use self::nominal::*;
pub use self::qualifier::*;

/// 非動詞
pub enum NonVerbal {
    /// 名詞
    Nominal(Nominal),

    /// 副用詞
    Qualifier(Qualifier),
}
