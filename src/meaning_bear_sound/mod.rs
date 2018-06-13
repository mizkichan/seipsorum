//! 有意音

mod action_verb;
mod adverb;
mod adverbal_postposition;
mod attribute;
mod bound_sound;
mod case_suffix;
mod clause_postposition;
mod conjunctive_postposition;
mod copulative_suffix;
mod derivational_verbal_suffix;
mod grammatical_verbal_suffix;
mod nominal;
mod nominal_suffix;
mod non_verbal;
mod noun_qualitative;
mod noun_substantive;
mod phrase_postposition;
mod postposition;
mod qualifier;
mod qualitative_verb;
mod sentence_final_postposition;
mod suffix;
mod thematic_postposition;
mod unbound_sound;
mod verbal;
mod verbal_suffix;
pub use self::action_verb::ActionVerb;
pub use self::adverb::Adverb;
pub use self::adverbal_postposition::AdverbalPostposition;
pub use self::attribute::Attribute;
pub use self::bound_sound::BoundSound;
pub use self::case_suffix::CaseSuffix;
pub use self::clause_postposition::ClausePostposition;
pub use self::conjunctive_postposition::ConjunctivePostposition;
pub use self::copulative_suffix::CopulativeSuffix;
pub use self::derivational_verbal_suffix::DerivationalVerbalSuffix;
pub use self::grammatical_verbal_suffix::GrammaticalVerbalSuffix;
pub use self::nominal::Nominal;
pub use self::nominal_suffix::NominalSuffix;
pub use self::non_verbal::NonVerbal;
pub use self::noun_qualitative::NounQualitative;
pub use self::noun_substantive::NounSubstantive;
pub use self::phrase_postposition::PhrasePostposition;
pub use self::postposition::Postposition;
pub use self::qualifier::Qualifier;
pub use self::qualitative_verb::QualitativeVerb;
pub use self::sentence_final_postposition::SentenceFinalPostposition;
pub use self::suffix::Suffix;
pub use self::thematic_postposition::ThematicPostposition;
pub use self::unbound_sound::UnboundSound;
pub use self::verbal::Verbal;
pub use self::verbal_suffix::VerbalSuffix;

/// 有意音
pub enum MeaningBearingSound {
    /// 自立音
    UnboundSound(UnboundSound),

    /// 従属音
    BoundSound(BoundSound),
}
