//! 動詞接尾辞

use meaning_bear_sound::DerivationalVerbalSuffix;
use meaning_bear_sound::GrammaticalVerbalSuffix;

/// 動詞接尾辞
pub enum VerbalSuffix {
    /// 文法接尾辞
    GrammaticalVerbalSuffix(GrammaticalVerbalSuffix),

    /// 派生接尾辞
    DerivationalVerbalSuffix(DerivationalVerbalSuffix),
}
