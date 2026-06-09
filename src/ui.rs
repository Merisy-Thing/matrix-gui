//! User Interface (UI) module for the matrix_gui framework.
//!
//! This module provides the core UI infrastructure including:
//! - Error handling for GUI operations
//! - Widget rendering and state management
//! - Interaction handling (touch/mouse input)
//! - Focus management for keyboard navigation
//! - Drawing primitives and utilities
//! - Alignment and layout helpers
//!
//! # Core Components
//!
//! - [`Ui`]: Main UI context that manages drawing, state, and interactions
//! - [`Widget`]: Trait for drawable UI components
//! - [`Response`]: Result type for widget operations
//! - [`Interaction`]: Input event types (pressed, drag, release)
//!
//! # Example
//!
//! ```ignore
//! use matrix_gui::prelude::*;
//! use embedded_graphics::primitives::Rectangle;
//!
//! // Create UI context
//! let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
//!
//! // Add widgets
//! ui.add(my_widget);
//!
//! // Handle interactions
//! ui.interact(Interaction::Pressed(point));
//! ```

#[cfg(feature = "interaction")]
use crate::region::Region;
use crate::style::Style;
use crate::widget_state::{RenderState, WidgetId, WidgetStates};
use core::fmt::Debug;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::pixelcolor::PixelColor;
#[cfg(feature = "clipped_draw")]
use embedded_graphics::prelude::DrawTargetExt;
#[cfg(feature = "interaction")]
use embedded_graphics::prelude::Point;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{Drawable, Pixel};

#[cfg(feature = "debug-color")]
static DBG_CNT: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// Error types that can occur during GUI operations.
///
/// This enum represents the various error conditions that can arise
/// when performing GUI operations such as drawing or widget management.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum GuiError {
    /// An error occurred during a drawing operation.
    DrawError,
    /// An invalid widget ID was provided.
    InvalidWidgetId,
    /// An invalid animation ID was provided.
    InvalidAnimId,
    /// A modal widget is already active.
    ModalActive,
}

/// Result type alias for GUI operations.
///
/// This type alias provides a convenient way to handle results from
/// GUI operations that can fail with [`GuiError`].
pub type GuiResult<T> = Result<T, GuiError>;

/// User interaction events.
///
/// This enum represents various types of user input events that can be
/// processed by the UI system, such as touch presses, drags, and releases.
#[cfg(feature = "interaction")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[non_exhaustive]
pub enum Interaction {
    /// A press event at the given point (e.g., finger down or mouse click).
    Pressed(Point),
    /// A drag event at the given point (e.g., finger movement while pressed).
    Drag(Point),
    /// A release event at the given point (e.g., finger up or mouse release).
    Release(Point),
    /// A click event at the given point
    Clicked(Point),
    /// No interaction event.
    #[default]
    None,
}

#[cfg(feature = "interaction")]
impl Interaction {
    /// Returns the point associated with this interaction, if any.
    ///
    /// # Returns
    ///
    /// `Some(point)` if the interaction has an associated point,
    /// `None` for `Interaction::None`.
    fn get_point(&self) -> Option<Point> {
        match self {
            Interaction::Pressed(p) => Some(*p),
            Interaction::Drag(p) => Some(*p),
            Interaction::Release(p) => Some(*p),
            Interaction::Clicked(p) => Some(*p),
            Interaction::None => None,
        }
    }
}

/// Response from a widget operation.
///
/// This enum represents the various responses that a widget can return
/// after processing an operation, such as drawing or handling interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Response {
    /// No significant response, widget is idle.
    Idle,
    /// The widget was clicked (e.g., checkbox toggled).
    Clicked,
    /// The widget is currently being pressed down.
    Pressed,
    /// The widget was released.
    Released,
    /// The widget gained focus.
    Focused,
    /// The widget lost focus.
    Defocused,
    /// The widget was selected (e.g., radio button, list item).
    Selected,
    /// The widget was triggered (e.g., button activated).
    Trigger,
    /// The widget's content was edited.
    Edited,
    /// The widget's value changed.
    ValueChanged,
    /// The widget is being hovered over.
    Hovered,
    /// The widget was scrolled.
    Scrolled,
    /// The widget was disabled.
    Disabled,
    /// User-defined response with custom code.
    User(u8),
    /// An error occurred during the operation.
    Error(GuiError),
}

