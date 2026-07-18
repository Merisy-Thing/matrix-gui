//! Focus management module for the matrix_gui framework.
//!
//! This module provides keyboard focus management functionality, allowing widgets
//! to receive and manage keyboard input focus. It includes:
//!
//! - [`Focused`]: Enum representing the focus state of a widget
//! - [`FocusTracker`]: Tracks the current focus position and size
//! - [`FocusState`]: Manages focus state for a fixed number of widgets
//!
//! # Core Concepts
//!
//! ## Focus Management
//!
//! The focus system allows widgets to be navigated using keyboard input.
//! Widgets can register themselves as focusable, and the focus tracker
//! maintains the current focus position.
//!
//! ## Focus States
//!
//! - `No`: Widget is not focused
//! - `Yes`: Widget is currently focused
//! - `Trigger`: Widget received a trigger action (e.g., Enter key)
//!
//! # Example
//!
//! ```rust
//! use matrix_gui::prelude::*;
//!
//! // Create a focus state for up to 4 widgets
//! let mut focus_state = FocusState::<4>::new();
//!
//! // Navigate focus
//! focus_state.focus_next();
//! focus_state.focus_prev();
//! focus_state.trigger_focus();
//! ```

use embedded_graphics::prelude::Point;

use crate::ui::Interaction;

/// Type alias for widget focus identifiers.
///
/// This type represents the unique identifier for a widget in the focus system.
/// It is typically derived from `WidgetId::id()`.
pub type FocusID = u16; // WidgetId::id()

/// Focus state of a widget.
///
/// This enum represents the current focus state of a widget, indicating
/// whether it is focused, not focused, or has received a trigger action.
///
/// # Variants
///
/// - `No`: Widget is not focused
/// - `Yes`: Widget is currently focused
/// - `Trigger`: Widget received a trigger action (e.g., Enter key press)
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Focused {
    /// Widget is not focused.
    No,
    /// Widget is currently focused.
    Yes,
    /// Widget received a trigger action.
    Trigger,
}

/// Converts a `Focused` state into an `Interaction`.
///
/// This implementation maps focus states to interaction events:
/// - `Focused::No` → `Interaction::None`
/// - `Focused::Yes` → `Interaction::Pressed(Point::zero())`
/// - `Focused::Trigger` → `Interaction::Release(Point::zero())`
impl From<Focused> for Interaction {
    fn from(focused: Focused) -> Self {
        match focused {
            Focused::No => Self::None,
            Focused::Yes => Self::Pressed(Point::zero()),
            Focused::Trigger => Self::Clicked(Point::zero()),
        }
    }
}

/// Converts a `Focused` state into a `RenderStatus`.
///
/// This implementation maps focus states to render status:
/// - `Focused::No` → `RenderStatus::Normal`
/// - `Focused::Yes` → `RenderStatus::Focused`
/// - `Focused::Trigger` → `RenderStatus::Triggered`
impl From<Focused> for crate::widget_state::RenderStatus {
    fn from(focused: Focused) -> Self {
        match focused {
            Focused::No => Self::Normal,
            Focused::Yes => Self::Focused,
            Focused::Trigger => Self::Triggered,
        }
    }
}

/// Focus tracker for managing current focus position.
///
/// This struct tracks the current focus position among registered widgets,
/// the total number of registered widgets, and any pending trigger action.
///
/// # Fields
///
/// - `pos`: Current focus position (index into widgets array)
/// - `size`: Total number of registered widgets
/// - `trigger`: Optional trigger action for current widget
#[derive(Debug, Default)]
pub struct FocusTracker {
    /// Current focus position (index into widgets array).
    pub(crate) pos: usize,
    /// Total number of registered widgets.
    pub(crate) size: usize,
    /// Optional trigger action for current widget.
    pub(crate) trigger: Option<FocusID>,
}

/// Focus state manager for a fixed number of widgets.
///
/// This struct manages focus state for up to N widgets, tracking their
/// IDs and maintaining the current focus position. It provides methods
/// for navigating focus and triggering actions on focused widgets.
///
/// # Type Parameters
///
/// * `N` - Maximum number of focusable widgets
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // Create a focus state for up to 4 widgets
/// let mut focus_state = FocusState::<4>::new();
///
/// // Navigate focus
/// focus_state.focus_next();
/// focus_state.focus_prev();
/// focus_state.trigger_focus();
/// ```
#[derive(Debug)]
pub struct FocusState<const N: usize> {
    /// Array of widget IDs in registration order.
    pub(crate) widgets_id: [FocusID; N],
    /// Focus tracker managing position and trigger state.
    pub(crate) tracker: FocusTracker,
}

impl<const N: usize> FocusState<N> {
    /// Creates a new focus state with no registered widgets.
    ///
    /// # Returns
    ///
    /// A new `FocusState` instance with empty widget array and default tracker.
    pub fn new() -> Self {
        Self {
            widgets_id: [0; N],
            tracker: FocusTracker::default(),
        }
    }

    /// Clears all focus state.
    ///
    /// This method resets the focus position to 0, clears the widget count,
    /// and removes any pending trigger action.
    pub fn clear_focus(&mut self) {
        self.tracker.pos = 0;
        self.tracker.size = 0;
        self.tracker.trigger = None;
    }

