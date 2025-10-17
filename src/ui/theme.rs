use ratatui::style::{Color, Style};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Theme {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub border: String,
    pub highlight_bg: String,
    pub highlight_fg: String,
    pub border_accent: String,
    pub highlight_not_focused_fg: String,
    pub highlight_not_focused_bg: String,
    pub highlight_line_number_fg: String,
}

impl Theme {
    pub fn color_from_hex(hex: &str) -> Color {
        Color::from_str(hex).unwrap_or(Color::Reset)
    }

    pub fn default() -> Theme {
        Theme {
            background: "#002626".to_string(),
            foreground: "#FCF1D5".to_string(),
            accent: "#FFA69E".to_string(),
            border: "#FCF1D5".to_string(),
            highlight_bg: "#FCF1D5".to_string(),
            highlight_fg: "#002626".to_string(),
            border_accent: "#FFA69E".to_string(),
            highlight_not_focused_fg: "#002626".to_string(),
            highlight_not_focused_bg: "#FCF1D5".to_string(),
            highlight_line_number_fg: "#002626".to_string(),
        }
    }

    pub fn fg(hex_color: &str) -> Style {
        Style::default().fg(Self::color_from_hex(hex_color))
    }

    pub fn bg(hex_color: &str) -> Style {
        Style::default().bg(Self::color_from_hex(hex_color))
    }

    pub fn fg_bg(fg_hex: &str, bg_hex: &str) -> Style {
        Style::default()
            .fg(Self::color_from_hex(fg_hex))
            .bg(Self::color_from_hex(bg_hex))
    }

    pub fn highlight(&self, focused: bool) -> Style {
        if focused {
            Self::fg_bg(&self.highlight_fg, &self.highlight_bg)
        } else {
            Self::fg_bg(
                &self.highlight_not_focused_fg,
                &self.highlight_not_focused_bg,
            )
        }
    }

    pub fn line_number(&self) -> Style {
        Self::fg(&self.highlight_line_number_fg)
    }
}
