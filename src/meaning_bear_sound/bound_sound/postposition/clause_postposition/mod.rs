//! 文助辞

pub mod conjunctive_postposition;
pub mod sentence_final_postposition;
use self::conjunctive_postposition::ConjunctivePostposition;
use self::sentence_final_postposition::SentenceFinalPostposition;

/// 文助辞
pub enum ClausePostposition {
    /// 終続助辞
    ConjunctivePostposition(ConjunctivePostposition),

    /// 終助辞
    SentenceFinalPostposition(SentenceFinalPostposition),
}