    /// Moves focus to the next widget.
    ///
    /// This method advances focus to the next registered widget, wrapping
    /// around to the first widget if at the end. Any pending trigger
    /// action is cleared.
    ///
    /// # Returns
    ///
    /// `true` if focus was moved, `false` if no widgets are registered.
    #[allow(clippy::should_implement_trait)]
    pub fn focus_next(&mut self) -> bool {
        self.focus_next_by(1)
    }

    /// Moves focus to the previous widget.
    ///
    /// This method moves focus to the previous registered widget, wrapping
    /// around to the last widget if at the beginning. Any pending trigger
    /// action is cleared.
    ///
    /// # Returns
    ///
    /// `true` if focus was moved, `false` if no widgets are registered.
    pub fn focus_prev(&mut self) -> bool {
        self.focus_prev_by(1)
    }

    /// Moves focus forward by the specified number of steps.
    ///
    /// This method advances focus by `steps` widgets, wrapping around to
    /// the beginning as needed. Steps larger than the number of registered
    /// widgets wrap around modulo the widget count (e.g. with 4 widgets,
    /// `focus_next_by(5)` is equivalent to `focus_next_by(1)`). Any
    /// pending trigger action is cleared.
    ///
    /// Calling this with `steps == 0` leaves the position unchanged but
    /// still clears the pending trigger and returns `true` when widgets
    /// are registered.
    ///
    /// # Arguments
    ///
    /// * `steps` - Number of steps to move forward
    ///
    /// # Returns
    ///
    /// `true` if the focus state is valid (at least one widget registered and
    /// the current position is in range), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use matrix_gui::prelude::*;
    ///
    /// let mut focus_state = FocusState::<8>::new();
    /// // Skip ahead by 3 widgets at once
    /// focus_state.focus_next_by(3);
    /// ```
    pub fn focus_next_by(&mut self, steps: usize) -> bool {
        self.tracker.trigger = None;
        let size = self.tracker.size;
        if size == 0 || self.tracker.pos >= size {
            return false;
        }
        self.tracker.pos = (self.tracker.pos + steps) % size;
        true
    }

    /// Moves focus backward by the specified number of steps.
    ///
    /// This method moves focus backward by `steps` widgets, wrapping around
    /// to the end as needed. Steps larger than the number of registered
    /// widgets wrap around modulo the widget count (e.g. with 4 widgets,
    /// `focus_prev_by(5)` is equivalent to `focus_prev_by(1)`). Any
    /// pending trigger action is cleared.
    ///
    /// Calling this with `steps == 0` leaves the position unchanged but
    /// still clears the pending trigger and returns `true` when widgets
    /// are registered.
    ///
    /// # Arguments
    ///
    /// * `steps` - Number of steps to move backward
    ///
    /// # Returns
    ///
    /// `true` if the focus state is valid (at least one widget registered and
    /// the current position is in range), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use matrix_gui::prelude::*;
    ///
    /// let mut focus_state = FocusState::<8>::new();
    /// // Jump back by 2 widgets at once
    /// focus_state.focus_prev_by(2);
    /// ```
    pub fn focus_prev_by(&mut self, steps: usize) -> bool {
        self.tracker.trigger = None;
        let size = self.tracker.size;
        if size == 0 || self.tracker.pos >= size {
            return false;
        }
        // Add `size` before subtracting to avoid underflow when
        // `steps % size` is larger than `pos`.
        self.tracker.pos = (self.tracker.pos + size - (steps % size)) % size;
        true
    }

    /// Triggers an action on the currently focused widget.
    ///
    /// This method sets a trigger action for the widget at the current
    /// focus position. The trigger can be checked by widgets to respond
    /// to actions like Enter key presses.
    pub fn trigger_focus(&mut self) {
        if self.tracker.pos < self.tracker.size {
            self.tracker.trigger = self.widgets_id.get(self.tracker.pos).copied();
        }
    }
}

/// Default implementation creates a new empty focus state.
impl<const N: usize> Default for FocusState<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Runtime focus management interface.
///
/// This struct provides a runtime interface for managing focus state,
/// allowing widgets to register themselves and check their focus status.
pub(crate) struct Focus<'a> {
    /// Mutable reference to the widget IDs array.
    pub widgets_id: &'a mut [FocusID],
    /// Mutable reference to the focus tracker.
    pub tracker: &'a mut FocusTracker,
}

impl<'a> Focus<'a> {
    /// Creates a new focus interface from a focus state.
    ///
    /// # Type Parameters
    ///
    /// * `N` - Maximum number of widgets in the focus state
    ///
    /// # Arguments
    ///
    /// * `focus_state` - Mutable reference to the focus state
    ///
    /// # Returns
    ///
    /// A new `Focus` instance
    pub fn new<const N: usize>(focus_state: &'a mut FocusState<N>) -> Self {
        Self {
            widgets_id: &mut focus_state.widgets_id,
            tracker: &mut focus_state.tracker,
        }
    }

