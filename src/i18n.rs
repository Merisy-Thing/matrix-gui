//! Internationalization (i18n) module for matrix_gui.
//!
//! This module provides support for multiple languages in the GUI framework. It allows
//! runtime language switching and provides macros for defining internationalized strings
//! and toggle types.
//!
//! # Features
//!
//! - Runtime language selection using atomic operations
//! - Support for multiple languages (extensible)
//! - Macros for defining internationalized strings
//! - Toggle types with language-aware string representations
//!
//! # Example
//!
//! ```rust
//! use matrix_gui::prelude::*;
//!
//! // Define internationalized strings
//! matrix_gui::i18n_string!(TIP_ON, "开", "ON");
//! matrix_gui::i18n_string!(TIP_OFF, "关", "OFF");
//!
//! // Define a toggle type
//! matrix_gui::i18n_toggle_type!(TipOnOff, TIP_ON, TIP_OFF);
//!
//! // Switch language
//! Languages::switch_language();
//! ```

use core::{
    ops::Deref,
    sync::atomic::{AtomicU8, Ordering},
};

/// Current language selection stored atomically.
///
/// This static variable holds the index of the currently selected language.
/// It uses atomic operations to ensure thread-safe access without locks.
static SELECTION: AtomicU8 = AtomicU8::new(Languages::LocalLang as u8);

/// Total number of supported languages.
///
/// This constant is computed at compile time using the `enum_iterator` crate
/// to determine the cardinality of the `Languages` enum.
const LANG_COUNT: usize = enum_iterator::cardinality::<Languages>();

/// Supported languages in the GUI framework.
///
/// This enum defines the available languages for internationalization.
/// New languages can be added by extending this enum.
#[derive(Debug, Copy, Clone, PartialEq, enum_iterator::Sequence)]
pub enum Languages {
    /// Local language (default, typically Chinese)
    LocalLang = 0,
    /// English language
    English = 1,
}

/// Conversion from `Languages` to a static string slice.
///
/// This implementation provides a human-readable name for each language.
/// For `LocalLang`, it attempts to use the `LOCAL_LANG_STRING` environment variable
/// at compile time, falling back to "中文" if not set.
impl From<Languages> for &'static str {
    fn from(lang: Languages) -> Self {
        match lang {
            Languages::LocalLang => option_env!("LOCAL_LANG_STRING").unwrap_or("中文"),
            Languages::English => "English",
        }
    }
}

/// Conversion from a byte index to `Languages`.
///
/// This implementation allows converting a language index (as stored in the
/// atomic `SELECTION` variable) back to a `Languages` enum variant.
impl From<u8> for Languages {
    fn from(lang: u8) -> Self {
        match lang {
            0 => Self::LocalLang,
            _ => Self::English,
        }
    }
}

impl Languages {
    /// Returns the next language in the sequence.
    ///
    /// This method cycles through the available languages, returning the next
    /// language after the current one. It wraps around to the first language
    /// when reaching the end.
    ///
    /// # Returns
    ///
    /// The next language in the sequence.
    pub fn next(self) -> Self {
        match self {
            Self::LocalLang => Self::English,
            Self::English => Self::LocalLang,
        }
    }

    /// Sets the current language globally.
    ///
    /// This method updates the global language selection using atomic operations.
    /// The change affects all subsequent string lookups throughout the application.
    ///
    /// # Arguments
    ///
    /// * `lang` - The language to set as current.
    pub fn set_language(lang: Languages) {
        SELECTION.store(lang as u8, Ordering::Relaxed);
    }

    /// Returns the currently selected language.
    ///
    /// This method retrieves the current language selection from the atomic
    /// storage. The returned language determines which string variant is used
    /// for all internationalized strings.
    ///
    /// # Returns
    ///
    /// The currently selected language.
    pub fn get_language() -> Languages {
        let lang_idx = SELECTION.load(Ordering::Relaxed) as usize;
        if lang_idx == Languages::English as usize {
            Languages::English
        } else {
            Languages::LocalLang
        }
    }

    /// Switches to the next language in the sequence.
    ///
    /// This is a convenience method that combines `get_language()` and `next()`
    /// to cycle through available languages. It updates the global language
    /// selection to the next language in the sequence.
    pub fn switch_language() {
        Self::set_language(Self::get_language().next());
    }
}

