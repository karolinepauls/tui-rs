use std::io;
use std::iter;

use itertools::Itertools;
use backend::Backend;
use buffer::Cell;
use layout::Rect;


pub struct DummyBuffer {
    width: u16,
    height: u16,
    cursor_shown: Option<bool>,
    buffer: Vec<Cell>
}

impl DummyBuffer {

    pub fn new(width: u16, height: u16) -> DummyBuffer {
        let size = usize::from(width) * usize::from(height);
        let drawn: Vec<Cell> = iter::repeat(Cell::default()).take(size).collect();
        DummyBuffer {
            width,
            height,
            cursor_shown: None,
            buffer: drawn
        }
    }

    fn clear(&mut self) {
        for mut cell in self.buffer.iter_mut() {
            cell.reset();
        }
    }

    fn hide_cursor(&mut self) {
        self.cursor_shown = Some(false);
    }

    fn show_cursor(&mut self) {
        self.cursor_shown = Some(true);
    }

    fn size(&self) -> io::Result<Rect> {
        Ok(Rect {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        })
    }

    fn draw<'a, I>(&mut self, content: I)
        where
            I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            if x > self.width || y > self.height {
                panic!("Drawing out of bounds");
            }
            self.buffer[usize::from(x) + usize::from(self.width * y)] = cell.clone();
        }
    }

    pub fn as_string(&self) -> String {
        self.buffer
            .chunks(usize::from(self.width))
            .map(|chunk| chunk.iter()
                .map(|chunk| &chunk.symbol[..])
                .collect::<String>())
            .join("\n")
    }

    pub fn assert_text_eq(&self, expected_text: &str) {
        let actual_text = self.as_string();
        assert!(actual_text == expected_text, "
=== Expected buffer: ===
{}
=== Actual buffer: ===
{}
", actual_text, expected_text);
    }
}

pub struct DummyBackend<'o> {
    buffer: &'o mut DummyBuffer
}

impl<'o> DummyBackend<'o> {
    pub fn new(buffer: &'o mut DummyBuffer) -> DummyBackend<'o> {
        DummyBackend { buffer }
    }
}

impl<'o> Backend for DummyBackend<'o> {
    fn clear(&mut self) -> io::Result<()> {
        self.buffer.clear();
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.buffer.hide_cursor();
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.buffer.show_cursor();
        Ok(())
    }

    fn size(&self) -> io::Result<Rect> {
        self.buffer.size()
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
        where
            I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        self.buffer.draw(content);
        Ok(())
    }
}
