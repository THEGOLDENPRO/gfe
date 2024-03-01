use iced::{widget::{self, text_editor::{Appearance, StyleSheet}}, Background, Border, Color, Theme};

pub struct TextBoxStyle {
    pub theme: Theme
}

impl StyleSheet for TextBoxStyle {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> widget::text_editor::Appearance {
        let mut colour = Color::BLACK;
        colour.a = 0.8;

        Appearance {
            background: Background::Color(colour),
            border: Border::with_radius(0),
        }
    }

    fn focused(&self, _: &Self::Style) -> widget::text_editor::Appearance {
        let mut colour = Color::BLACK;
        colour.a = 0.8;

        Appearance {
            background: Background::Color(colour),
            border: Border::with_radius(0),
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> iced::Color {
        let palette = self.theme.extended_palette();

        palette.background.strong.color
    }

    fn value_color(&self, _: &Self::Style) -> iced::Color {
        let palette = self.theme.extended_palette();

        palette.background.base.text
    }

    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        self.placeholder_color(style)
    }

    fn selection_color(&self, _: &Self::Style) -> iced::Color {
        let palette = self.theme.extended_palette();

        palette.secondary.weak.color
    }

    fn disabled(&self, _: &Self::Style) -> widget::text_editor::Appearance {
        Appearance {
            background: Background::Color(Color::BLACK),
            border: Border::with_radius(0),
        }
    }
}