// builder pattern
impl Response {
    /// Returns `true` if the response is idle.
    pub const fn is_idle(&self) -> bool {
        matches!(self, Response::Idle)
    }

    /// Returns `true` if the response is clicked.
    pub const fn is_clicked(&self) -> bool {
        matches!(self, Response::Clicked)
    }

    /// Returns `true` if the response is pressed.
    pub const fn is_pressed(&self) -> bool {
        matches!(self, Response::Pressed)
    }

    pub const fn is_released(&self) -> bool {
        matches!(self, Response::Released)
    }

    /// Returns `true` if the response is focused.
    pub const fn is_focused(&self) -> bool {
        matches!(self, Response::Focused)
    }

    /// Returns `true` if the response is defocused.
    pub const fn is_defocused(&self) -> bool {
        matches!(self, Response::Defocused)
    }

    /// Returns `true` if the response is selected.
    pub const fn is_selected(&self) -> bool {
        matches!(self, Response::Selected)
    }

    /// Returns `true` if the response is trigger.
    pub const fn is_trigger(&self) -> bool {
        matches!(self, Response::Trigger)
    }

    /// Returns `true` if the response is edited.
    pub const fn is_edited(&self) -> bool {
        matches!(self, Response::Edited)
    }

    /// Returns `true` if the response indicates a value change.
    pub const fn is_value_changed(&self) -> bool {
        matches!(self, Response::ValueChanged)
    }

    /// Returns `true` if the response is hovered.
    pub const fn is_hovered(&self) -> bool {
        matches!(self, Response::Hovered)
    }

    /// Returns `true` if the response is scrolled.
    pub const fn is_scrolled(&self) -> bool {
        matches!(self, Response::Scrolled)
    }

    /// Returns `true` if the response is disabled.
    pub const fn is_disabled(&self) -> bool {
        matches!(self, Response::Disabled)
    }

    /// Returns `true` if the response is a user-defined response.
    pub const fn is_user(&self) -> bool {
        matches!(self, Response::User(_))
    }

    /// Returns the user code if the response is a user-defined response.
    pub fn user_code(&self) -> Option<u8> {
        match self {
            Response::User(code) => Some(*code),
            _ => None,
        }
    }

    /// Returns the error if the response is an error.
    pub fn error(&self) -> Option<GuiError> {
        match self {
            Response::Error(e) => Some(*e),
            _ => None,
        }
    }
}

impl Response {
    /// Creates a response from a GUI error.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to convert into a response.
    ///
    /// # Returns
    ///
    /// `Response::Error(error)`.
    pub fn from_error(error: GuiError) -> Response {
        Response::Error(error)
    }

    /// Creates a response based on a change flag.
    ///
    /// # Arguments
    ///
    /// * `change` - `true` if a change occurred, `false` otherwise.
    ///
    /// # Returns
    ///
    /// `Response::ValueChanged` if `change` is `true`, `Response::Idle` otherwise.
    pub fn from_change(change: bool) -> Self {
        if change {
            Response::ValueChanged
        } else {
            Response::Idle
        }
    }
}

/// Converts an interaction event into a response.
///
/// This implementation maps interaction events to widget responses:
/// - `Interaction::None` → `Response::Idle`
/// - `Interaction::Pressed` or `Interaction::Drag` → `Response::Pressed`
/// - `Interaction::Release` → `Response::Trigger`
#[cfg(feature = "interaction")]
impl From<Interaction> for Response {
    fn from(interaction: Interaction) -> Self {
        match interaction {
            Interaction::None => Response::Idle,
            Interaction::Pressed(_) | Interaction::Drag(_) => Response::Pressed,
            Interaction::Release(_) => Response::Released,
            Interaction::Clicked(_) => Response::Clicked,
        }
    }
}

