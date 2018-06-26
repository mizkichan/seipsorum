//! 終助辞
/// 終助辞
pub struct SentenceFinalPostposition(String);

impl SentenceFinalPostposition {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> SentenceFinalPostposition
    where
        T: Into<String>,
    {
        SentenceFinalPostposition(value.into())
    }
}
