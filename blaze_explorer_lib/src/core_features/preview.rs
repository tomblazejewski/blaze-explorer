use crate::{
    components::component_helpers::Numbering,
    plugin::{base_popup::get_default_popup_keymap, plugin_popup::PluginPopUp},
    tools::center_rect,
};
use ratatui::{Frame, prelude::*, widgets::*};
use std::fmt::Debug;

///A trait allowing the struct to display its contents in a preview component.
pub trait Previewable {
    ///Returns the data to be displayed
    fn collect_data(&self) -> Vec<String>;
    ///Returns the type of numbering that should be used
    fn get_numbering(&self) -> Numbering;
    ///Returns the line numbers
    fn get_line_numbers(&self) -> Option<Vec<String>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreviewWindow<'a, T: Previewable> {
    source: &'a T,
    should_quit: bool,
}

impl<'a, T: Previewable> PreviewWindow<'a, T> {
    pub fn new(source: &'a T) -> Self {
        Self {
            source,
            should_quit: false,
        }
    }
}

impl<'a, T: Previewable + Clone + Debug + PartialEq> PluginPopUp for PreviewWindow<'a, T> {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::eyre::Result<()> {
        let rows = match self.source.get_line_numbers() {
            None => self
                .source
                .collect_data()
                .into_iter()
                .map(|x| Row::new(vec![Line::from(x.clone())]))
                .collect::<Vec<Row>>(),
            Some(line_numbers) => self
                .source
                .collect_data()
                .into_iter()
                .zip(line_numbers.clone())
                .map(|(data, line_number)| {
                    Row::new(vec![
                        Line::from(line_number.clone()),
                        Line::from(data.clone()),
                    ])
                })
                .collect::<Vec<Row>>(),
        };
        let widths = match self.source.get_line_numbers() {
            None => vec![Constraint::Percentage(100)],
            Some(_) => vec![Constraint::Percentage(5), Constraint::Percentage(95)],
        };
        let t = Table::new(rows, widths).block(Block::new().borders(Borders::ALL));
        frame.render_widget(t, area);
        Ok(())
    }

    fn push_search_char(&mut self, ch: char) -> Option<crate::action::Action> {
        None
    }

    fn drop_search_char(&mut self) -> Option<crate::action::Action> {
        None
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn erase_text(&mut self) -> Option<crate::action::Action> {
        None
    }

    fn get_search_query(&self) -> String {
        "".to_string()
    }

    fn display_details(&self) -> String {
        "Preview".to_string()
    }

    fn get_own_keymap(
        &self,
    ) -> std::collections::HashMap<
        (crate::mode::Mode, Vec<ratatui::crossterm::event::KeyEvent>),
        crate::action::Action,
    > {
        get_default_popup_keymap()
    }
}

#[cfg(test)]
mod tests {
    use crate::components::component_helpers::{Numbering, get_line_numbers};

    use super::*;
    use ratatui::{Terminal, backend::TestBackend, buffer::Buffer};

    #[derive(Debug, Clone, PartialEq)]
    struct DummyPreviewable {
        items: Vec<String>,
        numbering: Numbering,
        current_line: usize,
    }

    impl Previewable for DummyPreviewable {
        fn collect_data(&self) -> Vec<String> {
            self.items.clone()
        }

        fn get_numbering(&self) -> Numbering {
            self.numbering
        }

        fn get_line_numbers(&self) -> Option<Vec<String>> {
            get_line_numbers(self.items.len(), self.current_line, self.numbering)
        }
    }

    #[test]
    fn test_draw_previewwindow_numbering_none() {
        let dummy = DummyPreviewable {
            items: vec!["foo".into(), "bar".into()],
            numbering: Numbering::None,
            current_line: 0,
        };
        let mut preview = PreviewWindow::new(&dummy);

        let backend = TestBackend::new(20, 4);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = f.size();
                preview.draw(f, area).unwrap();
            })
            .unwrap();

        let expected = Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│foo               │",
            "│bar               │",
            "└──────────────────┘",
        ]);

        terminal.backend().assert_buffer(&expected);
    }

    #[test]
    fn test_draw_previewwindow_numbering_simple() {
        let dummy = DummyPreviewable {
            items: vec!["alpha".into(), "beta".into()],
            numbering: Numbering::Simple,
            current_line: 0,
        };
        let mut preview = PreviewWindow::new(&dummy);

        let backend = TestBackend::new(20, 4);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = f.size();
                preview.draw(f, area).unwrap();
            })
            .unwrap();

        // text first, then numbers
        let expected = Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│0 alpha           │",
            "│1 beta            │",
            "└──────────────────┘",
        ]);

        terminal.backend().assert_buffer(&expected);
    }

    #[test]
    fn test_draw_previewwindow_numbering_vimlike() {
        let dummy = DummyPreviewable {
            items: vec!["one".into(), "two".into(), "three".into(), "four".into()],
            numbering: Numbering::VimLike,
            current_line: 2,
        };
        let mut preview = PreviewWindow::new(&dummy);

        let backend = TestBackend::new(20, 6);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = f.size();
                preview.draw(f, area).unwrap();
            })
            .unwrap();

        // text first, then numbers
        let expected = Buffer::with_lines(vec![
            "┌──────────────────┐",
            "│1 one             │",
            "│2 two             │",
            "│1 three           │",
            "│2 four            │",
            "└──────────────────┘",
        ]);

        terminal.backend().assert_buffer(&expected);
    }
}
