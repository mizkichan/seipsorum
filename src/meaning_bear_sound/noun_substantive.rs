//! 実名詞
/// 実名詞
pub struct NounSubstantive(String);

impl NounSubstantive {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> NounSubstantive
    where
        T: Into<String>,
    {
        NounSubstantive(value.into())
    }
}
