use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::canvas::{Canvas, Map, MapResolution, Points};
use tui::widgets::{Block, BorderType, Borders, Gauge, Sparkline};
use tui::{layout::Alignment, Frame};

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Main Block with round corners")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let block = Block::default()
        .title(vec![
            Span::styled("With", Style::default().fg(Color::Yellow)),
            Span::from(" background"),
        ])
        .style(Style::default().bg(Color::Green));
    // f.render_widget(block, top_chunks[0]);

    let line = Sparkline::default()
        .block(Block::default().title("Sparkline").borders(Borders::ALL))
        .data(&[0, 2, 3, 4, 1, 4, 10])
        .max(5)
        .style(Style::default().fg(Color::Red).bg(Color::White));
    // f.render_widget(line, top_chunks[1]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(100),
                //Constraint::Percentage(20),
                //Constraint::Percentage(20),
                //Constraint::Percentage(20),
                //Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
}

