//! 文助辞

pub mod conjunctive_postposition;
pub mod sentence_final_postposition;
use self::conjunctive_postposition::ConjunctivePostPosition;
use self::sentence_final_postposition::SentenceFinalPostPosition;

/// 文助辞
pub enum ClausePostPosition {
    /// 終続助辞
    ConjunctivePostPosition(ConjunctivePostPosition),

    /// 終助辞
    SentenceFinalPostPosition(SentenceFinalPostPosition),
}