/// Trait for drawable UI widgets.
///
/// This trait defines the interface that all widgets must implement to be
/// rendered within the UI system. Widgets are responsible for their own
/// rendering and can return responses to indicate the result of operations.
///
/// # Type Parameters
///
/// * `DRAW` - The draw target type that implements [`DrawTarget`]
/// * `COL` - The pixel color type that implements [`PixelColor`]
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// struct MyWidget;
///
/// impl<DRAW, COL> Widget<DRAW, COL> for MyWidget
/// where
///     DRAW: DrawTarget<Color = COL>,
///     COL: PixelColor,
/// {
///     fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
///         // Widget rendering logic here
///         Ok(Response::Idle)
///     }
/// }
/// ```
pub trait Widget<DRAW: DrawTarget<Color = COL>, COL: PixelColor> {
    /// Draws the widget to the UI context.
    ///
    /// This method is called by the UI system to render the widget.
    /// The widget can use the provided UI context to access drawing
    /// primitives, style information, and handle interactions.
    ///
    /// # Arguments
    ///
    /// * `ui` - Mutable reference to the UI context.
    ///
    /// # Returns
    ///
    /// A [`GuiResult<Response>`] indicating the result of the draw operation.
    ///
    /// # Errors
    ///
    /// Returns [`GuiError::DrawError`] if drawing fails.
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response>;
}

/// Horizontal alignment options for UI elements.
///
/// This enum defines how content should be aligned horizontally within
/// a container or bounding box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HorizontalAlign {
    /// Align content to the left edge.
    Left,
    /// Align content to the center.
    Center,
    /// Align content to the right edge.
    Right,
    /// Justify content to fill the available width.
    Justify,
}

/// Vertical alignment options for UI elements.
///
/// This enum defines how content should be aligned vertically within
/// a container or bounding box.
#[derive(Clone, Copy, Debug)]
pub enum VerticalAlign {
    /// Align content to the top edge.
    Top,
    /// Align content to the center.
    Center,
    /// Align content to the bottom edge.
    Bottom,
}

/// Combined horizontal and vertical alignment.
///
/// This struct combines horizontal and vertical alignment into a single
/// convenient type for positioning UI elements.
///
/// # Example
///
/// ```rust
/// use matrix_gui::ui::{Align, HorizontalAlign, VerticalAlign};
///
/// // Center alignment
/// let centered = Align(HorizontalAlign::Center, VerticalAlign::Center);
///
/// // Top-left alignment (default)
/// let top_left = Align::default();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Align(pub HorizontalAlign, pub VerticalAlign);

/// Default alignment is top-left.
impl Default for Align {
    fn default() -> Self {
        Align(HorizontalAlign::Left, VerticalAlign::Top)
    }
}

/// Internal drawing helper that wraps a draw target.
///
/// This struct provides a thin wrapper around a draw target to simplify
/// drawing operations within the UI system. It implements both `Drawable`
/// and `DrawTarget` traits for flexibility.
struct Painter<'a, COL: PixelColor, DRAW: DrawTarget<Color = COL>> {
    /// The underlying draw target.
    target: &'a mut DRAW,

    /// The area to draw in, if clipping is enabled.
    #[cfg(feature = "clipped_draw")]
    clipped_area: Option<Rectangle>,
}

#[cfg(not(feature = "clipped_draw"))]
impl<'a, COL: PixelColor, DRAW: DrawTarget<Color = COL>> Painter<'a, COL, DRAW> {
    /// Creates a new painter wrapping the given draw target.
    ///
    /// # Arguments
    ///
    /// * `target` - Mutable reference to the draw target to wrap.
    ///
    /// # Returns
    ///
    /// A new `Painter` instance.
    fn new(target: &'a mut DRAW) -> Self {
        Self { target }
    }

    /// Draws a drawable item to the target.
    ///
    /// # Arguments
    ///
    /// * `item` - Reference to an item implementing `Drawable`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if drawing succeeds, `Err(GuiError::DrawError)` otherwise.
    fn draw(&mut self, item: &impl Drawable<Color = COL>) -> GuiResult<()> {
        item.draw(self.target).map_err(|_| GuiError::DrawError)?;
        Ok(())
    }
}

#[cfg(feature = "clipped_draw")]
impl<'a, COL: PixelColor, DRAW: DrawTarget<Color = COL>> Painter<'a, COL, DRAW> {
    /// Creates a new painter wrapping the given draw target.
    ///
    /// # Arguments
    ///
    /// * `target` - Mutable reference to the draw target to wrap.
    ///
    /// # Returns
    ///
    /// A new `Painter` instance.
    fn new(target: &'a mut DRAW) -> Self {
        Self {
            target,
            clipped_area: None,
        }
    }

