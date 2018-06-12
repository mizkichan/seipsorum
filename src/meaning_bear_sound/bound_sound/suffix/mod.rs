//! 接尾辞

pub mod nominal_suffix;
pub mod verbal_suffix;
use self::nominal_suffix::NominalSuffix;
use self::verbal_suffix::VerbalSuffix;

/// 接尾辞
pub enum Suffix {
    /// 名詞接尾辞
    NominalSuffix(NominalSuffix),

    /// 動詞接尾辞
    VerbalSuffix(VerbalSuffix),
}
