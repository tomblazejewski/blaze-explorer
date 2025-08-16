pub enum Numbering {
    None,   //don't render the numbers at all
    Simple, // label from 0 to n
    VimLike, //selected row is 0, lines are numbered according to how far they are from the
            //selected row
}
pub fn get_line_numbers(
    n_lines: usize,
    current_line: usize,
    numbering: Numbering,
) -> Option<Vec<String>> {
    match numbering {
        Numbering::None => None,
        Numbering::Simple => Some((0..n_lines).map(|number| number.to_string()).collect()),
        Numbering::VimLike => {
            //create all string labels before the selected line
            let before_selected = (1..current_line)
                .rev()
                .map(|number| number.to_string())
                .collect::<Vec<String>>();
            let mut current_lines = before_selected;
            let n_lines_after = n_lines - current_line;
            let after_selected_iter = (1..n_lines_after + 1).map(|number| number.to_string());
            let current_line_string = format!("{} ", current_line);
            current_lines.append(&mut vec![current_line_string]);
            current_lines.extend(after_selected_iter);
            Some(current_lines)
        }
    }
}

mod tests {
    use crate::components::component_helpers::{Numbering, get_line_numbers};
    #[test]
    fn test_get_line_numbers_none() {
        let current_line = 3_usize;
        let line_length = 6_usize;
        let result = get_line_numbers(line_length, current_line, Numbering::None);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_line_numbers_simple() {
        let line_length = 4_usize;
        let current_line = 1_usize;
        let result = get_line_numbers(line_length, current_line, Numbering::Simple);
        let expected_result = Some(vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_line_numbers_vim_like() {
        let line_length = 4_usize;
        let current_line = 3_usize;
        let result = get_line_numbers(line_length, current_line, Numbering::VimLike);
        let expected_result = Some(vec![
            String::from("2"),
            String::from("1"),
            String::from("3 "),
            String::from("1"),
        ]);
        assert_eq!(result, expected_result);
    }
}
