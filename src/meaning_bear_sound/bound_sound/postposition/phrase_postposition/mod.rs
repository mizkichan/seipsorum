//! 句助辞

pub mod adverbal_postposition;
pub mod thematic_postposition;
use self::adverbal_postposition::AdverbalPostPosition;
use self::thematic_postposition::ThematicPostPosition;

/// 句助辞
pub enum PhrasePostPosition {
    /// 副助辞
    AdverbalPostPosition(AdverbalPostPosition),

    /// 提題助辞
    ThematicPostPosition(ThematicPostPosition),
}
