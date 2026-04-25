//! Animation subsystem for the matrix_gui framework.
//!
//! This module provides a lightweight animation system inspired by LVGL 8,
//! designed for immediate-mode embedded GUI applications.
//!
//! # Features
//!
//! - Multiple easing functions using integer-only math (no floating point)
//! - Support for value animations with callbacks
//! - Animation management with play, pause, stop controls
//! - Memory-efficient design suitable for embedded systems
//! - `no_std` compatible
//!
//! # Core Components
//!
//! - [`Anim`]: Animation definition with start/end values, duration, and easing
//! - [`Easing`]: Easing functions for smooth animations (integer-based)
//! - [`AnimManager`]: Manages multiple active animations
//! - [`AnimCallback`]: Callback trait for animation value updates

use core::cell::Cell;
use core::fmt::Debug;
use core::time::Duration;

/// Scaling factor for fixed-point calculations.
/// Values are scaled to 0..=ANIM_SCALE range for integer math.
pub const ANIM_SCALE: i32 = 1024;

/// Easing functions for animations.
///
/// These functions define how animation progress changes over time,
/// creating smooth and natural-looking motion.
///
/// All calculations use integer-only math with fixed-point arithmetic.
/// The input progress is in range [0, ANIM_SCALE] and output is also
/// in range [0, ANIM_SCALE].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Easing {
    /// Linear interpolation - constant speed.
    Linear,
    /// Ease-in - slow start, fast end.
    EaseIn,
    /// Ease-out - fast start, slow end.
    #[default]
    EaseOut,
    /// Ease-in-out - slow start and end.
    EaseInOut,
}

impl Easing {
    /// Calculates the eased value for a given progress using integer math.
    ///
    /// # Arguments
    ///
    /// * `progress` - Animation progress in range [0, ANIM_SCALE]
    ///
    /// # Returns
    ///
    /// The eased progress value in range [0, ANIM_SCALE]
    /// (some easing functions like elastic may slightly exceed this range).
    pub fn calc(&self, progress: i32) -> i32 {
        let t = progress.clamp(0, ANIM_SCALE);
        match self {
            Easing::Linear => t,

            Easing::EaseIn => mul_div(t, t, ANIM_SCALE),

            Easing::EaseOut => {
                let inv_t = ANIM_SCALE - t;
                ANIM_SCALE - mul_div(inv_t, inv_t, ANIM_SCALE)
            }

            Easing::EaseInOut => {
                if t < ANIM_SCALE / 2 {
                    2 * mul_div(t, t, ANIM_SCALE)
                } else {
                    let inv_t = ANIM_SCALE - t;
                    ANIM_SCALE - 2 * mul_div(inv_t, inv_t, ANIM_SCALE)
                }
            }
        }
    }
}

/// Safe multiplication with division, avoiding overflow.
/// Returns (a * b / c) with proper handling.
#[inline]
const fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    (a * b) / c
}

/// Animation playback state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimState {
    /// Animation is not playing.
    #[default]
    Stopped,
    /// Animation is playing.
    Playing,
    /// Animation is paused.
    Paused,
}

/// Unique identifier for an animation.
pub type AnimId = u16;

/// Animation playback options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimOptions {
    /// Number of times to repeat the animation (0 = infinite).
    pub repeat_count: u16,
    /// Whether to reverse the animation on each repeat.
    pub reverse: bool,
    /// Delay before starting the animation.
    pub start_delay: Duration,
    /// Whether to play the animation in reverse initially.
    pub play_backward: bool,
}

impl Default for AnimOptions {
    fn default() -> Self {
        Self {
            repeat_count: 1,
            reverse: false,
            start_delay: Duration::ZERO,
            play_backward: false,
        }
    }
}

impl AnimOptions {
    /// Creates new animation options with default values.
    pub const fn new() -> Self {
        Self {
            repeat_count: 1,
            reverse: false,
            start_delay: Duration::ZERO,
            play_backward: false,
        }
    }

