//! Region and layout management module for the matrix_gui framework.
//!
//! This module provides utilities for defining and managing widget regions within
//! the GUI. It includes:
//!
//! - [`Region`]: A struct representing a rectangular area with an associated widget ID
//! - Layout functions for arranging widgets in grid patterns
//! - Macros for generating region IDs and layouts at compile time
//!
//! # Core Concepts
//!
//! ## Regions
//!
//! A [`Region`] represents a rectangular area on the screen where a widget can be
//! placed. Each region has an associated widget ID that uniquely identifies it.
//!
//! ## Layouts
//!
//! The module provides several layout functions for arranging regions in grids:
//!
//! - **Row-major**: Regions are arranged left-to-right, top-to-bottom
//! - **Column-major**: Regions are arranged top-to-bottom, left-to-right
//! - Both runtime and compile-time variants are available
//!
//! # Macros
//!
//! The module provides several macros for generating region layouts:
//!
//! - [`free_form_region!`]: Define regions with custom positions and sizes
//! - [`region_id!`]: Generate region ID enums
//! - [`grid_layout_row_major!`]: Create row-major grid layouts
//! - [`grid_layout_column_major!`]: Create column-major grid layouts
//! - [`grid_layout_row_major_with_start!`]: Create grid layouts with custom start ID
//! - [`grid_layout_column_major_with_start!`]: Create grid layouts with custom start ID

use embedded_graphics::{
    geometry::AnchorPoint,
    prelude::{Point, Size},
    primitives::Rectangle,
};

use crate::{
    prelude::{DeltaResize, LwPoint, LwRectangle, LwSize},
    widget_state::WidgetId,
};

/// A region representing a rectangular area with an associated widget ID.
///
/// This struct is used to define the position and size of widgets within the GUI.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Region<ID> {
    /// The widget ID associated with this region.
    id: ID,
    /// The rectangular area of this region.
    area: LwRectangle<i16, u16>,
}

impl<ID: WidgetId> Region<ID> {
    /// Creates a new region with the specified ID and dimensions.
    ///
    /// # Arguments
    ///
    /// * `id` - The widget ID for this region
    /// * `x` - The x-coordinate of the top-left corner
    /// * `y` - The y-coordinate of the top-left corner
    /// * `w` - The width of the region
    /// * `h` - The height of the region
    pub const fn new(id: ID, x: i16, y: i16, w: u16, h: u16) -> Self {
        let area = LwRectangle::new(LwPoint::new(x, y), LwSize::new(w, h));
        Region { id, area }
    }

    /// Creates a new region with the specified ID and area.
    pub const fn new_with_area(id: ID, area: LwRectangle<i16, u16>) -> Self {
        Region { id, area }
    }

    /// Creates a new region with zero dimensions at the origin.
    ///
    /// # Arguments
    ///
    /// * `id` - The widget ID for this region
    pub const fn zero(id: ID) -> Self {
        Self::new(id, 0, 0, 0, 0)
    }

    /// Returns the widget ID associated with this region.
    pub const fn id(&self) -> ID {
        self.id
    }

    /// Returns the area of the region.
    pub const fn area(&self) -> LwRectangle<i16, u16> {
        self.area
    }

    /// Replace the widget ID of the region.
    pub const fn replace_id(&self, id: ID) -> Self {
        Self {
            id,
            area: self.area,
        }
    }

    /// Returns the top-left corner of the region as a `Point`.
    pub fn top_left(&self) -> Point {
        self.area.top_left.into()
    }

    /// Returns the size of the region as a `Size`.
    pub fn size(&self) -> Size {
        self.area.size.into()
    }

    /// Returns the region as a `Rectangle`.
    pub fn rectangle(&self) -> Rectangle {
        self.area.into()
    }

    /// Returns the x-coordinate of the top-left corner as an `i32`.
    pub const fn x(&self) -> i32 {
        self.area.top_left.x as i32
    }

    /// Returns the y-coordinate of the top-left corner as an `i32`.
    pub const fn y(&self) -> i32 {
        self.area.top_left.y as i32
    }

    /// Returns the width of the region as a `u32`.
    pub const fn width(&self) -> u32 {
        self.area.size.width as u32
    }

