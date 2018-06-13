//! 動詞接尾辞

pub mod derivational_verbal_suffix;
pub mod grammatical_verbal_suffix;
pub use self::derivational_verbal_suffix::*;
pub use self::grammatical_verbal_suffix::*;

/// 動詞接尾辞
pub enum VerbalSuffix {
    /// 文法接尾辞
    GrammaticalVerbalSuffix(GrammaticalVerbalSuffix),

    /// 派生接尾辞
    DerivationalVerbalSuffix(DerivationalVerbalSuffix),
}
