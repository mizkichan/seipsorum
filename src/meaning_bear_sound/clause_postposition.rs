//! 文助辞

use meaning_bear_sound::ConjunctivePostposition;
use meaning_bear_sound::SentenceFinalPostposition;

/// 文助辞
pub enum ClausePostposition {
    /// 終続助辞
    ConjunctivePostposition(ConjunctivePostposition),

    /// 終助辞
    SentenceFinalPostposition(SentenceFinalPostposition),
}
