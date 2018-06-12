//! 句助辞

pub mod adverbal_postposition;
pub mod thematic_postposition;
use self::adverbal_postposition::AdverbalPostposition;
use self::thematic_postposition::ThematicPostposition;

/// 句助辞
pub enum PhrasePostposition {
    /// 副助辞
    AdverbalPostposition(AdverbalPostposition),

    /// 提題助辞
    ThematicPostposition(ThematicPostposition),
}