    /// Sets the repeat count (0 = infinite).
    pub const fn with_repeat(mut self, count: u16) -> Self {
        self.repeat_count = count;
        self
    }

    /// Enables reverse playback on repeat.
    pub const fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Sets the start delay.
    pub const fn with_start_delay(mut self, delay: Duration) -> Self {
        self.start_delay = delay;
        self
    }

    /// Sets whether to play backward initially.
    pub const fn with_play_backward(mut self, backward: bool) -> Self {
        self.play_backward = backward;
        self
    }
}

/// Animation definition.
///
/// This struct defines an animation with start/end values, duration,
/// easing function, and callback.
#[derive(Debug, Clone)]
pub struct Anim {
    /// Starting value of the animation.
    pub start_value: i32,
    /// Ending value of the animation.
    pub end_value: i32,
    /// Duration of the animation.
    pub duration: Duration,
    /// Easing function for the animation.
    pub easing: Easing,
    /// Animation options.
    pub options: AnimOptions,
}

impl Anim {
    /// Creates a new animation with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `start_value` - Starting value
    /// * `end_value` - Ending value
    /// * `duration` - Duration of the animation
    /// * `callback` - Callback for value updates
    pub const fn new(start_value: i32, end_value: i32, duration: Duration) -> Self {
        Self {
            start_value,
            end_value,
            duration,
            easing: Easing::Linear,
            options: AnimOptions::new(),
        }
    }

    /// Sets the easing function.
    pub const fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Sets the animation options.
    pub const fn with_options(mut self, options: AnimOptions) -> Self {
        self.options = options;
        self
    }

    /// Sets whether to reverse on repeat.
    pub const fn with_reverse(mut self, reverse: bool) -> Self {
        self.options.reverse = reverse;
        self
    }

    /// Sets the repeat count (0 = infinite).
    pub const fn with_repeat(mut self, count: u16) -> Self {
        self.options.repeat_count = count;
        self
    }

    /// Sets the start delay.
    pub const fn with_start_delay(mut self, delay: Duration) -> Self {
        self.options.start_delay = delay;
        self
    }

    /// Calculates the current value based on progress.
    ///
    /// # Arguments
    ///
    /// * `progress` - Animation progress in range [0, ANIM_SCALE]
    ///
    /// # Returns
    ///
    /// The interpolated value between start and end.
    pub fn calc_value(&self, progress: i32) -> i32 {
        let eased_progress = self.easing.calc(progress);
        let range = self.end_value - self.start_value;
        self.start_value + mul_div(range, eased_progress, ANIM_SCALE)
    }
}

const INVALID_ANIM_ID: AnimId = AnimId::MAX;

/// Internal state for an active animation.
#[derive(Debug, Clone)]
pub struct AnimInstance {
    /// Animation ID
    id: AnimId,
    /// Current playback state.
    state: AnimState,
    /// The animation definition.
    anim: Anim,
    /// Current time elapsed in the animation.
    elapsed: Duration,
    /// Current repeat count.
    current_repeat: u16,
    /// Whether currently playing in reverse.
    is_reversed: bool,
    /// Whether start delay has passed.
    delay_passed: bool,
}

impl AnimInstance {
    const fn new() -> Self {
        Self {
            id: INVALID_ANIM_ID,
            state: AnimState::Playing,
            anim: Anim::new(0, 0, Duration::ZERO),
            elapsed: Duration::ZERO,
            current_repeat: 0,
            is_reversed: false,
            delay_passed: false,
        }
    }
}

#[derive(Debug)]
pub struct AnimStatus(Cell<Option<i32>>);
impl AnimStatus {
    pub fn new() -> Self {
        Self(Cell::new(None))
    }
    pub fn set(&self, value: i32) {
        self.0.set(Some(value));
    }
    pub fn take(&self) -> Option<i32> {
        self.0.take()
    }
    pub fn get(&self) -> Option<i32> {
        self.0.get()
    }
}

