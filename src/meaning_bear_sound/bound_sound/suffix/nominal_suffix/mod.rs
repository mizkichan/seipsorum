//! 名詞接尾辞

pub mod case_suffix;
pub mod copulative_suffix;
use self::case_suffix::CaseSuffix;
use self::copulative_suffix::CopulativeSuffix;

/// 名詞接尾辞
pub enum NominalSuffix {
    /// 格接尾辞
    CaseSuffix(CaseSuffix),

    /// 繋辞
    CopulativeSuffix(CopulativeSuffix),
}
