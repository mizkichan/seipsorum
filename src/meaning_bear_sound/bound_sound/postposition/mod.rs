//! 接尾辞

pub mod clause_postposition;
pub mod phrase_postposition;
use self::clause_postposition::ClausePostposition;
use self::phrase_postposition::PhrasePostposition;

/// 助辞
pub enum Postposition {
    /// 句助辞
    PhrasePostposition(PhrasePostposition),

    /// 文助辞
    ClausePostposition(ClausePostposition),
}
