use tui::backend::Backend;
use tui::style::{Color,  Style};
use tui::widgets::{Block,  Borders};
use tui::{layout::Alignment, Frame};

use crate::games::CardGame;

pub struct MyList<T> {
    items: Vec<T> 
}

struct MyListItem<'a> {
}

static MY_LIST: Vec<(&str, i32)> = [
    ("ItemO", 1),
    ("Item1", 2),
    ("Item2", 3),
];

pub fn ui<B: Backend>(f: &mut Frame<B>, card_game: &CardGame) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("ToDos")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    f.render_widget(block, size);
}
