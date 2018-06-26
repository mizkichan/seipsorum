//! 提題助辞
/// 提題助辞
pub struct ThematicPostposition(String);

impl ThematicPostposition {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> ThematicPostposition
    where
        T: Into<String>,
    {
        ThematicPostposition(value.into())
    }
}