    /// Draws a drawable item to the target.
    ///
    /// # Arguments
    ///
    /// * `item` - Reference to an item implementing `Drawable`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if drawing succeeds, `Err(GuiError::DrawError)` otherwise.
    fn draw(&mut self, item: &impl Drawable<Color = COL>) -> GuiResult<()> {
        if let Some(area) = &self.clipped_area {
            let mut clipped = self.target.clipped(area);
            item.draw(&mut clipped).map_err(|_| GuiError::DrawError)?;
        } else {
            item.draw(self.target).map_err(|_| GuiError::DrawError)?;
        }
        Ok(())
    }

    /// Sets the area to draw in, if clipping is enabled.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to draw in.
    ///
    fn set_clipped_area(&mut self, area: Option<Rectangle>) {
        self.clipped_area = area;
    }
}

/// Provides dimensions information for the painter.
impl<COL: PixelColor, DRAW: DrawTarget<Color = COL, Error = ERR>, ERR> Dimensions
    for Painter<'_, COL, DRAW>
{
    /// Returns the bounding box of the underlying draw target.
    ///
    /// # Returns
    ///
    /// A `Rectangle` representing the bounds of the draw target.
    fn bounding_box(&self) -> Rectangle {
        self.target.bounding_box()
    }
}

/// Allows the painter to act as a draw target.
impl<COL: PixelColor, DRAW: DrawTarget<Color = COL, Error = ERR>, ERR> DrawTarget
    for Painter<'_, COL, DRAW>
{
    type Color = COL;
    type Error = ERR;

    /// Draws an iterator of pixels to the target.
    ///
    /// # Arguments
    ///
    /// * `pixels` - An iterator of pixels to draw.
    ///
    /// # Returns
    ///
    /// `Ok(())` if drawing succeeds, `Err(Self::Error)` otherwise.
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.target.draw_iter(pixels)
    }
}

/// Main UI context for managing widgets, drawing, and interactions.
///
/// This struct is the central hub for all UI operations. It manages the
/// draw target, widget states, styling, interactions, and focus management.
/// All widgets are rendered through this context.
///
/// # Type Parameters
///
/// * `'a` - Lifetime of the borrowed resources
/// * `DRAW` - The draw target type implementing [`DrawTarget`]
/// * `COL` - The pixel color type implementing [`PixelColor`]
///
/// # Example
///
/// ```ignore
/// use matrix_gui::prelude::*;
/// use embedded_graphics::primitives::Rectangle;
///
/// // Create UI context with custom bounds
/// let bounds = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
/// let mut ui = Ui::new(&mut display, bounds, &widget_states, &style);
///
/// // Or create fullscreen UI
/// let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
/// ```
pub struct Ui<'a, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    /// The bounding rectangle for the UI area.
    bounds: Rectangle,
    /// Internal painter for drawing operations.
    painter: Painter<'a, COL, DRAW>,
    /// Shared widget state storage.
    widget_states: &'a WidgetStates<'a>,
    /// Current styling configuration.
    style: &'a Style<COL>,
    /// Flag indicating if the background has been cleared.
    cleared: bool,
    /// Current interaction event (if interaction feature is enabled).
    #[cfg(feature = "interaction")]
    interact: Interaction,
    /// Focus management state (if focus feature is enabled).
    #[cfg(feature = "focus")]
    focus: Option<crate::focus::Focus<'a>>,
    /// Debug color for drawing widget bounds (if debug-color feature is enabled).
    #[cfg(feature = "debug-color")]
    debug_color: Option<COL>,
}

impl<DRAW, COL> Ui<'_, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    /// Returns the screen width in pixels.
    ///
    /// # Returns
    ///
    /// The width of the UI bounds in pixels.
    pub const fn get_screen_width(&self) -> u32 {
        self.bounds.size.width
    }

    /// Returns the screen height in pixels.
    ///
    /// # Returns
    ///
    /// The height of the UI bounds in pixels.
    pub const fn get_screen_height(&self) -> u32 {
        self.bounds.size.height
    }

    /// Returns the screen bounding rectangle.
    ///
    /// # Returns
    ///
    /// A `Rectangle` representing the UI bounds.
    pub const fn get_screen_bounds(&self) -> Rectangle {
        self.bounds
    }
}

