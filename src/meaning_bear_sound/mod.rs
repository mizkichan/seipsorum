//! 有意音

pub mod bound_sound;
pub mod unbound_sound;
pub use self::bound_sound::*;
pub use self::unbound_sound::*;

/// 有意音
pub enum MeaningBearingSound {
    /// 自立音
    UnboundSound(UnboundSound),

    /// 従属音
    BoundSound(BoundSound),
}
