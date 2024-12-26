use ratatui::{
    style::Style,
    text::{Line, Span},
};

pub fn highlight_search_result(line_text: String, query: &str, highlighted_style: Style) -> Line {
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
    if line_text.contains(query) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_highlight() {
        let line_text = String::from("Hello world");
        let query = "w";
        let char = 'w';
        let query_style = Style::new().fg(ratatui::style::Color::Red);
        let char_style = Style::new().fg(ratatui::style::Color::Green);
        let line = jump_highlight(line_text, query, char, query_style, char_style);
        let beginning = Span::from("Hello ");
        let query_span = Span::styled(query, query_style);
        let char_span = Span::styled(char.to_string(), char_style);
        let remainder_span = Span::from("rld");
        let expected_line = Line::from(vec![beginning, query_span, char_span, remainder_span]);
        assert_eq!(line, expected_line);
    }

    #[test]
    fn test_highlight_search_result() {
        let line_text = String::from("Hello world");
        let query = "worl";
        let highlighted_style = Style::new().fg(ratatui::style::Color::Green);
        let line = highlight_search_result(line_text, query, highlighted_style);
        let beginning = Span::from("Hello ");
        let query_span = Span::styled(query, highlighted_style);
        let ending = Span::from("d");
        let expected_line = Line::from(vec![beginning, query_span, ending]);
        assert_eq!(line, expected_line);
    }
}
