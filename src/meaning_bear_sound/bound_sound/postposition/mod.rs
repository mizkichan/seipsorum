//! 接尾辞

pub mod clause_postposition;
pub mod phrase_postposition;
use self::clause_postposition::ClausePostPosition;
use self::phrase_postposition::PhrasePostPosition;

/// 助辞
pub enum PostPosition {
    /// 句助辞
    PhrasePostPosition(PhrasePostPosition),

    /// 文助辞
    ClausePostPosition(ClausePostPosition),
}