    /// Returns the height of the region as a `u32`.
    pub const fn height(&self) -> u32 {
        self.area.size.height as u32
    }

    /// Returns a new region resized according to the specified delta.
    ///
    /// # Arguments
    ///
    /// * `delta` - The resize delta to use for resizing
    pub fn delta_resize(&self, delta: DeltaResize) -> Region<ID> {
        let area = self.area.delta_resize(delta);

        Region { id: self.id, area }
    }

    /// Moves the region by the specified amount.
    ///
    /// # Arguments
    ///
    /// * `dx` - The x-direction offset
    /// * `dy` - The y-direction offset
    ///
    /// # Returns
    ///
    /// A new `Region` instance with the moved position.
    pub fn move_by(&self, dx: i16, dy: i16) -> Self {
        Self {
            id: self.id,
            area: self.area.move_by(dx, dy),
        }
    }

    /// Returns a new region resized according to the specified width and height.
    ///
    /// # Arguments
    ///
    /// * `width` - The new width of the region
    /// * `height` - The new height of the region
    /// * `anchor_point` - The anchor point to use for resizing
    ///
    /// # Returns
    ///
    /// A new `Region` instance with the resized dimensions.
    pub fn resized(&self, width: u16, height: u16, anchor_point: AnchorPoint) -> Self {
        Self {
            id: self.id,
            area: self.area.resized(width, height, anchor_point),
        }
    }
}

