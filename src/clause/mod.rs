//! 節

use clause;
use meaning_bear_sound;
use word;

/// 名詞節を表す構造体．
///
/// 主要部は実名詞，従属部は連体節である．実名詞は省略されうる．
pub struct NominalClause {
    /// 実名詞
    pub noun_substantive: meaning_bear_sound::NounSubstantive,

    /// 連体節
    pub attributive_clause: Option<clause::AttributiveClause>,
}

/// 連体節を表す構造体．
///
/// 主要部は連体語，従属部は？
pub struct AttributiveClause {
    /// 連体語
    // NOTE: 名称要検討
    pub attributive: word::attributive::Attributive,
}
