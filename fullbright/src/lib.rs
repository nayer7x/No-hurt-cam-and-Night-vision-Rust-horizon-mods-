//! Port of the datapack's behavior. Host integration must call `tick` each game tick.
use std::collections::{HashMap, HashSet};
#[derive(Default)]
pub struct Fullbright {
    pub disabled_by_default: bool,
    toggled: HashSet<String>,
    blacklisted: HashSet<String>,
    trigger: HashMap<String, u32>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Action { GiveNightVision { player: String, seconds: u32, amplifier: u8, hide_particles: bool }, ClearNightVision { player: String }, Message { player: String, enabled: bool }, Sound { player: String, enabled: bool } }
impl Fullbright {
    pub fn set_blacklisted(&mut self, player: &str, yes: bool) { if yes {self.blacklisted.insert(player.into());} else {self.blacklisted.remove(player);} }
    pub fn request_toggle(&mut self, player: &str) { if !self.blacklisted.contains(player) { *self.trigger.entry(player.into()).or_default() += 1; } }
    pub fn enabled(&self, player: &str) -> bool { !self.blacklisted.contains(player) && (self.disabled_by_default == self.toggled.contains(player)) }
    pub fn tick<'a>(&mut self, players: impl IntoIterator<Item=&'a str>) -> Vec<Action> {
        let mut actions = Vec::new();
        for player in players {
            if self.trigger.remove(player).unwrap_or(0) > 0 && !self.blacklisted.contains(player) {
                if !self.toggled.insert(player.into()) { self.toggled.remove(player); }
                let enabled = self.enabled(player);
                actions.push(Action::Message { player: player.into(), enabled });
                actions.push(Action::Sound { player: player.into(), enabled });
            }
            if self.enabled(player) { actions.push(Action::GiveNightVision {player:player.into(),seconds:11,amplifier:1,hide_particles:true}); }
            else { actions.push(Action::ClearNightVision {player:player.into()}); }
        }
        actions
    }
}
#[cfg(test)] mod tests { use super::*; #[test] fn toggle() {let mut m=Fullbright::default();assert!(m.enabled("a"));m.request_toggle("a");assert!(m.tick(["a"]).iter().any(|a|matches!(a,Action::Message{enabled:false,..})));assert!(!m.enabled("a"));} }

 
// Native Horizon registration. The feature-specific client hooks are still
// missing: this entry point does NOT activate the ported behavior in-game.
use mod_api::*;
#[derive(Default)]
struct NativePort;
#[mod_api::horizon_mod]
impl Hooks for NativePort {}