/// Creates a free-form region definition with multiple regions.
///
/// This macro generates:
/// - A `RegionID` enum with variants for each region
/// - A `REGIONID_COUNT` constant representing the number of regions
/// - Constants for each region as uppercase identifiers
///
/// # Usage
///
/// ```rust
/// use matrix_gui::free_form_region;
/// use matrix_gui::prelude::*;
///
/// free_form_region!(
///     RegionID,
///     (button, 10, 10, 100, 40),
///     (label, 10, 60, 100, 20),
/// );
/// ```
///
/// This will generate:
/// - A `RegionID` enum with variants `Button` and `Label`
/// - A `REGIONID_COUNT` constant set to 2
/// - Constants `BUTTON` and `LABEL` pointing to the respective regions
///
/// Note: If `width` or `height` is 0, the region constant will not be generated.
#[macro_export]
macro_rules! free_form_region {
    (
        $enum_name:ident,
        $(($name:ident $(, $x:expr, $y:expr, $width:expr, $height:expr)?)),+
        $(,)?
    ) => {
        paste::paste! {
            pub const [<$enum_name:upper _COUNT>]: usize = enum_iterator::cardinality::<[<$enum_name:camel>]>();

            #[derive(Debug, PartialEq, Clone, Copy, Default, enum_iterator::Sequence)]
            #[repr(u16)]
            pub enum [<$enum_name:camel>] {
                #[default]
                $([<$name:camel>]),+
            }

            impl $crate::widget_state::WidgetId for [<$enum_name:camel>] {
                fn id(&self) -> usize {
                    *self as usize
                }
            }
        }

        $(
            matrix_gui::free_form_region_gen_region!(
                $enum_name, $name, $($x, $y, $width, $height)?
            );
        )+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! free_form_region_gen_region {
    ($enum_name:ident, $name:ident,) => {};
    ($enum_name:ident, $name:ident, $x:expr, $y:expr, $width:expr, $height:expr) => {
        paste::paste! {
            pub const [<$name:upper>]: &$crate::region::Region<[<$enum_name:camel>]> = &$crate::region::Region::new(
                [<$enum_name:camel>]::[<$name:camel>],
                $x, $y, $width, $height
            );
        }
    };
}

/// Creates a region ID enum starting from 0.
///
/// This macro generates an enum with the specified name and variants, implementing
/// the `WidgetId` trait. It calls `region_id_with_start!` with a start value of 0.
///
/// # Usage
///
/// ```rust
/// use matrix_gui::region_id;
/// use matrix_gui::prelude::*;
///
/// region_id!(Button, [Ok, Cancel, Reset]);
/// ```
///
/// This will generate a `Button` enum with variants `Ok`, `Cancel`, and `Reset`
/// starting from 0.
#[macro_export]
macro_rules! region_id {
    ($name:ident, [$first:ident, $($rest:ident),+ $(,)?]) => {
        matrix_gui::region_id_with_start!($name, 0, [$first, $($rest),+]);
    };
}

/// Creates a region ID enum with a specified starting value.
///
/// This macro generates an enum with the specified name and variants, implementing
/// the `WidgetId` trait. It also generates a constant for the count of variants
/// and an `all()` method that returns all enum variants.
///
/// # Usage
///
/// ```rust
/// use matrix_gui::region_id_with_start;
/// use matrix_gui::prelude::*;
///
/// region_id_with_start!(Button, 5, [Ok, Cancel, Reset]);
/// ```
///
/// This will generate a `Button` enum with variants `Ok` (5), `Cancel` (6), and `Reset` (7).
#[macro_export]
macro_rules! region_id_with_start {
    ($name:ident, $start:expr, [$first:ident, $($rest:ident),+ $(,)?]) => {
        paste::paste! {
            pub const [<$name:upper _COUNT>] : usize = enum_iterator::cardinality::<[<$name:camel>]>();
            #[derive(Debug, PartialEq, Clone, Copy, Default, enum_iterator::Sequence)]
            #[repr(u16)]
            pub enum [<$name:camel>] {
                #[default]
                [<$first:camel>] = $start as u16,
                $([<$rest:camel>]),+
            }

            impl $crate::widget_state::WidgetId for [<$name:camel>] {
                fn id(&self) -> usize {
                    *self as usize
                }
            }

            impl [<$name:camel>] {
                /// Returns an array containing all enum variants.
                pub const fn all() -> [[<$name:camel>]; [<$name:upper _COUNT>]] {
                [
                    [<$name:camel>]::[<$first:camel>],
                    $([<$name:camel>]::[<$rest:camel>]),+
                ]
                }
            }
        }
    };

    ($name:ident, $start:expr, [$only:ident]) => {
        paste::paste! {
            #[repr(u16)]
            pub enum [<$name:camel>] {
                [<$only:camel>] = $start
            }
        }
    };
}

/// Creates a column-major grid layout in place.
///
/// This function fills the provided region slice with regions arranged in a grid layout,
/// ordered by columns (filling each column top to bottom before moving to the next column).
///
/// # Arguments
///
/// * `start_id` - The starting widget ID
/// * `area` - The bounding rectangle for the entire grid
/// * `rows` - The number of rows in the grid
/// * `cols` - The number of columns in the grid
/// * `gap` - The gap between adjacent regions
/// * `regions` - The slice of regions to fill
pub fn grid_layout_column_major_mut<ID: WidgetId>(
    start_id: ID,
    area: &Rectangle,
    rows: u32,
    cols: u32,
    gap: u32,
    regions: &mut [Region<ID>],
) {
    let col_width = (area.size.width - gap * (cols.max(1) - 1)) / cols.max(1);
    let row_height = (area.size.height - gap * (rows.max(1) - 1)) / rows.max(1);
    let region_size = Size::new(col_width, row_height);
    let mut region_top_left = area.top_left;
    let mut rows_count = 0;
    let mut curr_id = start_id;

    for region in regions {
        region.id = curr_id;
        region.area = Rectangle::new(region_top_left, region_size).into();

        rows_count += 1;
        if rows_count < rows {
            region_top_left.y += row_height as i32 + gap as i32;
        } else {
            if region_top_left.x - area.top_left.x >= area.size.width as i32 {
                break;
            }
            region_top_left.x += col_width as i32 + gap as i32;
            region_top_left.y = area.top_left.y;
            rows_count = 0;
        }

        if let Some(id) = curr_id.next() {
            curr_id = id;
        }
    }
}

/// Creates a column-major grid layout and returns it as an array.
///
/// This function creates an array of regions arranged in a grid layout,
/// ordered by columns (filling each column top to bottom before moving to the next column).
///
/// # Arguments
///
/// * `start_id` - The starting widget ID
/// * `area` - The bounding rectangle for the entire grid
/// * `rows` - The number of rows in the grid
/// * `cols` - The number of columns in the grid
/// * `gap` - The gap between adjacent regions
///
/// # Returns
///
/// An array of regions arranged in a column-major grid layout
pub fn grid_layout_column_major<ID: WidgetId, const N: usize>(
    start_id: ID,
    area: &Rectangle,
    rows: u32,
    cols: u32,
    gap: u32,
) -> [Region<ID>; N] {
    let mut regions = [Region::zero(start_id); N];
    grid_layout_column_major_mut(start_id, area, rows, cols, gap, &mut regions);

    regions
}

/// Creates a row-major grid layout in place.
///
/// This function fills the provided region slice with regions arranged in a grid layout,
/// ordered by rows (filling each row left to right before moving to the next row).
///
/// # Arguments
///
/// * `start_id` - The starting widget ID
/// * `area` - The bounding rectangle for the entire grid
/// * `rows` - The number of rows in the grid
/// * `cols` - The number of columns in the grid
/// * `gap` - The gap between adjacent regions
/// * `regions` - The slice of regions to fill
pub fn grid_layout_row_major_mut<ID: WidgetId>(
    start_id: ID,
    area: &Rectangle,
    rows: u32,
    cols: u32,
    gap: u32,
    regions: &mut [Region<ID>],
) {
    let col_width = (area.size.width - gap * (cols.max(1) - 1)) / cols.max(1);
    let row_height = (area.size.height - gap * (rows.max(1) - 1)) / rows.max(1);
    let region_size = Size::new(col_width, row_height);
    let mut region_top_left = area.top_left;
    let mut cols_count = 0;
    let mut curr_id = start_id;

    for region in regions {
        region.id = curr_id;
        region.area = Rectangle::new(region_top_left, region_size).into();

        cols_count += 1;
        if cols_count < cols {
            region_top_left.x += col_width as i32 + gap as i32;
        } else {
            if region_top_left.y - area.top_left.y >= area.size.height as i32 {
                break;
            }
            region_top_left.y += row_height as i32 + gap as i32;
            region_top_left.x = area.top_left.x;
            cols_count = 0;
        }

        if let Some(id) = curr_id.next() {
            curr_id = id;
        }
    }
}

/// Creates a row-major grid layout and returns it as an array.
///
/// This function creates an array of regions arranged in a grid layout,
/// ordered by rows (filling each row left to right before moving to the next row).
///
/// # Arguments
///
/// * `start_id` - The starting widget ID
/// * `area` - The bounding rectangle for the entire grid
/// * `rows` - The number of rows in the grid
/// * `cols` - The number of columns in the grid
/// * `gap` - The gap between adjacent regions
///
/// # Returns
///
/// An array of regions arranged in a row-major grid layout
pub fn grid_layout_row_major<ID: WidgetId, const N: usize>(
    start_id: ID,
    area: &Rectangle,
    rows: u32,
    cols: u32,
    gap: u32,
) -> [Region<ID>; N] {
    let mut regions = [Region::zero(start_id); N];
    grid_layout_row_major_mut(start_id, area, rows, cols, gap, &mut regions);

    regions
}

/// Creates a column-major grid layout at compile time.
///
/// This const function creates an array of regions arranged in a grid layout,
/// ordered by columns (filling each column top to bottom before moving to the next column).
///
/// # Arguments
///
/// * `id_list` - An array of widget IDs to use for the regions
/// * `area` - The bounding rectangle for the entire grid
/// * `rows` - The number of rows in the grid
/// * `cols` - The number of columns in the grid
/// * `gap` - The gap between adjacent regions
///
/// # Returns
///
/// An array of regions arranged in a column-major grid layout
pub const fn const_grid_layout_column_major<ID: WidgetId, const N: usize>(
    id_list: &[ID; N],
    area: &Rectangle,
    rows: u32,
    cols: u32,
    gap: u32,
) -> [Region<ID>; N] {
    assert!(N > 0, "id_list must not be empty");
    let mut regions = [Region::zero(id_list[0]); N];

    let cols_max = if cols > 0 { cols } else { 1 };
    let rows_max = if rows > 0 { rows } else { 1 };
    let col_width = ((area.size.width - gap * (cols_max - 1)) / cols_max) as u16;
    let row_height = ((area.size.height - gap * (rows_max - 1)) / rows_max) as u16;

    let mut x = area.top_left.x;
    let mut y = area.top_left.y;
    let mut rows_count = 0;

    let mut i = 0;
    while i < N {
        regions[i] = Region::new(id_list[i], x as i16, y as i16, col_width, row_height);

        rows_count += 1;
        if rows_count < rows {
            y += row_height as i32 + gap as i32;
        } else {
            if x - area.top_left.x >= area.size.width as i32 {
                break;
            }
            x += col_width as i32 + gap as i32;
            y = area.top_left.y;
            rows_count = 0;
        }

        i += 1;
    }

    regions
}

/// Creates a row-major grid layout at compile time.
///
/// This const function creates an array of regions arranged in a grid layout,
/// ordered by rows (filling each row left to right before moving to the next row).
///
/// # Arguments
///
/// * `id_list` - An array of widget IDs to use for the regions
/// * `area` - The bounding rectangle for the entire grid
/// * `rows` - The number of rows in the grid
/// * `cols` - The number of columns in the grid
/// * `gap` - The gap between adjacent regions
///
/// # Returns
///
/// An array of regions arranged in a row-major grid layout
pub const fn const_grid_layout_row_major<ID: WidgetId, const N: usize>(
    id_list: &[ID; N],
    area: &Rectangle,
    rows: u32,
    cols: u32,
    gap: u32,
) -> [Region<ID>; N] {
    assert!(N > 0, "id_list must not be empty");
    let mut regions = [Region::zero(id_list[0]); N];

    let cols_max = if cols > 0 { cols } else { 1 };
    let rows_max = if rows > 0 { rows } else { 1 };
    let col_width = ((area.size.width - gap * (cols_max - 1)) / cols_max) as u16;
    let row_height = ((area.size.height - gap * (rows_max - 1)) / rows_max) as u16;

    let mut x = area.top_left.x;
    let mut y = area.top_left.y;
    let mut cols_count = 0;

    let mut i = 0;
    while i < N {
        regions[i] = Region::new(id_list[i], x as i16, y as i16, col_width, row_height);

        cols_count += 1;
        if cols_count < cols {
            x += col_width as i32 + gap as i32;
        } else {
            if y - area.top_left.y >= area.size.height as i32 {
                break;
            }
            y += row_height as i32 + gap as i32;
            x = area.top_left.x;
            cols_count = 0;
        }

        i += 1;
    }

    regions
}

/// Creates constant variables for a grid layout.
///
/// This macro generates:
/// - A constant for the layout area
/// - A constant for the grid layout array
/// - Constants for each region in the grid
///
/// # Arguments
///
/// * `name` - The base name for the generated constants and enum
/// * `base_idx` - The base index to use for region constants
/// * `(x, y, width, height)` - The area for the grid layout
/// * `(rows, cols, gap)` - The grid dimensions and gap
/// * `[first, rest...]` - The region names
/// * `layout_fn` - The layout function to use (column or row)
/// * `suffix` - The suffix to use for the layout array constant
#[macro_export]
macro_rules! grid_layout_const_var {
    ($name:ident, $base_idx:expr, ($x:expr, $y:expr, $width:expr, $height:expr),
    ($rows:expr, $cols:expr, $gap:expr), [$first:ident, $($rest:ident),+ $(,)?], $layout_fn:ident, $suffix:ident) => {
        paste::paste! {
            pub const [<$name:upper _AREA>]: Rectangle =
                Rectangle::new(Point::new($x, $y), Size::new($width, $height));
            pub const [<$name:upper _ $suffix>]: [Region<[<$name:camel>]>; [<$name:upper _COUNT>] ] =
                matrix_gui::region::[<const_grid_layout_ $layout_fn _major>](&[<$name:camel>]::all(),
                                    &[<$name:upper _AREA>], $rows, $cols, $gap);
        }
        paste::paste! {
            pub const [<$first:upper>]: &Region<[<$name:camel>]> =
                &[<$name:upper _ $suffix>][[<$name:camel>]::[<$first:camel>] as usize - $base_idx];
        }
        $(paste::paste! {
            pub const [<$rest:upper>]: &Region<[<$name:camel>]> =
                &[<$name:upper _ $suffix>][[<$name:camel>]::[<$rest:camel>] as usize - $base_idx];
        })+
    }
}

/// Creates a column-major grid layout with region IDs starting from 0.
///
/// This macro generates:
/// - A region ID enum with variants for each region
/// - Constants for each region in the grid
/// - A constant for the grid layout array
///
/// # Usage
///
/// ```rust
/// use matrix_gui::grid_layout_column_major;
/// use matrix_gui::prelude::*;
///
/// grid_layout_column_major!(
///     Button, (10, 10, 200, 100),
///     (2, 2, 10), [Ok, Cancel, Reset, Exit]
/// );
/// ```
#[macro_export]
macro_rules! grid_layout_column_major {
    ($name:ident, ($x:expr, $y:expr, $width:expr, $height:expr),
        ($rows:expr, $cols:expr, $gap:expr), [$first:ident, $($rest:ident),+ $(,)?]) => {
        matrix_gui::region_id!($name, [$first, $($rest),+]);
        matrix_gui::grid_layout_const_var!($name, 0, ($x, $y, $width, $height),
                                        ($rows, $cols, $gap), [$first, $($rest),+], column, GLCM);
    }
}

/// Creates a row-major grid layout with region IDs starting from 0.
///
/// This macro generates:
/// - A region ID enum with variants for each region
/// - Constants for each region in the grid
/// - A constant for the grid layout array
///
/// # Usage
///
/// ```rust
/// use matrix_gui::grid_layout_row_major;
/// use matrix_gui::prelude::*;
///
/// grid_layout_row_major!(
///     Button, (10, 10, 200, 100),
///     (2, 2, 10), [Ok, Cancel, Reset, Exit]
/// );
/// ```
#[macro_export]
macro_rules! grid_layout_row_major {
    ($name:ident, ($x:expr, $y:expr, $width:expr, $height:expr),
        ($rows:expr, $cols:expr, $gap:expr), [$first:ident, $($rest:ident),+ $(,)?]) => {
        matrix_gui::region_id!($name, [$first, $($rest),+]);
        matrix_gui::grid_layout_const_var!($name, 0, ($x, $y, $width, $height),
                                        ($rows, $cols, $gap), [$first, $($rest),+], row, GLRM);
    }
}

/// Creates a column-major grid layout with region IDs starting from a specified value.
///
/// This macro generates:
/// - A region ID enum with variants for each region, starting from the specified value
/// - Constants for each region in the grid
/// - A constant for the grid layout array
///
/// # Usage
///
/// ```rust
/// use matrix_gui::grid_layout_column_major_with_start;
/// use matrix_gui::prelude::*;
///
/// grid_layout_column_major_with_start!(
///     Button, 5, (10, 10, 200, 100),
///     (2, 2, 10), [Ok, Cancel, Reset, Exit]
/// );
/// ```
#[macro_export]
macro_rules! grid_layout_column_major_with_start {
    ($name:ident, $start:expr, ($x:expr, $y:expr, $width:expr, $height:expr),
        ($rows:expr, $cols:expr, $gap:expr), [$first:ident, $($rest:ident),+ $(,)?]) => {
        matrix_gui::region_id_with_start!($name, $start, [$first, $($rest),+]);
        matrix_gui::grid_layout_const_var!($name, $start, ($x, $y, $width, $height),
                                        ($rows, $cols, $gap), [$first, $($rest),+], column, GLCM);
    }
}

/// Creates a row-major grid layout with region IDs starting from a specified value.
///
/// This macro generates:
/// - A region ID enum with variants for each region, starting from the specified value
/// - Constants for each region in the grid
/// - A constant for the grid layout array
///
/// # Usage
///
/// ```rust
/// use matrix_gui::grid_layout_row_major_with_start;
/// use matrix_gui::prelude::*;
///
/// grid_layout_row_major_with_start!(
///     Button, 5, (10, 10, 200, 100),
///     (2, 2, 10), [Ok, Cancel, Reset, Exit]
/// );
/// ```
#[macro_export]
macro_rules! grid_layout_row_major_with_start {
    ($name:ident, $start:expr, ($x:expr, $y:expr, $width:expr, $height:expr),
        ($rows:expr, $cols:expr, $gap:expr), [$first:ident, $($rest:ident),+ $(,)?]) => {
        matrix_gui::region_id_with_start!($name, $start, [$first, $($rest),+]);
        matrix_gui::grid_layout_const_var!($name, $start, ($x, $y, $width, $height),
                                        ($rows, $cols, $gap), [$first, $($rest),+], row, GLRM);
    }
}
