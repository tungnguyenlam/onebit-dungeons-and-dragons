use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VfxTier {
    T0Ascii,
    T1Minimal,
    T2Color,
    T3Full,
}

impl VfxTier {
    pub fn from_env() -> Self {
        if std::env::var("DND_REDUCED_MOTION").is_ok() {
            return VfxTier::T0Ascii;
        }

        if let Ok(tier) = std::env::var("DND_VFX_TIER") {
            match tier.as_str() {
                "0" => return VfxTier::T0Ascii,
                "1" => return VfxTier::T1Minimal,
                "2" => return VfxTier::T2Color,
                "3" => return VfxTier::T3Full,
                _ => {}
            }
        }

        VfxTier::T2Color
    }

    pub fn supports_animations(&self) -> bool {
        matches!(self, VfxTier::T2Color | VfxTier::T3Full)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VfxEffect {
    DamageFloater {
        position: Position,
        text: String,
        lifetime_ms: u64,
        started_at: Instant,
    },
    TilePulse {
        position: Position,
        lifetime_ms: u64,
        started_at: Instant,
    },
    ScreenWipe {
        lifetime_ms: u64,
        started_at: Instant,
        wipe_type: WipeType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WipeType {
    FadeOut,
    FadeIn,
    SlideLeft,
    SlideRight,
}

pub struct VfxEngine {
    effects: Vec<VfxEffect>,
    tier: VfxTier,
    frame_interval: Duration,
    ui_animations: Vec<UiAnimation>,
}

impl VfxEngine {
    pub fn new() -> Self {
        let tier = VfxTier::from_env();
        let frame_interval = if tier.supports_animations() {
            Duration::from_millis(33)
        } else {
            Duration::from_millis(250)
        };

        Self {
            effects: Vec::new(),
            tier,
            frame_interval,
            ui_animations: Vec::new(),
        }
    }

    pub fn tier(&self) -> VfxTier {
        self.tier
    }

    pub fn frame_interval(&self) -> Duration {
        self.frame_interval
    }

    pub fn spawn_damage_floater(&mut self, position: Position, damage: i32, is_critical: bool) {
        if !self.tier.supports_animations() {
            return;
        }

        let text = if is_critical {
            format!("-{}!", damage)
        } else {
            format!("-{}", damage)
        };

        self.effects.push(VfxEffect::DamageFloater {
            position,
            text,
            lifetime_ms: 1500,
            started_at: Instant::now(),
        });
    }

    pub fn spawn_tile_pulse(&mut self, position: Position) {
        if !self.tier.supports_animations() {
            return;
        }

        self.effects.push(VfxEffect::TilePulse {
            position,
            lifetime_ms: 1000,
            started_at: Instant::now(),
        });
    }

    pub fn spawn_screen_wipe(&mut self, wipe_type: WipeType, duration_ms: u64) {
        if !self.tier.supports_animations() {
            return;
        }

        self.effects.push(VfxEffect::ScreenWipe {
            lifetime_ms: duration_ms,
            started_at: Instant::now(),
            wipe_type,
        });
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.effects.retain(|effect| {
            let lifetime = match effect {
                VfxEffect::DamageFloater {
                    lifetime_ms,
                    started_at,
                    ..
                } => now.duration_since(*started_at).as_millis() < (*lifetime_ms as u128),
                VfxEffect::TilePulse {
                    lifetime_ms,
                    started_at,
                    ..
                } => now.duration_since(*started_at).as_millis() < (*lifetime_ms as u128),
                VfxEffect::ScreenWipe {
                    lifetime_ms,
                    started_at,
                    ..
                } => now.duration_since(*started_at).as_millis() < (*lifetime_ms as u128),
            };
            lifetime
        });

        self.tick_animations();
    }

    pub fn effects(&self) -> &[VfxEffect] {
        &self.effects
    }

    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    pub fn clear(&mut self) {
        self.effects.clear();
    }
}

impl Default for VfxEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DamageFloaterData {
    pub text: String,
    pub opacity: f32,
    pub y_offset: f32,
}

impl VfxEngine {
    pub fn get_damage_floaters(&self) -> Vec<(Position, DamageFloaterData)> {
        let now = Instant::now();
        self.effects
            .iter()
            .filter_map(|effect| {
                if let VfxEffect::DamageFloater {
                    position,
                    text,
                    lifetime_ms,
                    started_at,
                } = effect
                {
                    let elapsed = now.duration_since(*started_at).as_millis() as f32;
                    let progress = elapsed / *lifetime_ms as f32;
                    let opacity = 1.0 - progress;
                    let y_offset = progress * 2.0;

                    Some((
                        *position,
                        DamageFloaterData {
                            text: text.clone(),
                            opacity,
                            y_offset,
                        },
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_tile_pulses(&self) -> Vec<(Position, f32)> {
        let now = Instant::now();
        self.effects
            .iter()
            .filter_map(|effect| {
                if let VfxEffect::TilePulse {
                    position,
                    lifetime_ms,
                    started_at,
                } = effect
                {
                    let elapsed = now.duration_since(*started_at).as_millis() as f32;
                    let progress = elapsed / *lifetime_ms as f32;
                    let pulse = (progress * std::f32::consts::PI * 2.0).sin().abs();

                    Some((*position, pulse))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiAnimationType {
    FadeIn,
    FadeOut,
    SlideLeft,
    SlideRight,
    Blink,
    Pulse,
    Shake,
}

pub struct UiAnimation {
    pub id: String,
    pub animation_type: UiAnimationType,
    pub started_at: Instant,
    pub duration_ms: u64,
    pub target_element: String,
}

impl UiAnimation {
    pub fn new(
        id: &str,
        animation_type: UiAnimationType,
        duration_ms: u64,
        target_element: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            animation_type,
            started_at: Instant::now(),
            duration_ms,
            target_element: target_element.to_string(),
        }
    }

    pub fn progress(&self) -> f32 {
        let elapsed = self.started_at.elapsed().as_millis() as f32;
        (elapsed / self.duration_ms as f32).min(1.0)
    }

    pub fn is_complete(&self) -> bool {
        self.progress() >= 1.0
    }

    pub fn value(&self) -> f32 {
        let p = self.progress();
        match self.animation_type {
            UiAnimationType::FadeIn => p,
            UiAnimationType::FadeOut => 1.0 - p,
            UiAnimationType::SlideLeft => p,
            UiAnimationType::SlideRight => p,
            UiAnimationType::Blink => (p * 4.0).sin().abs(),
            UiAnimationType::Pulse => (p * std::f32::consts::PI * 2.0).sin().abs(),
            UiAnimationType::Shake => {
                if p < 0.3 {
                    ((p * 50.0).sin() * (1.0 - p * 3.0)).abs()
                } else {
                    0.0
                }
            }
        }
    }
}

impl VfxEngine {
    pub fn add_ui_animation(&mut self, animation: UiAnimation) {
        if !self.tier.supports_animations() {
            return;
        }
        self.ui_animations.push(animation);
    }

    pub fn trigger_fade_in(&mut self, element: &str, duration_ms: u64) {
        self.add_ui_animation(UiAnimation::new(
            &format!("fade_in_{}", element),
            UiAnimationType::FadeIn,
            duration_ms,
            element,
        ));
    }

    pub fn trigger_fade_out(&mut self, element: &str, duration_ms: u64) {
        self.add_ui_animation(UiAnimation::new(
            &format!("fade_out_{}", element),
            UiAnimationType::FadeOut,
            duration_ms,
            element,
        ));
    }

    pub fn trigger_slide(&mut self, element: &str, left: bool, duration_ms: u64) {
        let anim_type = if left {
            UiAnimationType::SlideLeft
        } else {
            UiAnimationType::SlideRight
        };
        self.add_ui_animation(UiAnimation::new(
            &format!("slide_{}", element),
            anim_type,
            duration_ms,
            element,
        ));
    }

    pub fn trigger_pulse(&mut self, element: &str, duration_ms: u64) {
        self.add_ui_animation(UiAnimation::new(
            &format!("pulse_{}", element),
            UiAnimationType::Pulse,
            duration_ms,
            element,
        ));
    }

    pub fn trigger_blink(&mut self, element: &str, duration_ms: u64) {
        self.add_ui_animation(UiAnimation::new(
            &format!("blink_{}", element),
            UiAnimationType::Blink,
            duration_ms,
            element,
        ));
    }

    pub fn trigger_shake(&mut self, element: &str, duration_ms: u64) {
        self.add_ui_animation(UiAnimation::new(
            &format!("shake_{}", element),
            UiAnimationType::Shake,
            duration_ms,
            element,
        ));
    }

    pub fn tick_animations(&mut self) {
        self.ui_animations.retain(|a| !a.is_complete());
    }

    pub fn get_animations_for_element(&self, element: &str) -> Vec<&UiAnimation> {
        self.ui_animations
            .iter()
            .filter(|a| a.target_element == element)
            .collect()
    }

    pub fn has_active_animations(&self) -> bool {
        !self.ui_animations.is_empty()
    }
}
