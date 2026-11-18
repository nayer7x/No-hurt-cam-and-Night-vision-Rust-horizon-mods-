//! Camera tilt port. Integrate with Horizon's camera/render hook when documented.
#[derive(Clone, Copy, Debug)]
pub struct Config { pub intensity: f32 }
impl Default for Config { fn default() -> Self { Self { intensity: 0.0 } } }
#[derive(Clone, Copy, Debug, Default)]
pub struct CameraState { pub is_living: bool, pub is_dead_or_dying: bool, pub death_time: f32, pub hurt_time: f32, pub hurt_duration: i32, pub hurt_dir: f32, pub damage_tilt_strength: f64 }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation { pub axis: Axis, pub degrees: f32 }
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis { Y, Z }
/// `None` means leave the game's vanilla camera effect intact.
/// `Some(rotations)` means replace vanilla hurt/death camera rotations.
pub fn camera_rotations(state: CameraState, config: Config) -> Option<Vec<Rotation>> {
    let intensity = config.intensity;
    if intensity >= 1.0 { return None; }
    let mut result = Vec::new();
    if !state.is_living { return Some(result); }
    if state.is_dead_or_dying {
        let time = state.death_time.min(20.0);
        result.push(Rotation { axis: Axis::Z, degrees: (40.0 - 8000.0 / (time + 200.0)) * intensity });
    }
    if state.hurt_time < 0.0 || state.hurt_duration <= 0 { return Some(result); }
    let fraction = state.hurt_time / state.hurt_duration as f32;
    let wave = (fraction.powi(4) * std::f32::consts::PI).sin();
    result.push(Rotation { axis: Axis::Y, degrees: -state.hurt_dir });
    result.push(Rotation { axis: Axis::Z, degrees: (-wave * 14.0 * state.damage_tilt_strength as f32) * intensity });
    result.push(Rotation { axis: Axis::Y, degrees: state.hurt_dir });
    Some(result)
}
#[cfg(test)] mod tests { use super::*; #[test] fn default_disables_shake() {let s=CameraState {is_living:true,hurt_time:5.0,hurt_duration:10, ..Default::default()};let r=camera_rotations(s,Config::default()).unwrap();assert_eq!(r[1].degrees,0.0);} #[test] fn vanilla_at_one() {assert!(camera_rotations(CameraState::default(),Config{intensity:1.0}).is_none());} }

 
// Native Horizon registration. The feature-specific client hooks are still
// missing: this entry point does NOT activate the ported behavior in-game.
use mod_api::*;
#[derive(Default)]
struct NativePort;
#[mod_api::horizon_mod]
impl Hooks for NativePort {}
