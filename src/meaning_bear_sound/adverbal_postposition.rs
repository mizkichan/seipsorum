//! 副助辞
/// 副助辞
pub struct AdverbalPostposition(String);

impl AdverbalPostposition {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> AdverbalPostposition
    where
        T: Into<String>,
    {
        AdverbalPostposition(value.into())
    }
}
