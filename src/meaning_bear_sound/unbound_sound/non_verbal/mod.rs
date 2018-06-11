//! 非動詞

pub mod nominal;
pub mod qualifier;
use self::nominal::Nominal;
use self::qualifier::Qualifier;

/// 非動詞
pub enum NonVerbal {
    /// 名詞
    Nominal(Nominal),

    /// 副用詞
    Qualifier(Qualifier),
}
