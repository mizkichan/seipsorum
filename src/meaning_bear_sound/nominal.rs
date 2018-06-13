//! 名詞

use meaning_bear_sound::NounQualitative;
use meaning_bear_sound::NounSubstantive;

/// 名詞
pub enum Nominal {
    /// 実名詞
    NounSubstantive(NounSubstantive),

    /// 形状名詞
    NounQualitative(NounQualitative),
}
