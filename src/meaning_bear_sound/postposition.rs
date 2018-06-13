//! 助辞

use meaning_bear_sound::ClausePostposition;
use meaning_bear_sound::PhrasePostposition;

/// 助辞
pub enum Postposition {
    /// 句助辞
    PhrasePostposition(PhrasePostposition),

    /// 文助辞
    ClausePostposition(ClausePostposition),
}
