//! 接続助辞
/// 接続助辞
pub struct ConjunctivePostposition(String);

impl ConjunctivePostposition {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> ConjunctivePostposition
    where
        T: Into<String>,
    {
        ConjunctivePostposition(value.into())
    }
}
