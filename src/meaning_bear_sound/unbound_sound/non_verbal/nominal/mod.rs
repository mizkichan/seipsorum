//! 名詞

pub mod noun_qualitative;
pub mod noun_substantive;
use self::noun_qualitative::NounQualitative;
use self::noun_substantive::NounSubstantive;

/// 名詞
pub enum Nominal {
    /// 実名詞
    NounSubstantive(NounSubstantive),

    /// 形状名詞
    NounQualitative(NounQualitative),
}
