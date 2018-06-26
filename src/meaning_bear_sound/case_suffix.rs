//! 格接尾辞
/// 格接尾辞
pub struct CaseSuffix(String);

impl CaseSuffix {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> CaseSuffix
    where
        T: Into<String>,
    {
        CaseSuffix(value.into())
    }
}
