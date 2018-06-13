//! 句助辞

use meaning_bear_sound::AdverbalPostposition;
use meaning_bear_sound::ThematicPostposition;

/// 句助辞
pub enum PhrasePostposition {
    /// 副助辞
    AdverbalPostposition(AdverbalPostposition),

    /// 提題助辞
    ThematicPostposition(ThematicPostposition),
}
