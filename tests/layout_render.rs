extern crate failure;
extern crate log;
extern crate stderrlog;
extern crate termion;
extern crate tui;

use tui::backend::{DummyBackend, DummyBuffer};
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;


#[test]
fn layout_rendering() {
    let mut buffer = DummyBuffer::new(40, 20);
    {
        let backend = DummyBackend::new(&mut buffer);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.hide_cursor().unwrap();

        let size = terminal.size().unwrap();
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                        .as_ref(),
                ).split(size);

            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .render(&mut f, chunks[0]);
            Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .render(&mut f, chunks[2]);
        }).unwrap();
    }

    buffer.assert_text_eq(concat!(
        "┌Block─────────────────────────────────┐\n",
        "└──────────────────────────────────────┘\n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "┌Block 2───────────────────────────────┐\n",
        "└──────────────────────────────────────┘"));
}
