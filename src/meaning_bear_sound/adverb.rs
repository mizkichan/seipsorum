//! 副詞
/// 副詞
pub struct Adverb(String);

impl Adverb {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> Adverb
    where
        T: Into<String>,
    {
        Adverb(value.into())
    }
}
