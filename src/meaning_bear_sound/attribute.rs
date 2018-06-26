//! 連体詞
/// 連体詞
pub struct Attribute(String);

impl Attribute {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> Attribute
    where
        T: Into<String>,
    {
        Attribute(value.into())
    }
}
