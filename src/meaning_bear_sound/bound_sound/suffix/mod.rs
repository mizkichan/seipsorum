pub mod verbal_suffix;
use self::verbal_suffix::VerbalSuffix;

/// 接尾辞
pub enum Suffix {
    ///// 名詞接尾辞
    //NominalSuffix(NominalSuffix),
    /// 動詞接尾辞
    VerbalSuffix(VerbalSuffix),
}

/*
/// 名詞接尾辞
pub enum NominalSuffix {
    /// 格接尾辞
    CaseSuffix(CaseSuffix),

    /// 繋辞
    CopulativeSuffix(CopulativeSuffix),
}
*/
