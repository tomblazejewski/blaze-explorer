use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Cell, Row},
};

use crate::{
    components::explorer_table::{FileData, GlobalStyling},
    themes::CustomTheme,
};
fn highlight_search_result(line_text: String, query: &str, highlighted_style: Style) -> Line {
    if line_text.contains(query) {
        let splits = line_text.split(&query);
        let chunks = splits.into_iter().map(|c| Span::from(c.to_owned()));
        let pattern = Span::styled(query, highlighted_style);
        itertools::intersperse(chunks, pattern)
            .collect::<Vec<Span>>()
            .into()
    } else {
        Line::from(line_text)
    }
}

pub fn jump_highlight(
    line_text: String,
    query: &str,
    char: char,
    query_style: Style,
    char_style: Style,
) -> Line {
    if line_text.contains(&query) {
        let mut splits = line_text.split(&query);
        let beginning = Span::from(splits.next().unwrap().to_string());
        let query_span = Span::styled(query, query_style);
        let mut remainder = splits.remainder().unwrap_or("").to_string();
        if !remainder.is_empty() {
            remainder.remove(0);
        }
        let char_span = Span::styled(char.to_string(), char_style);
        let remainder_span = Span::from(remainder);
        Line::from(vec![beginning, query_span, char_span, remainder_span])
    } else {
        Line::from(line_text)
    }
}
pub fn convert_filedata_to_row(
    element: FileData,
    row_number: String,
    theme: CustomTheme,
    styling: GlobalStyling,
) -> Row {
    // There is more than one source of formatting
    // Formatting is done in particular order, so that later stages can overwrite already set
    // properties
    // 1. Git status
    //
    let row_number_cell = Cell::from(Text::from(row_number).alignment(Alignment::Right));
    let file_name_cell = Cell::from(match styling {
        GlobalStyling::None => Line::from(element.filename.clone()),
        GlobalStyling::HighlightSearch(_) => {
            highlight_search_result(element.filename.clone(), &query, self.theme.search_result)
        }
        GlobalStyling::HighlightJump(_, _) => jump_highlight(
            element.filename.clone(),
            &query,
            inverted_map.get(&element.id).unwrap_or(&' ').to_owned(),
            self.theme.highlight_query.clone(),
            self.theme.highlight_jump_char.clone(),
        ),
    });
}