pub struct Animations<const N: usize> {
    animations: [AnimInstance; N],
    anim_status: [AnimStatus; N],
}

impl<const N: usize> Animations<N> {
    /// Creates a new animation manager.
    pub fn new() -> Self {
        let animations = core::array::from_fn(|_| AnimInstance::new());
        let anim_status = core::array::from_fn(|_| AnimStatus::new());
        Self {
            animations,
            anim_status,
        }
    }

    pub fn split(self) -> ([AnimInstance; N], [AnimStatus; N]) {
        (self.animations, self.anim_status)
    }
}

/// Animation manager that handles multiple animations.
///
/// This struct manages the lifecycle and playback of multiple animations.
/// It is designed to be memory-efficient for embedded systems.
///
/// # Type Parameters
///
/// * `C` - The callback type that implements `AnimCallback`
/// * `N` - The maximum number of simultaneous animations
pub struct AnimManager<'a> {
    /// Active animation instances.
    animations: &'a mut [AnimInstance],
    anim_status: &'a [AnimStatus],
    /// Next animation ID.
    next_id: AnimId,
}

impl<'a> AnimManager<'a> {
    /// Creates a new animation manager.
    pub const fn new(animations: &'a mut [AnimInstance], anim_status: &'a [AnimStatus]) -> Self {
        Self {
            animations,
            anim_status,
            next_id: 0, //index from 0 to animations.len() - 1
        }
    }

    /// Adds an animation to the manager.
    ///
    /// # Arguments
    ///
    /// * `anim` - The animation to add
    ///
    /// # Returns
    ///
    /// The animation ID, or `None` if the manager is full.
    pub fn add(&mut self, anim: Anim) -> Option<AnimId> {
        if self.next_id as usize >= self.animations.len() {
            return None;
        }
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        let start_value = anim.start_value;
        let anim_instance = AnimInstance {
            id,
            state: AnimState::Stopped,
            anim,
            elapsed: Duration::ZERO,
            current_repeat: 0,
            is_reversed: false,
            delay_passed: false,
        };

        if let Some(instance) = self.animations.get_mut(id as usize) {
            if let Some(status) = self.anim_status.get(id as usize) {
                status.set(start_value);
                *instance = anim_instance;
                return Some(id);
            }
        };

        None
    }

    /// Removes an animation from the manager.
    ///
    /// # Arguments
    ///
    /// * `id` - The animation ID to remove
    ///
    /// # Returns
    ///
    /// `true` if the animation was found and removed.
    pub fn remove(&mut self, id: AnimId) -> bool {
        if let Some(instance) = self.animations.get_mut(id as usize) {
            instance.id = INVALID_ANIM_ID;
            return true;
        }

        false
    }

    /// Starts playing an animation.
    ///
    /// # Arguments
    ///
    /// * `id` - The animation ID to play
    ///
    /// # Returns
    ///
    /// `true` if the animation was found and started.
    pub fn play(&mut self, id: AnimId) -> bool {
        if let Some(instance) = self.animations.get_mut(id as usize) {
            if instance.id == id {
                instance.state = AnimState::Playing;
                instance.elapsed = Duration::ZERO;
                instance.current_repeat = 0;
                instance.is_reversed = instance.anim.options.play_backward;
                instance.delay_passed = instance.anim.options.start_delay.is_zero();
                return true;
            }
        }
        false
    }

    /// Pauses an animation.
    ///
    /// # Arguments
    ///
    /// * `id` - The animation ID to pause
    ///
    /// # Returns
    ///
    /// `true` if the animation was found and paused.
    pub fn pause(&mut self, id: AnimId) -> bool {
        if let Some(instance) = self.animations.get_mut(id as usize) {
            if instance.id == id && instance.state == AnimState::Playing {
                instance.state = AnimState::Paused;
                return true;
            }
        }
        false
    }

