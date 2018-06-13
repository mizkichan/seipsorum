//! 接尾辞

use meaning_bear_sound::NominalSuffix;
use meaning_bear_sound::VerbalSuffix;

/// 接尾辞
pub enum Suffix {
    /// 名詞接尾辞
    NominalSuffix(NominalSuffix),

    /// 動詞接尾辞
    VerbalSuffix(VerbalSuffix),
}
