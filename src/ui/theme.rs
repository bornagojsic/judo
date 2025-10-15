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
    // Add more as needed
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
}
