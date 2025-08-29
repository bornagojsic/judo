pub struct Logo;
use crate::db::models::{NewTodoItem, TodoItem, UIItem, UIList};
use anyhow::Result;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph,
    StatefulWidget, Widget,
};
use sqlx::SqlitePool;
use std::str::FromStr;

// Use the judo ascii logo
const ascii_logo: &str = r#"
     ██╗██╗   ██╗██████╗  ██████╗ 
     ██║██║   ██║██╔══██╗██╔═══██╗
     ██║██║   ██║██║  ██║██║   ██║
██   ██║██║   ██║██║  ██║██║   ██║
╚█████╔╝╚██████╔╝██████╔╝╚██████╔╝
 ╚════╝  ╚═════╝ ╚═════╝  ╚═════╝ 
        "#;

impl Logo {
    pub fn render(area: Rect, buf: &mut Buffer) {
        // Define a block and pad
        let block = Block::default().padding(Padding::horizontal(2));

        Paragraph::new(ascii_logo)
            .bold()
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}
