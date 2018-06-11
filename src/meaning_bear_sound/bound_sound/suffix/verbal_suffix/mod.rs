pub mod derivational_verbal_suffix;
pub mod grammatical_verbal_suffix;
use self::derivational_verbal_suffix::DerivationalVerbalSuffix;
use self::grammatical_verbal_suffix::GrammaticalVerbalSuffix;

/// 動詞接尾辞
pub enum VerbalSuffix {
    /// 文法接尾辞
    GrammaticalVerbalSuffix(GrammaticalVerbalSuffix),

    /// 派生接尾辞
    DerivationalVerbalSuffix(DerivationalVerbalSuffix),
}
