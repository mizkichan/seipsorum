//! 有意音

pub mod bound_sound;
pub mod unbound_sound;

use self::bound_sound::BoundSound;
use self::unbound_sound::UnboundSound;

/// 有意音
pub enum MeaningBearingSound {
    /// 自立音
    UnboundSound(UnboundSound),

    /// 従属音
    BoundSound(BoundSound),
}
