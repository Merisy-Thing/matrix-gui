use crate::ui::{GuiError, GuiResult};
use core::cell::Cell;
use enum_iterator::Sequence;

pub trait WidgetId: Copy + Sequence + PartialEq + Default {
    fn id(&self) -> usize;
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
#[repr(u8)]
pub enum RenderStatus {
    NeedsRedraw = 0,
    Rendered = 1,
    Normal = 2,
    Dragging = 3,
    Pressed = 4,
    Released = 5,
    Focused = 6,
    Defocused = 7,
    Triggered = 8,
    Inactive = 9,
    UserStatus0 = 10,
    UserStatus1 = 11,
    UserStatus2 = 12,
    UserStatus3 = 13,
    Unknown = 127,
}

impl From<u8> for RenderStatus {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NeedsRedraw,
            1 => Self::Rendered,
            2 => Self::Normal,
            3 => Self::Dragging,
            4 => Self::Pressed,
            5 => Self::Released,
            6 => Self::Focused,
            7 => Self::Defocused,
            8 => Self::Triggered,
            9 => Self::Inactive,
            10 => Self::UserStatus0,
            11 => Self::UserStatus1,
            12 => Self::UserStatus2,
            13 => Self::UserStatus3,
            _ => Self::Unknown,
        }
    }
}

const MASK_VALUE: u8 = 0x3F; // 6 bits for value
const MASK_INTERACTIVE: u8 = 0x80; // 1 bit for interactive
const MASK_INTERFLAG: u8 = 0x40; // 1 bit for interaction flag

#[derive(Clone, Debug)]
pub struct RenderState(Cell<u8>);

#[cfg(not(feature = "interaction"))]
impl RenderState {
    #[inline]
    const fn raw_val(&self) -> u8 {
        self.0.get()
    }

    #[inline]
    const fn flags(&self) -> u8 {
        self.0.get() & !MASK_VALUE
    }

    #[inline]
    pub fn set_status(&self, val: RenderStatus) {
        self.0.set(val as u8);
    }
}

#[cfg(feature = "interaction")]
impl RenderState {
    #[inline]
    const fn raw_val(&self) -> u8 {
        self.0.get() & MASK_VALUE
    }

    #[inline]
    const fn flags(&self) -> u8 {
        self.0.get() & !MASK_VALUE
    }

    #[inline]
    pub fn mark_as_interact(&self) {
        self.0.set(self.0.get() | MASK_INTERACTIVE);
    }

    pub fn set_status(&self, val: RenderStatus) {
        self.0.set(self.flags() | val as u8);
    }
}

impl RenderState {
    pub const fn new(status: RenderStatus) -> Self {
        Self(Cell::new(status as u8))
    }

    pub fn new_array<const N: usize>() -> [Self; N] {
        core::array::from_fn(|_| RenderState::needs_redraw())
    }

    pub const fn needs_redraw() -> Self {
        Self(Cell::new(RenderStatus::NeedsRedraw as u8))
    }

    pub fn status(&self) -> RenderStatus {
        self.raw_val().into()
    }

    #[inline]
    pub const fn is_interact(&self) -> bool {
        (self.flags() & MASK_INTERACTIVE) != 0
    }

    #[inline]
    pub const fn is_static(&self) -> bool {
        !self.is_interact()
    }

    #[inline]
    pub(crate) fn interflag(&self) -> bool {
        (self.0.get() & MASK_INTERFLAG) != 0
    }

    #[inline]
    pub(crate) fn set_interflag(&self, flag: bool) {
        let val = if flag { MASK_INTERFLAG } else { 0 };
        self.0.set(self.0.get() & !MASK_INTERFLAG | val);
    }

    pub fn force_redraw(&self) {
        self.0.set(RenderStatus::NeedsRedraw as u8);
    }

    pub fn compare_set(&self, other: RenderStatus) -> bool {
        let result = self.raw_val() == other as u8;
        if !result {
            self.set_status(other);
        }
        result
    }

    pub fn compare(&self, other: RenderStatus) -> bool {
        self.raw_val() == other as u8
    }
}

impl PartialEq for RenderState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug)]
pub struct WidgetStates<'a> {
    states: &'a [RenderState],
    #[cfg(feature = "animation")]
    anim_status: &'a [crate::prelude::AnimStatus],
    #[cfg(feature = "popup")]
    modal_active: Cell<bool>,
}

impl<'a> WidgetStates<'a> {
    pub const fn new(states: &'a [RenderState]) -> Self {
        WidgetStates {
            states,
            #[cfg(feature = "animation")]
            anim_status: &[],
            #[cfg(feature = "popup")]
            modal_active: Cell::new(false),
        }
    }

