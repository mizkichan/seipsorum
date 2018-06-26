//! 形状動詞
/// 形状動詞
pub struct QualitativeVerb(String);

impl QualitativeVerb {
    /// 動作動詞を作成する．
    pub fn new<T>(value: T) -> QualitativeVerb
    where
        T: Into<String>,
    {
        QualitativeVerb(value.into())
    }
}
