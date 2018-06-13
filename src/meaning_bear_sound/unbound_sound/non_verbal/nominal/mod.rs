//! 名詞

pub mod noun_qualitative;
pub mod noun_substantive;
pub use self::noun_qualitative::*;
pub use self::noun_substantive::*;

/// 名詞
pub enum Nominal {
    /// 実名詞
    NounSubstantive(NounSubstantive),

    /// 形状名詞
    NounQualitative(NounQualitative),
}
