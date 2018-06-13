//! 名詞接尾辞

use meaning_bear_sound::CaseSuffix;
use meaning_bear_sound::CopulativeSuffix;

/// 名詞接尾辞
pub enum NominalSuffix {
    /// 格接尾辞
    CaseSuffix(CaseSuffix),

    /// 繋辞
    CopulativeSuffix(CopulativeSuffix),
}
