//! 文助辞

pub mod conjunctive_postposition;
pub mod sentence_final_postposition;
pub use self::conjunctive_postposition::*;
pub use self::sentence_final_postposition::*;

/// 文助辞
pub enum ClausePostposition {
    /// 終続助辞
    ConjunctivePostposition(ConjunctivePostposition),

    /// 終助辞
    SentenceFinalPostposition(SentenceFinalPostposition),
}