    pub fn get_state<ID: WidgetId>(&self, widget_id: ID) -> GuiResult<&RenderState> {
        #[cfg(feature = "popup")]
        if self.is_modal_active() {
            return Err(GuiError::ModalActive);
        }
        self.states
            .get(widget_id.id())
            .ok_or(GuiError::InvalidWidgetId)
    }

    pub fn set_status<ID: WidgetId>(&self, widget_id: ID, status: RenderStatus) -> bool {
        if let Some(state) = self.states.get(widget_id.id()) {
            state.set_status(status);
            true
        } else {
            false
        }
    }

    pub fn get_internal_flag<ID: WidgetId>(&self, widget_id: ID) -> GuiResult<bool> {
        if let Some(state) = self.states.get(widget_id.id()) {
            Ok(state.interflag())
        } else {
            Err(GuiError::InvalidWidgetId)
        }
    }

    pub fn set_internal_flag<ID: WidgetId>(&self, widget_id: ID, value: bool) -> GuiResult<()> {
        if let Some(state) = self.states.get(widget_id.id()) {
            state.set_interflag(value);
            Ok(())
        } else {
            Err(GuiError::InvalidWidgetId)
        }
    }

    pub fn compare_set<ID: WidgetId>(&self, widget_id: ID, value: RenderStatus) -> bool {
        if let Some(state) = self.states.get(widget_id.id()) {
            state.compare_set(value)
        } else {
            false
        }
    }

    pub fn force_redraw_all(&self) {
        #[cfg(feature = "popup")]
        {
            self.set_modal_active(false);
        }

        for state in self.states.iter() {
            state.force_redraw();
        }
    }

    pub fn force_redraw_range<ID: WidgetId>(&self, start: ID, end: ID) {
        let mut curr = start;
        self.force_redraw(curr);
        while let Some(next) = curr.next() {
            self.force_redraw(next);

            if next == end {
                break;
            }
            curr = next;
        }
    }

    pub fn force_redraw_multi<ID: WidgetId>(&self, widget_ids: &[ID]) {
        for widget_id in widget_ids {
            self.force_redraw(*widget_id);
        }
    }

    pub fn force_redraw<ID: WidgetId>(&self, widget_id: ID) {
        if let Some(state) = self.states.get(widget_id.id()) {
            state.force_redraw();
        }
    }
}

#[cfg(feature = "popup")]
impl<'a> WidgetStates<'a> {
    pub fn set_modal_active(&self, modal_status: bool) {
        self.modal_active.set(modal_status);
    }

    pub const fn is_modal_active(&self) -> bool {
        self.modal_active.get()
    }
}

#[cfg(feature = "animation")]
impl<'a> WidgetStates<'a> {
    pub const fn new_with_anim(
        states: &'a [RenderState],
        anim_status: &'a [crate::prelude::AnimStatus],
    ) -> Self {
        WidgetStates {
            states,
            anim_status,
            #[cfg(feature = "popup")]
            modal_active: Cell::new(false),
        }
    }

    pub fn take_anim_status(&self, anim_id: crate::prelude::AnimId) -> GuiResult<Option<i32>> {
        self.anim_status
            .get(anim_id as usize)
            .map_or(Err(GuiError::InvalidAnimId), |s| Ok(s.take()))
    }

    pub fn get_anim_status(&self, anim_id: crate::prelude::AnimId) -> GuiResult<Option<i32>> {
        self.anim_status
            .get(anim_id as usize)
            .map_or(Err(GuiError::InvalidAnimId), |s| Ok(s.get()))
    }
}

impl<'a> WidgetStates<'a> {
    #[cfg(feature = "interaction")]
    pub(crate) fn mark_as_interact<ID: WidgetId>(&self, widget_id: ID) {
        if let Some(state) = self.states.get(widget_id.id()) {
            state.mark_as_interact();
        }
    }

    pub fn should_redraw<ID: WidgetId>(&self, widget_id: ID) -> bool {
        if let Ok(state) = self.get_state(widget_id) {
            state.compare(RenderStatus::NeedsRedraw)
        } else {
            false
        }
    }

    pub fn should_redraw_multi<ID: WidgetId>(&self, widget_ids: &[ID]) -> bool {
        for widget_id in widget_ids {
            if let Ok(state) = self.get_state(*widget_id)
                && state.compare(RenderStatus::NeedsRedraw)
            {
                return true;
            }
        }

        false
    }

    pub fn should_redraw_static(&self) -> bool {
        for state in self.states {
            if state.is_static() && state.compare(RenderStatus::NeedsRedraw) {
                return true;
            }
        }

        false
    }

    pub fn should_redraw_any(&self) -> bool {
        for state in self.states {
            if state.compare(RenderStatus::NeedsRedraw) {
                return true;
            }
        }

        false
    }
}
