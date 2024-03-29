use bevy::{prelude::*, utils::HashMap};
use bevy_lunex::prelude::NodeSize;




pub struct Theme {
    /// Name of the theme
    pub name: String,
    /// Theme tags (ex. "Dark" or "Colorblind")
    pub tags: Vec<String>,

    /// Primary striking color
    pub primary   : ColorPair,
    /// Secondary striking color
    pub secondary : ColorPair,
    /// Tertiary striking color
    pub tertiary  : ColorPair,
    /// Color of the "INFO" widgets
    pub info      : ColorPair,
    /// Color of the "WARNING" widgets
    pub warning   : ColorPair,
    /// Color of the "SUCCESS" widgets
    pub success   : ColorPair,
    /// Color of the "ERROR" widgets
    pub error     : ColorPair,
    /// Color of the background fill
    pub surface   : ColorPair,

    /// The color of the text
    pub text: Color,

    /// Additional custom colors that didn't fit in the template
    pub custom: HashMap<String, ColorPair>,

    /// Text font of general text
    pub font_base: Handle<Font>,
    /// Text font of headings
    pub font_heading: Handle<Font>,


    pub rounding_container: NodeSize<Vec4>,
    pub rounding_base     : NodeSize<Vec4>,
    pub border_thickness  : NodeSize<Vec4>,

    pub highlight_color   : ThemeColor,
    pub border_color      : ThemeColor,
}


/// Pair of color set and one text color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ColorPair {
    pub text: Color,
    pub base: ColorSet,
}

/// Set of a different shades of one color sorted from lightest to darkest
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ColorSet {
    pub shades: Vec<Color>,
}
impl ColorSet {
    pub fn get_val(&self, x: f32) -> Color {
        let n = self.shades.len() as f32;
        if n == 0.0 { return Color::WHITE }
        let i = f32::clamp((n - 1.0) * (x / 1000.0), 0.0, n) as usize;
        self.shades[i]
    }
    pub fn get_index(&self, i: usize) -> Color {
        if i+1 >= self.shades.len() { return Color::WHITE }
        self.shades[i]
    }
}
impl From<Color> for ColorSet {
    fn from(value: Color) -> Self {
        ColorSet { shades: vec![value] }
    }
}
impl From<Vec<Color>> for ColorSet {
    fn from(value: Vec<Color>) -> Self {
        ColorSet { shades: value }
    }
}

/// Color selector from theme
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeColor {
    Primary(f32),
    Secondary(f32),
    Tertiary(f32),
    Info(f32),
    Warning(f32),
    Success(f32),
    Error(f32),
    Surface(f32),
    Custom(String, f32),
    Unique(ColorPair, f32),
}
impl Default for ThemeColor {
    fn default() -> Self {
        ThemeColor::SURFACE_500
    }
}
impl ThemeColor {
    pub const PRIMARY_50: ThemeColor = ThemeColor::Primary(50.0);
    pub const PRIMARY_100: ThemeColor = ThemeColor::Primary(100.0);
    pub const PRIMARY_200: ThemeColor = ThemeColor::Primary(200.0);
    pub const PRIMARY_300: ThemeColor = ThemeColor::Primary(300.0);
    pub const PRIMARY_400: ThemeColor = ThemeColor::Primary(400.0);
    pub const PRIMARY_500: ThemeColor = ThemeColor::Primary(500.0);
    pub const PRIMARY_600: ThemeColor = ThemeColor::Primary(600.0);
    pub const PRIMARY_700: ThemeColor = ThemeColor::Primary(700.0);
    pub const PRIMARY_800: ThemeColor = ThemeColor::Primary(800.0);
    pub const PRIMARY_900: ThemeColor = ThemeColor::Primary(900.0);

    pub const SECONDARY_50: ThemeColor = ThemeColor::Secondary(50.0);
    pub const SECONDARY_100: ThemeColor = ThemeColor::Secondary(100.0);
    pub const SECONDARY_200: ThemeColor = ThemeColor::Secondary(200.0);
    pub const SECONDARY_300: ThemeColor = ThemeColor::Secondary(300.0);
    pub const SECONDARY_400: ThemeColor = ThemeColor::Secondary(400.0);
    pub const SECONDARY_500: ThemeColor = ThemeColor::Secondary(500.0);
    pub const SECONDARY_600: ThemeColor = ThemeColor::Secondary(600.0);
    pub const SECONDARY_700: ThemeColor = ThemeColor::Secondary(700.0);
    pub const SECONDARY_800: ThemeColor = ThemeColor::Secondary(800.0);
    pub const SECONDARY_900: ThemeColor = ThemeColor::Secondary(900.0);