impl<'a, COL, DRAW> Ui<'a, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    /// Creates a new UI context with the specified bounds.
    ///
    /// # Arguments
    ///
    /// * `drawable` - Mutable reference to the draw target.
    /// * `bounds` - The bounding rectangle for the UI area.
    /// * `widget_states` - Reference to the widget state storage.
    /// * `style` - Reference to the styling configuration.
    ///
    /// # Returns
    ///
    /// A new `Ui` instance.
    pub fn new(
        drawable: &'a mut DRAW,
        bounds: Rectangle,
        widget_states: &'a WidgetStates<'a>,
        style: &'a Style<COL>,
    ) -> Self {
        Self {
            bounds,
            painter: Painter::new(drawable),
            widget_states,
            style,
            cleared: false,
            #[cfg(feature = "interaction")]
            interact: Interaction::None,
            #[cfg(feature = "focus")]
            focus: None,
            #[cfg(feature = "debug-color")]
            debug_color: None,
        }
    }

    /// Creates a new UI context covering the entire draw target.
    ///
    /// This is a convenience method that automatically determines the bounds
    /// from the draw target's bounding box.
    ///
    /// # Arguments
    ///
    /// * `drawable` - Mutable reference to the draw target.
    /// * `widget_states` - Reference to the widget state storage.
    /// * `style` - Reference to the styling configuration.
    ///
    /// # Returns
    ///
    /// A new `Ui` instance with bounds covering the entire draw target.
    pub fn new_fullscreen(
        drawable: &'a mut DRAW,
        widget_states: &'a WidgetStates<'a>,
        style: &'a Style<COL>,
    ) -> Self {
        let bounds = drawable.bounding_box();
        Ui::new(drawable, bounds, widget_states, style)
    }

    /// Adds a widget to the UI and draws it.
    ///
    /// This method calls the widget's `draw` method and returns the response.
    /// If drawing fails, it returns an error response.
    ///
    /// # Arguments
    ///
    /// * `widget` - The widget to add and draw.
    ///
    /// # Returns
    ///
    /// The widget's response, or an error response if drawing fails.
    pub fn add(&mut self, mut widget: impl Widget<DRAW, COL>) -> Response {
        widget.draw(self).unwrap_or_else(Response::from_error)
    }

    /// Adds multiple widgets to the UI and draws them.
    ///
    /// This method iterates over the widgets in the provided iterator and calls
    /// [`add`] for each one.
    ///
    /// # Arguments
    ///
    /// * `widgets` - An iterator that yields widgets to add and draw.
    ///
    /// # Returns
    ///
    /// If any widget returns a non-`Response::Idle`, the method returns that response.
    /// If all widgets return `Response::Idle`, the method returns `Response::Idle`.
    pub fn add_widgets(
        &mut self,
        widgets: impl IntoIterator<Item = impl Widget<DRAW, COL>>,
    ) -> Response {
        for widget in widgets {
            let response = self.add(widget);
            if response != Response::Idle {
                return response;
            }
        }
        Response::Idle
    }

    /// Conditionally draws a widget based on its redraw state.
    ///
    /// This method only calls the drawing closure if the widget needs to be
    /// redrawn (based on its state). This is useful for optimizing rendering
    /// performance by avoiding unnecessary redraws.
    ///
    /// # Type Parameters
    ///
    /// * `ID` - The widget ID type implementing [`WidgetId`]
    /// * `F` - The closure type that performs the drawing
    ///
    /// # Arguments
    ///
    /// * `id` - The widget's identifier.
    /// * `f` - A closure that performs the drawing operation.
    ///
    /// # Returns
    ///
    /// The response from the drawing closure, or `Response::Idle` if no redraw was needed.
    pub fn lazy_draw<F, ID: WidgetId>(&mut self, id: ID, f: F) -> Response
    where
        F: FnOnce(&mut Ui<DRAW, COL>) -> Response,
    {
        if self.widget_states.should_redraw(id) {
            f(self)
        } else {
            Response::Idle
        }
    }

    /// Returns a reference to the current style configuration.
    ///
    /// # Returns
    ///
    /// A reference to the [`Style`] configuration.
    pub fn style(&self) -> &Style<COL> {
        self.style
    }

    /// Sets the current interaction event.
    ///
    /// This method updates the interaction state that widgets can query
    /// to handle user input events.
    ///
    /// # Arguments
    ///
    /// * `interaction` - The interaction event to set.
    #[cfg(feature = "interaction")]
    pub fn interact(&mut self, interaction: Interaction) {
        self.interact = interaction;
    }

    /// Checks if the current interaction is within a widget's region.
    ///
    /// This method determines if the current interaction point (if any) is
    /// contained within the specified region. It also marks the widget as
    /// having received interaction.
    ///
    /// # Type Parameters
    ///
    /// * `ID` - The widget ID type implementing [`WidgetId`]
    ///
    /// # Arguments
    ///
    /// * `region` - The region to check for interaction.
    ///
    /// # Returns
    ///
    /// The interaction if it's within the region, `Interaction::None` otherwise.
    #[cfg(feature = "interaction")]
    pub fn check_interact<ID: WidgetId>(&mut self, region: &Region<ID>) -> Interaction {
        self.widget_states.mark_as_interact(region.id());

        #[cfg(feature = "popup")]
        if self.is_modal_active() {
            return Interaction::None;
        }

        let area = &region.rectangle();
        if self
            .interact
            .get_point()
            .map(|pt| area.contains(pt))
            .unwrap_or(false)
        {
            self.interact
        } else {
            Interaction::None
        }
    }

    /// Sets the area to draw in, if clipping is enabled.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to draw in.
    #[cfg(feature = "clipped_draw")]
    pub fn set_clipped_area(&mut self, area: Option<Rectangle>) {
        self.painter.set_clipped_area(area);
    }
}