/// An internationalized string that supports multiple languages.
///
/// This struct stores string variants for all supported languages and provides
/// automatic language selection based on the current global language setting.
/// It dereferences to a `&str` for convenient usage.
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // Create an internationalized string
/// let greeting = I18NString::new(["你好", "Hello"]);
///
/// // Use it like a string
/// println!("{}", greeting.as_str());
/// ```
#[derive(Debug, Copy, Clone)]
pub struct I18NString {
    /// Array of string variants, one for each supported language.
    /// The index corresponds to the `Languages` enum variant value.
    strings: [&'static str; LANG_COUNT],
}

impl I18NString {
    /// Creates a new internationalized string from an array of language-specific strings.
    ///
    /// The array must contain one string for each supported language, in the same order
    /// as the `Languages` enum variants.
    ///
    /// # Arguments
    ///
    /// * `strings` - An array of static string slices, one for each language.
    ///
    /// # Returns
    ///
    /// A new `I18NString` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use matrix_gui::prelude::*;
    ///
    /// const GREETING: I18NString = I18NString::new(["你好", "Hello"]);
    /// ```
    pub const fn new(strings: [&'static str; LANG_COUNT]) -> Self {
        Self { strings }
    }

    /// Returns the string for the currently selected language.
    ///
    /// This method looks up the appropriate string variant based on the global
    /// language selection. If the current language index is out of bounds,
    /// it falls back to the first language.
    ///
    /// # Returns
    ///
    /// A static string slice for the current language.
    pub fn as_str(&self) -> &'static str {
        let lang_idx = SELECTION.load(Ordering::Relaxed) as usize;
        self.strings.get(lang_idx).unwrap_or(&self.strings[0])
    }
}

/// Allows `I18NString` to be used like a string slice.
///
/// This implementation enables automatic dereferencing to `&str`, allowing
/// `I18NString` instances to be used wherever a string slice is expected.
impl Deref for I18NString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

/// Macro to define an internationalized string constant.
///
/// This macro creates a public constant of type `I18NString` that contains
/// string variants for all supported languages. The strings are provided
/// in the same order as the `Languages` enum variants.
///
/// # Syntax
///
/// ```ignore
/// matrix_gui::i18n_string!(NAME, string_for_lang0, string_for_lang1);
/// ```
///
/// # Arguments
///
/// * `NAME` - The identifier name for the constant
/// * `string_for_langN` - The string for each language (must match LANG_COUNT)
///
/// # Examples
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // Define internationalized strings for UI elements
/// matrix_gui::i18n_string!(TIP_ON, "开", "ON");
/// matrix_gui::i18n_string!(TIP_OFF, "关", "OFF");
/// matrix_gui::i18n_string!(MENU_SETTINGS, "设置", "Settings");
///
/// // Use the strings
/// println!("{}", TIP_ON.as_str());
/// ```
///
/// # Notes
///
/// - The number of string arguments must match the number of supported languages
/// - The order of strings must correspond to the `Languages` enum variants
/// - The generated constant is public and can be used across modules
#[macro_export]
macro_rules! i18n_string {
    ($name:ident, $($lang:expr),+) => {
        pub const $name: I18NString = I18NString::new([$($lang),+]);
    };
}

/// Macro to define a toggle type with internationalized string representations.
///
/// This macro creates a new struct that wraps a boolean value and provides
/// language-aware string representations through dereferencing. The struct
/// automatically selects the appropriate string based on the boolean value.
///
/// # Syntax
///
/// ```ignore
/// matrix_gui::i18n_toggle_type!(TypeName, true_string, false_string);
/// ```
///
/// # Arguments
///
/// * `TypeName` - The identifier name for the new struct
/// * `true_string` - The `I18NString` constant to use when the boolean is `true`
/// * `false_string` - The `I18NString` constant to use when the boolean is `false`
///
/// # Examples
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // First define the internationalized strings
/// matrix_gui::i18n_string!(TIP_ON, "开", "ON");
/// matrix_gui::i18n_string!(TIP_OFF, "关", "OFF");
///
/// // Define a toggle type
/// matrix_gui::i18n_toggle_type!(TipOnOff, TIP_ON, TIP_OFF);
///
/// // Use the toggle type
/// let toggle: &str = &TipOnOff(true);
/// println!("{}", toggle); // Prints "开" or "ON" based on current language
/// ```
///
/// # Generated Code
///
/// The macro generates a struct with the following structure:
///
/// ```ignore
/// pub struct TypeName(pub bool);
///
/// impl core::ops::Deref for TypeName {
///     type Target = str;
///     fn deref(&self) -> &Self::Target {
///         match self.0 {
///             true => &true_string,
///             false => &false_string,
///         }
///     }
/// }
/// ```
///
/// # Notes
///
/// - The generated struct implements `Deref` to `str` for easy string access
/// - The boolean value is stored in a tuple struct field
/// - Language switching affects the string representation automatically
#[macro_export]
macro_rules! i18n_toggle_type {
    ($type_name:ident, $true_str:expr, $false_str:expr) => {
        pub struct $type_name(pub bool);

        impl core::ops::Deref for $type_name {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                match self.0 {
                    true => &$true_str,
                    false => &$false_str,
                }
            }
        }
    };
}
