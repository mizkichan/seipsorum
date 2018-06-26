//! 動作動詞
/// 動作動詞
pub struct ActionVerb(String);

impl ActionVerb {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> ActionVerb
    where
        T: Into<String>,
    {
        ActionVerb(value.into())
    }
}
