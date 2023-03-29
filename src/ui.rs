use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color,  Style};
use tui::widgets::{Block,  Borders};
use tui::{layout::Alignment, Frame};

use crate::games::CardGame;

pub fn ui<B: Backend>(f: &mut Frame<B>, card_game: &CardGame) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("ToDos")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    f.render_widget(block, size);
}
