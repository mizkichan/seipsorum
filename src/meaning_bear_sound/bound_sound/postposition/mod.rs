//! 助辞

pub mod clause_postposition;
pub mod phrase_postposition;
pub use self::clause_postposition::*;
pub use self::phrase_postposition::*;

/// 助辞
pub enum Postposition {
    /// 句助辞
    PhrasePostposition(PhrasePostposition),

    /// 文助辞
    ClausePostposition(ClausePostposition),
}
