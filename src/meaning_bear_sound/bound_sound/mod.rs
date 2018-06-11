//! 従属音

pub mod suffix;
use self::suffix::Suffix;

/// 従属音
pub enum BoundSound {
    /// 接尾辞
    Suffix(Suffix),
    ///// 助辞
    //PostPosition(PostPosition),
}

/*
/// 助辞
pub enum PostPosition {
    /// 句助辞
    PhrasePostPosition(PhrasePostPosition),

    /// 文助辞
    ClausePostPosition(ClausePostPosition),
}

/// 句助辞
pub enum PhrasePostPosition {
    /// 副助辞
    AdverbalPostPosition(AdverbalPostPosition),

    /// 提題助辞
    ThematicPostPosition(ThematicPostPosition),
}

/// 文助辞
pub enum ClausePostPosition {
    /// 終続助辞
    ConjunctivePostPosition(ConjunctivePostPosition),

    /// 終助辞
    SentenceFinalPostPosition(SentenceFinalPostPosition),
}
*/
