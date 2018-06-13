//! 句助辞

pub mod adverbal_postposition;
pub mod thematic_postposition;
pub use self::adverbal_postposition::*;
pub use self::thematic_postposition::*;

/// 句助辞
pub enum PhrasePostposition {
    /// 副助辞
    AdverbalPostposition(AdverbalPostposition),

    /// 提題助辞
    ThematicPostposition(ThematicPostposition),
}
