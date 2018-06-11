//! 名詞

pub mod noun_qualitative;
pub mod noun_substansive;
use self::noun_qualitative::NounQualitative;
use self::noun_substansive::NounSubstansive;

/// 名詞
pub enum Nominal {
    /// 実名詞
    NounSubstansive(NounSubstansive),

    /// 形状名詞
    NounQualitative(NounQualitative),
}
