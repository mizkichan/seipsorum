//! 名詞節

use clause::AttributiveClause;
use meaning_bear_sound::{CaseSuffix, NounSubstantive};

/// 名詞節を表す構造体．
/// 「本」「この本」「私の本」「買った本」「赤い本」「綺麗な本」など．
pub struct NominalClause {
    /// 連体節
    pub attributive_clause: Option<AttributiveClause>,

    /// 実名詞
    pub noun_substantive: NounSubstantive,
}

impl NominalClause {
    /// 名詞節を作成する．
    pub fn new<A, N>(attribute: A, noun: N) -> NominalClause
    where
        A: Into<Option<AttributiveClause>>,
        N: Into<NounSubstantive>,
    {
        NominalClause {
            attributive_clause: attribute.into(),
            noun_substantive: noun.into(),
        }
    }
}

/// 格接尾辞付き名詞節を表す構造体．
pub struct NominalClauseWithCase {
    /// 名詞節
    pub nominal_clause: NominalClause,

    /// 格接尾辞
    pub case_suffix: CaseSuffix,
}

impl NominalClauseWithCase {
    /// 格接尾辞付き名詞節を作成する．
    pub fn new<N, C>(noun: N, case: C) -> NominalClauseWithCase
    where
        N: Into<NominalClause>,
        C: Into<CaseSuffix>,
    {
        NominalClauseWithCase {
            nominal_clause: noun.into(),
            case_suffix: case.into(),
        }
    }
}