impl<COL, DRAW> Ui<'_, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    /// Returns whether the background has been cleared.
    ///
    /// # Returns
    ///
    /// `true` if the background has been cleared, `false` otherwise.
    pub const fn cleared(&self) -> bool {
        self.cleared
    }

    /// Clears the specified area with the background color.
    ///
    /// This method fills the given rectangle with the background color from
    /// the current style. If the background has already been cleared and the
    /// debug-color feature is not enabled, this method returns early.
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangle to clear.
    ///
    /// # Returns
    ///
    /// `Ok(())` if clearing succeeds, `Err(GuiError)` otherwise.
    pub fn clear_area(&mut self, area: &Rectangle) -> GuiResult<()> {
        #[cfg(not(feature = "debug-color"))]
        if self.cleared {
            return Ok(());
        }

        self.clear_area_raw(area, self.style.background_color)?;

        #[cfg(feature = "debug-color")]
        self.draw_widget_bound_box(area);

        Ok(())
    }

    /// Clears the specified area with a custom color.
    ///
    /// This method fills the given rectangle with the specified color.
    /// It uses either the standard `StyledDrawable` trait or the optimized
    /// `fill-rect` feature if enabled.
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangle to clear.
    /// * `color` - The color to fill the area with.
    ///
    /// # Returns
    ///
    /// `Ok(())` if clearing succeeds, `Err(GuiError::DrawError)` otherwise.
    pub fn clear_area_raw(&mut self, area: &Rectangle, color: COL) -> GuiResult<()> {
        #[cfg(not(feature = "fill-rect"))]
        {
            use embedded_graphics::{prelude::Primitive, primitives::PrimitiveStyle};
            self.draw(&area.into_styled(PrimitiveStyle::with_fill(color)))
        }
        #[cfg(feature = "fill-rect")]
        {
            crate::fill_rect::fill_with_color(area, color);
            Ok(())
        }
    }

    /// Clears the entire background area.
    ///
    /// This method fills the entire UI bounds with the background color
    /// from the current style and sets the cleared flag to `true`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if clearing succeeds, `Err(GuiError)` otherwise.
    pub fn clear_background(&mut self) -> GuiResult<()> {
        self.cleared = true;
        self.clear_area_raw(&self.bounds.clone(), self.style.background_color)
    }
}

impl<'a, COL, DRAW> Ui<'a, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    /// Draws a drawable item to the UI.
    ///
    /// This method provides direct access to draw any item that implements
    /// the `Drawable` trait to the UI's draw target.
    ///
    /// # Arguments
    ///
    /// * `item` - Reference to the item to draw.
    ///
    /// # Returns
    ///
    /// `Ok(())` if drawing succeeds, `Err(GuiError::DrawError)` otherwise.
    pub fn draw(&mut self, item: &impl Drawable<Color = COL>) -> GuiResult<()> {
        #[cfg(feature = "debug-color")]
        {
            self.update_debug_counter();
        }

        self.painter.draw(item)
    }
}