    /// Registers a widget as focusable.
    ///
    /// This method adds a widget ID to the focus system if it's not
    /// already registered and there's space available. Widgets should
    /// call this during their draw operation to ensure they can receive focus.
    ///
    /// # Arguments
    ///
    /// * `id` - The widget ID to register
    ///
    /// # Returns
    ///
    /// `true` if the widget was registered successfully,
    /// `false` if the widget was already registered or the
    /// focus system is full.
    pub fn register_focus(&mut self, id: usize) -> bool {
        let id = id as FocusID;
        if self.tracker.size > 0
            && self
                .widgets_id
                .get(..self.tracker.size)
                .is_some_and(|slice| slice.contains(&id))
        {
            return true;
        }

        if self.tracker.size < self.widgets_id.len() {
            if let Some(slot) = self.widgets_id.get_mut(self.tracker.size) {
                *slot = id;
                self.tracker.size += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Checks if a widget is currently focused.
    ///
    /// This method determines if the widget with the specified ID is
    /// at the current focus position.
    ///
    /// # Arguments
    ///
    /// * `id` - The widget ID to check
    ///
    /// # Returns
    ///
    /// `true` if the widget is focused, `false` otherwise.
    pub fn is_focused(&self, id: usize) -> bool {
        self.widgets_id
            .get(self.tracker.pos)
            .is_some_and(|&widget_id| widget_id == id as FocusID)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that builds a `FocusState` with `count` registered widgets.
    fn build_state<const N: usize>(count: usize) -> FocusState<N> {
        let mut state = FocusState::<N>::new();
        let mut focus = Focus::new(&mut state);
        for i in 0..count {
            assert!(focus.register_focus(i));
        }
        state
    }

    #[test]
    fn focus_next_by_advances_multiple_steps() {
        let mut state = build_state::<8>(4);
        // pos == 0
        assert!(state.focus_next_by(2));
        assert_eq!(state.tracker.pos, 2);
        assert!(state.focus_next_by(2));
        assert_eq!(state.tracker.pos, 0, "expected wrap-around to 0");
    }

    #[test]
    fn focus_next_by_wraps_when_steps_exceed_size() {
        let mut state = build_state::<8>(4);
        // (0 + 5) % 4 == 1
        assert!(state.focus_next_by(5));
        assert_eq!(state.tracker.pos, 1);
    }

    #[test]
    fn focus_next_by_zero_is_noop_but_clears_trigger() {
        let mut state = build_state::<8>(4);
        state.trigger_focus();
        assert!(state.tracker.trigger.is_some());
        assert!(state.focus_next_by(0));
        assert_eq!(state.tracker.pos, 0);
        assert!(state.tracker.trigger.is_none(), "trigger must be cleared");
    }

    #[test]
    fn focus_prev_by_decreases_multiple_steps() {
        let mut state = build_state::<8>(4);
        state.tracker.pos = 3;
        assert!(state.focus_prev_by(2));
        assert_eq!(state.tracker.pos, 1);
    }

    #[test]
    fn focus_prev_by_wraps_at_zero() {
        let mut state = build_state::<8>(4);
        // pos == 0 → (0 + 4 - 1) % 4 == 3
        assert!(state.focus_prev_by(1));
        assert_eq!(state.tracker.pos, 3);
        // pos == 3 → (3 + 4 - 2) % 4 == 1
        assert!(state.focus_prev_by(2));
        assert_eq!(state.tracker.pos, 1);
    }

    #[test]
    fn focus_prev_by_wraps_when_steps_exceed_size() {
        let mut state = build_state::<8>(4);
        // (0 + 4 - (5 % 4)) % 4 == (0 + 4 - 1) % 4 == 3
        assert!(state.focus_prev_by(5));
        assert_eq!(state.tracker.pos, 3);
    }

    #[test]
    fn single_step_methods_match_multi_step_with_one() {
        // focus_next() must remain equivalent to focus_next_by(1).
        let mut a = build_state::<8>(4);
        let mut b = build_state::<8>(4);
        for _ in 0..10 {
            let ra = a.focus_next();
            let rb = b.focus_next_by(1);
            assert_eq!(ra, rb);
            assert_eq!(a.tracker.pos, b.tracker.pos);
        }

        // focus_prev() must remain equivalent to focus_prev_by(1).
        let mut a = build_state::<8>(4);
        let mut b = build_state::<8>(4);
        for _ in 0..10 {
            let ra = a.focus_prev();
            let rb = b.focus_prev_by(1);
            assert_eq!(ra, rb);
            assert_eq!(a.tracker.pos, b.tracker.pos);
        }
    }

    #[test]
    fn multi_step_returns_false_without_widgets() {
        let mut state = FocusState::<4>::new();
        assert!(!state.focus_next_by(2));
        assert!(!state.focus_prev_by(2));
    }

    #[test]
    fn multi_step_handles_single_widget() {
        let mut state = build_state::<4>(1);
        // Any movement on a single-widget chain stays at 0.
        assert!(state.focus_next_by(3));
        assert_eq!(state.tracker.pos, 0);
        assert!(state.focus_prev_by(7));
        assert_eq!(state.tracker.pos, 0);
    }
}
