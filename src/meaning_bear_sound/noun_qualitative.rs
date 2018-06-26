//! 形状名詞
/// 形状名詞
pub struct NounQualitative(String);

impl NounQualitative {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> NounQualitative
    where
        T: Into<String>,
    {
        NounQualitative(value.into())
    }
}