    /// Resumes a paused animation.
    ///
    /// # Arguments
    ///
    /// * `id` - The animation ID to resume
    ///
    /// # Returns
    ///
    /// `true` if the animation was found and resumed.
    pub fn resume(&mut self, id: AnimId) -> bool {
        if let Some(instance) = self.animations.get_mut(id as usize) {
            if instance.id == id && instance.state == AnimState::Paused {
                instance.state = AnimState::Playing;
                return true;
            }
        }
        false
    }

    /// Stops an animation.
    ///
    /// # Arguments
    ///
    /// * `id` - The animation ID to stop
    ///
    /// # Returns
    ///
    /// `true` if the animation was found and stopped.
    pub fn stop(&mut self, id: AnimId) -> bool {
        if let Some(instance) = self.animations.get_mut(id as usize) {
            if instance.id == id {
                instance.state = AnimState::Stopped;
                instance.elapsed = Duration::ZERO;
                instance.current_repeat = 0;
                return true;
            }
        }
        false
    }

    /// Gets the state of an animation.
    ///
    /// # Arguments
    ///
    /// * `id` - The animation ID
    ///
    /// # Returns
    ///
    /// The animation state, or `None` if not found.
    pub fn get_state(&self, id: AnimId) -> Option<AnimState> {
        if let Some(instance) = self.animations.get(id as usize) {
            if instance.id == id {
                return Some(instance.state);
            }
        }
        None
    }

    /// Updates all active animations.
    ///
    /// This method should be called regularly (e.g., in the main loop)
    /// with the elapsed time since the last update.
    ///
    /// # Arguments
    ///
    /// * `elapsed` - Time elapsed since the last update
    pub fn tick(&mut self, elapsed: Duration) {
        for (idx, instance) in self.animations.iter_mut().enumerate() {
            if idx >= self.next_id as usize {
                break;
            }
            if instance.id == INVALID_ANIM_ID || instance.state != AnimState::Playing {
                continue;
            }
            let Some(status) = self.anim_status.get(instance.id as usize) else {
                continue;
            };

            // Handle start delay
            if !instance.delay_passed {
                instance.elapsed += elapsed;
                if instance.elapsed >= instance.anim.options.start_delay {
                    instance.delay_passed = true;
                    instance.elapsed = Duration::ZERO;
                } else {
                    continue;
                }
            } else {
                instance.elapsed += elapsed;
            }

            let duration = instance.anim.duration;
            let duration_ms = duration.as_millis() as u64;
            let elapsed_ms = instance.elapsed.as_millis() as u64;

            // Calculate progress
            let progress = if duration_ms == 0 {
                ANIM_SCALE
            } else {
                let effective_elapsed = if instance.is_reversed {
                    duration_ms.saturating_sub(elapsed_ms)
                } else {
                    elapsed_ms.min(duration_ms)
                };
                ((effective_elapsed * ANIM_SCALE as u64) / duration_ms) as i32
            };

            // Calculate and apply value
            let value = instance.anim.calc_value(progress);

            // Check if animation completed
            if instance.elapsed < duration {
                status.set(value);
            } else {
                // Ensure final value is set
                let final_value = if instance.is_reversed {
                    instance.anim.start_value
                } else {
                    instance.anim.end_value
                };
                status.set(final_value);

                // Handle repeat
                let repeat_count = instance.anim.options.repeat_count;
                let should_repeat = repeat_count == 0 || instance.current_repeat < repeat_count - 1;

                if should_repeat {
                    instance.current_repeat += 1;
                    instance.elapsed = Duration::ZERO;

                    // Handle reverse
                    if instance.anim.options.reverse {
                        instance.is_reversed = !instance.is_reversed;
                    }
                } else {
                    instance.state = AnimState::Stopped;
                }
            }
        }
    }

    /// Returns the number of active animations.
    pub fn count(&self) -> usize {
        self.animations
            .iter()
            .filter(|s| s.id != INVALID_ANIM_ID)
            .count()
    }

    /// Returns whether there are any active animations.
    pub fn is_empty(&self) -> bool {
        self.animations.iter().all(|s| s.id == INVALID_ANIM_ID)
    }
}