    pub const TERTIARY_50: ThemeColor = ThemeColor::Tertiary(50.0);
    pub const TERTIARY_100: ThemeColor = ThemeColor::Tertiary(100.0);
    pub const TERTIARY_200: ThemeColor = ThemeColor::Tertiary(200.0);
    pub const TERTIARY_300: ThemeColor = ThemeColor::Tertiary(300.0);
    pub const TERTIARY_400: ThemeColor = ThemeColor::Tertiary(400.0);
    pub const TERTIARY_500: ThemeColor = ThemeColor::Tertiary(500.0);
    pub const TERTIARY_600: ThemeColor = ThemeColor::Tertiary(600.0);
    pub const TERTIARY_700: ThemeColor = ThemeColor::Tertiary(700.0);
    pub const TERTIARY_800: ThemeColor = ThemeColor::Tertiary(800.0);
    pub const TERTIARY_900: ThemeColor = ThemeColor::Tertiary(900.0);

    pub const INFO_50: ThemeColor = ThemeColor::Info(50.0);
    pub const INFO_100: ThemeColor = ThemeColor::Info(100.0);
    pub const INFO_200: ThemeColor = ThemeColor::Info(200.0);
    pub const INFO_300: ThemeColor = ThemeColor::Info(300.0);
    pub const INFO_400: ThemeColor = ThemeColor::Info(400.0);
    pub const INFO_500: ThemeColor = ThemeColor::Info(500.0);
    pub const INFO_600: ThemeColor = ThemeColor::Info(600.0);
    pub const INFO_700: ThemeColor = ThemeColor::Info(700.0);
    pub const INFO_800: ThemeColor = ThemeColor::Info(800.0);
    pub const INFO_900: ThemeColor = ThemeColor::Info(900.0);

    pub const WARNING_50: ThemeColor = ThemeColor::Warning(50.0);
    pub const WARNING_100: ThemeColor = ThemeColor::Warning(100.0);
    pub const WARNING_200: ThemeColor = ThemeColor::Warning(200.0);
    pub const WARNING_300: ThemeColor = ThemeColor::Warning(300.0);
    pub const WARNING_400: ThemeColor = ThemeColor::Warning(400.0);
    pub const WARNING_500: ThemeColor = ThemeColor::Warning(500.0);
    pub const WARNING_600: ThemeColor = ThemeColor::Warning(600.0);
    pub const WARNING_700: ThemeColor = ThemeColor::Warning(700.0);
    pub const WARNING_800: ThemeColor = ThemeColor::Warning(800.0);
    pub const WARNING_900: ThemeColor = ThemeColor::Warning(900.0);

    pub const SUCCESS_50: ThemeColor = ThemeColor::Success(50.0);
    pub const SUCCESS_100: ThemeColor = ThemeColor::Success(100.0);
    pub const SUCCESS_200: ThemeColor = ThemeColor::Success(200.0);
    pub const SUCCESS_300: ThemeColor = ThemeColor::Success(300.0);
    pub const SUCCESS_400: ThemeColor = ThemeColor::Success(400.0);
    pub const SUCCESS_500: ThemeColor = ThemeColor::Success(500.0);
    pub const SUCCESS_600: ThemeColor = ThemeColor::Success(600.0);
    pub const SUCCESS_700: ThemeColor = ThemeColor::Success(700.0);
    pub const SUCCESS_800: ThemeColor = ThemeColor::Success(800.0);
    pub const SUCCESS_900: ThemeColor = ThemeColor::Success(900.0);

    pub const ERROR_50: ThemeColor = ThemeColor::Error(50.0);
    pub const ERROR_100: ThemeColor = ThemeColor::Error(100.0);
    pub const ERROR_200: ThemeColor = ThemeColor::Error(200.0);
    pub const ERROR_300: ThemeColor = ThemeColor::Error(300.0);
    pub const ERROR_400: ThemeColor = ThemeColor::Error(400.0);
    pub const ERROR_500: ThemeColor = ThemeColor::Error(500.0);
    pub const ERROR_600: ThemeColor = ThemeColor::Error(600.0);
    pub const ERROR_700: ThemeColor = ThemeColor::Error(700.0);
    pub const ERROR_800: ThemeColor = ThemeColor::Error(800.0);
    pub const ERROR_900: ThemeColor = ThemeColor::Error(900.0);

    pub const SURFACE_50: ThemeColor = ThemeColor::Surface(50.0);
    pub const SURFACE_100: ThemeColor = ThemeColor::Surface(100.0);
    pub const SURFACE_200: ThemeColor = ThemeColor::Surface(200.0);
    pub const SURFACE_300: ThemeColor = ThemeColor::Surface(300.0);
    pub const SURFACE_400: ThemeColor = ThemeColor::Surface(400.0);
    pub const SURFACE_500: ThemeColor = ThemeColor::Surface(500.0);
    pub const SURFACE_600: ThemeColor = ThemeColor::Surface(600.0);
    pub const SURFACE_700: ThemeColor = ThemeColor::Surface(700.0);
    pub const SURFACE_800: ThemeColor = ThemeColor::Surface(800.0);
    pub const SURFACE_900: ThemeColor = ThemeColor::Surface(900.0);

}