impl<'a, COL, DRAW> Ui<'a, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    /// Retrieves the render state for a widget.
    ///
    /// This method returns a reference to the render state for the widget
    /// with the specified ID. The render state contains information about
    /// whether the widget needs to be redrawn.
    ///
    /// # Type Parameters
    ///
    /// * `ID` - The widget ID type implementing [`WidgetId`]
    ///
    /// # Arguments
    ///
    /// * `widget_id` - The identifier of the widget.
    ///
    /// # Returns
    ///
    /// A reference to the [`RenderState`] if the widget exists,
    /// `Err(GuiError::InvalidWidgetId)` otherwise.
    pub fn get_widget_state<ID: WidgetId>(&self, widget_id: ID) -> GuiResult<&RenderState> {
        self.widget_states.get_state(widget_id)
    }

    /// Retrieves the internal flag for a widget.
    ///
    /// This method returns the internal flag for the widget with the specified ID.
    /// The internal flag is a boolean value that is used to track the state of the widget.
    ///
    /// # Type Parameters
    ///
    /// * `ID` - The widget ID type implementing [`WidgetId`]
    ///
    /// # Arguments
    ///
    /// * `widget_id` - The identifier of the widget.
    ///
    /// # Returns
    ///
    /// The internal flag as a `bool` value.
    pub fn get_widget_internal_flag<ID: WidgetId>(&self, widget_id: ID) -> GuiResult<bool> {
        self.widget_states.get_internal_flag(widget_id)
    }

    /// Sets the internal flag for a widget.
    ///
    /// This method sets the internal flag for the widget with the specified ID.
    /// The internal flag is a boolean value that is used to track the state of the widget.
    ///
    /// # Type Parameters
    ///
    /// * `ID` - The widget ID type implementing [`WidgetId`]
    ///
    /// # Arguments
    ///
    /// * `widget_id` - The identifier of the widget.
    /// * `value` - The new value for the internal flag.
    ///
    /// # Returns
    ///
    /// `Ok(())` if setting the internal flag succeeds, `Err(GuiError)` otherwise.
    pub fn set_widget_internal_flag<ID: WidgetId>(
        &self,
        widget_id: ID,
        value: bool,
    ) -> GuiResult<()> {
        self.widget_states.set_internal_flag(widget_id, value)
    }

    /// Retrieves the animation status for a widget.
    ///
    /// This method returns the current animation status for the widget with the
    /// specified ID. The animation status is a value that tracks the progress of
    /// the animation.
    ///
    /// # Arguments
    ///
    /// * `anim_id` - The identifier of the animation.
    ///
    /// # Returns
    ///
    /// The current animation status as an `Option<i32>` value.
    #[cfg(feature = "animation")]
    pub fn take_anim_status(&self, anim_id: crate::prelude::AnimId) -> GuiResult<Option<i32>> {
        self.widget_states.take_anim_status(anim_id)
    }

    /// Retrieves the animation status for a widget.
    ///
    /// This method returns the current animation status for the widget with the
    /// specified ID. The animation status is a value that tracks the progress of
    /// the animation.
    ///
    /// # Arguments
    ///
    /// * `anim_id` - The identifier of the animation.
    ///
    /// # Returns
    ///
    /// The current animation status as an `Option<i32>` value.
    #[cfg(feature = "animation")]
    pub fn get_anim_status(&self, anim_id: crate::prelude::AnimId) -> GuiResult<Option<i32>> {
        self.widget_states.get_anim_status(anim_id)
    }

    /// Forces a widget to be redrawn on the next frame.
    ///
    /// This method marks the widget with the specified ID as needing to be
    /// redrawn, regardless of its current state.
    ///
    /// # Type Parameters
    ///
    /// * `ID` - The widget ID type implementing [`WidgetId`]
    ///
    /// # Arguments
    ///
    /// * `widget_id` - The identifier of the widget to force redraw.
    pub fn force_redraw<ID: WidgetId>(&self, widget_id: ID) {
        self.widget_states.force_redraw(widget_id);
    }

    /// Forces all widgets to be redrawn on the next frame.
    ///
    /// This method marks all widgets as needing to be redrawn, regardless of
    /// their current state. This is useful for refreshing the entire UI.
    pub fn force_redraw_all(&self) {
        self.widget_states.force_redraw_all();
    }
}

