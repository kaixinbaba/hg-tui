use std::collections::HashMap;
use std::str::FromStr;

use anyhow::bail;
use lazy_static::lazy_static;
use tui::style::Color;
use tui::style::Style;

use crate::app_global::IS_COLORFUL;
use crate::widget::content::Category;

lazy_static! {
    pub static ref CATEGORY_STYLE: HashMap<Category, Style> = init_category_style();
    pub static ref THEME_STYLE: HashMap<Theme, ThemeStyle> = init_theme();
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ThemeStyle {
    pub background: Style,
    pub title: Style,
    pub tips: Style,
    pub selected: Style,
    pub text: Style,
    pub key: Style,
}

fn init_theme() -> HashMap<Theme, ThemeStyle> {
    let mut map = HashMap::new();

    map.insert(Theme::LightDefault, init_light_default());

    map.insert(Theme::LightColorful, init_light_colorful());

    map.insert(Theme::DarkDefault, init_dark_default());

    map.insert(Theme::DarkColorful, init_dark_colorful());

    map.insert(Theme::HighContrast, init_high_contrast());

    map
}

fn init_light_default() -> ThemeStyle {
    ThemeStyle {
        background: Style::default().bg(Color::Rgb(242, 242, 242)),
        title: Style::default().fg(Color::Rgb(0, 46, 186)),
        tips: Style::default().fg(Color::Rgb(100, 100, 100)),
        selected: Style::default().bg(Color::Rgb(252, 250, 236)),
        text: Style::default().fg(Color::Rgb(8, 8, 8)),
        key: Style::default().fg(Color::Rgb(255, 0, 0)),
    }
}

fn init_light_colorful() -> ThemeStyle {
    ThemeStyle {
        background: Style::default().bg(Color::Rgb(242, 242, 242)),
        title: Style::default().fg(Color::Rgb(0, 46, 186)),
        tips: Style::default().fg(Color::Rgb(100, 100, 100)),
        selected: Style::default().bg(Color::Rgb(252, 250, 236)),
        text: Style::default().fg(Color::Rgb(8, 8, 8)),
        key: Style::default().fg(Color::Rgb(255, 0, 0)),
    }
}

fn init_dark_default() -> ThemeStyle {
    ThemeStyle {
        background: Style::default().bg(Color::Rgb(43, 43, 43)),
        title: Style::default().fg(Color::Rgb(196, 107, 28)),
        tips: Style::default().fg(Color::Rgb(95, 99, 102)),
        selected: Style::default().bg(Color::Rgb(50, 50, 50)),
        text: Style::default().fg(Color::Rgb(166, 183, 200)),
        key: Style::default().fg(Color::Green),
    }
}

fn init_dark_colorful() -> ThemeStyle {
    ThemeStyle {
        background: Style::default().bg(Color::Rgb(43, 43, 43)),
        title: Style::default().fg(Color::Rgb(196, 107, 28)),
        tips: Style::default().fg(Color::Rgb(95, 99, 102)),
        selected: Style::default().bg(Color::Rgb(50, 50, 50)),
        text: Style::default().fg(Color::Rgb(166, 183, 200)),
        key: Style::default().fg(Color::Green),
    }
}

fn init_high_contrast() -> ThemeStyle {
    ThemeStyle {
        background: Style::default().bg(Color::Rgb(19, 19, 20)),
        title: Style::default().fg(Color::Rgb(0, 226, 240)),
        tips: Style::default().fg(Color::Rgb(104, 197, 234)),
        selected: Style::default().bg(Color::Rgb(5, 0, 107)),
        text: Style::default().fg(Color::Rgb(255, 255, 255)),
        key: Style::default().fg(Color::Rgb(252, 140, 255)),
    }
}

fn init_category_style() -> HashMap<Category, Style> {
    let mut map = HashMap::new();

    map.insert(Category::C, Style::default().fg(Color::Rgb(85, 85, 85)));
    map.insert(Category::Cpp, Style::default().fg(Color::Rgb(243, 75, 125)));
    map.insert(
        Category::Csharp,
        Style::default().fg(Color::Rgb(23, 134, 1)),
    );
    map.insert(Category::Css, Style::default().fg(Color::Rgb(86, 62, 124)));
    map.insert(
        Category::Java,
        Style::default().fg(Color::Rgb(175, 114, 25)),
    );
    map.insert(
        Category::Javascript,
        Style::default().fg(Color::Rgb(240, 224, 90)),
    );
    map.insert(Category::Go, Style::default().fg(Color::Rgb(1, 173, 216)));
    map.insert(
        Category::Rust,
        Style::default().fg(Color::Rgb(221, 163, 132)),
    );
    map.insert(
        Category::Python,
        Style::default().fg(Color::Rgb(53, 114, 165)),
    );
    map.insert(Category::Php, Style::default().fg(Color::Rgb(79, 93, 149)));
    map.insert(
        Category::ObjectC,
        Style::default().fg(Color::Rgb(67, 142, 255)),
    );
    map.insert(Category::Ruby, Style::default().fg(Color::Rgb(112, 20, 21)));
    map.insert(
        Category::Swift,
        Style::default().fg(Color::Rgb(240, 81, 55)),
    );
    map.insert(
        Category::Kotlin,
        Style::default().fg(Color::Rgb(169, 123, 255)),
    );

    map
}

pub fn choose_font_style(category: &Category, theme_style: &ThemeStyle) -> Style {
    if IS_COLORFUL.load(std::sync::atomic::Ordering::Relaxed) {
        if let Some(color_style) = CATEGORY_STYLE.get(category) {
            *color_style
        } else {
            Style::default().fg(Color::White)
        }
    } else {
        theme_style.text
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Theme {
    /// 浅色默认样式
    LightDefault,

    /// 深色彩色样式
    LightColorful,

    /// 深色默认样式
    DarkDefault,

    /// 深色彩色样式
    DarkColorful,

    /// 高对比度
    HighContrast,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::DarkColorful
    }
}

impl Theme {
    pub fn theme_list() -> Vec<&'static str> {
        vec![
            Theme::LightDefault.into(),
            Theme::LightColorful.into(),
            Theme::DarkDefault.into(),
            Theme::DarkColorful.into(),
            Theme::HighContrast.into(),
        ]
    }
}

impl From<Theme> for &str {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::LightDefault => "lightdefault",
            Theme::LightColorful => "lightcolorful",
            Theme::DarkDefault => "darkdefault",
            Theme::DarkColorful => "darkcolorful",
            Theme::HighContrast => "highcontrast",
        }
    }
}

impl FromStr for Theme {
    type Err = anyhow::Error;

    fn from_str(color_theme: &str) -> Result<Self, Self::Err> {
        let color_theme = color_theme.to_lowercase();
        let theme = match color_theme.as_ref() {
            "lightdefault" => Theme::LightDefault,
            "lightcolorful" => Theme::LightColorful,
            "darkdefault" => Theme::DarkDefault,
            "darkcolorful" => Theme::DarkColorful,
            "highcontrast" => Theme::HighContrast,
            _ => bail!("unsupport theme '{}'", &color_theme),
        };

        Ok(theme)
    }
}
