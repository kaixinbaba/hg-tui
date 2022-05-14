use std::collections::HashMap;

use lazy_static::lazy_static;
use tui::style::Color;
use tui::style::Style;

use crate::app_global::IS_POOR;
use crate::widget::content::Category;

lazy_static! {
    pub static ref CATEGORY_STYLE: HashMap<Category, Style> = {
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
    };
    pub static ref TITLE_STYLE: Style = Style::default().fg(Color::Rgb(255, 192, 102));
}

pub fn choose_font_style(category: &Category) -> Style {
    if IS_POOR.load(std::sync::atomic::Ordering::Relaxed) {
        Style::default().fg(Color::White)
    } else if let Some(color_style) = CATEGORY_STYLE.get(category) {
        *color_style
    } else {
        Style::default().fg(Color::White)
    }
}