/// Focus management implementation (requires "focus" feature).
///
/// This implementation provides methods for managing keyboard focus
/// and navigation between widgets.
#[cfg(feature = "focus")]
impl<'a, COL, DRAW> Ui<'a, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    /// Sets the focus state for the UI.
    ///
    /// This method associates a focus state tracker with the UI, enabling
    /// keyboard navigation and focus management.
    ///
    /// # Type Parameters
    ///
    /// * `N` - The maximum number of focusable widgets
    ///
    /// # Arguments
    ///
    /// * `focus_state` - Mutable reference to the focus state tracker.
    pub fn set_focus_state<const N: usize>(
        &mut self,
        focus_state: &'a mut crate::focus::FocusState<N>,
    ) {
        self.focus = Some(crate::focus::Focus::new(focus_state));
    }

    /// Checks if a widget is focused.
    ///
    /// This method determines if the widget with the specified ID is currently
    /// focused. It also handles trigger events and updates the focus state.
    ///
    /// # Type Parameters
    ///
    /// * `ID` - The widget ID type implementing [`WidgetId`]
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the widget to check.
    ///
    /// # Returns
    ///
    /// A [`crate::focus::Focused`] enum indicating the focus status.
    pub fn check_focused<ID: WidgetId>(&mut self, id: ID) -> crate::focus::Focused {
        use crate::focus::Focused;

        #[cfg(feature = "popup")]
        if self.is_modal_active() {
            return Focused::No;
        }

        if let Some(focus) = self.focus.as_mut() {
            self.widget_states.mark_as_interact(id);

            if let Some(trigger_id) = focus.tracker.trigger
                && trigger_id == id.id() as u16
            {
                focus.tracker.trigger = None;
                return Focused::Trigger;
            } else if focus.register_focus(id.id()) && focus.is_focused(id.id()) {
                return Focused::Yes;
            }
        }

        Focused::No
    }
}

#[cfg(feature = "popup")]
impl<'a, COL, DRAW> Ui<'a, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    pub const fn is_modal_active(&self) -> bool {
        self.widget_states.is_modal_active()
    }

    pub fn set_modal_active(&self, active: bool) {
        self.widget_states.set_modal_active(active);
    }
}

/// Debug drawing implementation (requires "debug-color" feature).
///
/// This implementation provides methods for drawing debug visualizations
/// of widget bounds and UI areas.
#[cfg(feature = "debug-color")]
impl<COL, DRAW> Ui<'_, DRAW, COL>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
{
    /// Draws a debug border around the UI bounds.
    ///
    /// This method draws a 1-pixel wide border around the entire UI area
    /// using the specified color. This is useful for debugging UI layout.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to use for the border.
    ///
    /// # Returns
    ///
    /// `Ok(())` if drawing succeeds, `Err(GuiError::DrawError)` otherwise.
    pub fn draw_bounds_debug(&mut self, color: COL) -> GuiResult<()> {
        use embedded_graphics::primitives::{PrimitiveStyleBuilder, StyledDrawable};

        let bounds = self.bounds;
        bounds
            .draw_styled(
                &PrimitiveStyleBuilder::new()
                    .stroke_color(color)
                    .stroke_width(1)
                    .build(),
                &mut self.painter,
            )
            .map_err(|_| GuiError::DrawError)
    }

    /// Enables debug drawing for widget bounds.
    ///
    /// This method sets a color that will be used to draw borders around
    /// widget areas when they are cleared. This is useful for visualizing
    /// widget boundaries during development.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to use for widget bound boxes.
    pub fn draw_widget_bounds_debug(&mut self, color: COL) {
        self.debug_color = Some(color);
    }

    /// Draws a debug border around a widget area.
    ///
    /// This method draws a 1-pixel wide border around the specified area
    /// if debug drawing is enabled. This is called automatically when
    /// clearing widget areas.
    ///
    /// # Arguments
    ///
    /// * `area` - The rectangle to draw a border around.
    pub fn draw_widget_bound_box(&mut self, area: &Rectangle) {
        if let Some(debug_color) = self.debug_color {
            use embedded_graphics::primitives::{PrimitiveStyleBuilder, StyledDrawable};

            area.draw_styled(
                &PrimitiveStyleBuilder::new()
                    .stroke_color(debug_color)
                    .stroke_width(1)
                    .build(),
                &mut self.painter,
            )
            .ok();
        }
    }

    /// Returns the current debug counter value.
    ///
    /// This method is useful for debugging and tracking the number of draw calls
    /// made by the UI context.
    pub fn get_debug_counter(&self) -> u32 {
        DBG_CNT.load(core::sync::atomic::Ordering::Relaxed)
    }

    /// Updates the debug counter.
    fn update_debug_counter(&mut self) {
        DBG_CNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    }
}
