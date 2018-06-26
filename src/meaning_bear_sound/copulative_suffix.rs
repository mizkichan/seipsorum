//! 繋辞
/// 繋辞
pub struct CopulativeSuffix(String);

impl CopulativeSuffix {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> CopulativeSuffix
    where
        T: Into<String>,
    {
        CopulativeSuffix(value.into())
    }
}
