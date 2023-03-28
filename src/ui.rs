use tui::backend::Backend;
use tui::widgets::{Block, BorderType, Borders};
use tui::{layout::Alignment, Frame};

use crate::games::CardGame;

pub fn ui<B: Backend>(f: &mut Frame<B>, card_game: &CardGame) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Main Block with round corners")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);
    for card in card_game.draw_pile[0..4].iter()  {
        println!("{:?}", card)
    }
}
