use crate::input_machine::input_machine_helpers::{convert_str_to_events, parse_key_sequence};
#[macro_export]
macro_rules! insert_binding {
    ($map:expr, $mode:expr, $binding:expr, $functionality:expr) => {{
        let events = convert_str_to_events($binding);
        $map.insert(($mode, events), $functionality.to_string());
    }};
}

#[macro_export]
macro_rules! insert_permutated_functionality {
    ($map:expr, $key_template:expr, [$($set:expr),+], $func:ident, $($ch:ident),+) => {{
        use itertools::Itertools;

        let mut pools = Vec::new();
        $(
            pools.push($set.elements());
        )+

        for combination in pools.iter().map(|v| v.iter()).multi_cartesian_product() {
            // Create named bindings for each char in the combination
            let mut iter = combination.iter();
            $(
                let $ch = **iter.next().unwrap();
            )+

            // Replace each {} in the template with the current character
            let mut key = $key_template.to_string();
            for c in &combination {
                key = key.replacen("{}", &c.to_string(), 1);
            }

            let action = $func($($ch),+);
            $map.insert(key, action);
        }
    }};
}

#[macro_export]
macro_rules! insert_permutated_binding {
    (
        $map:expr,
        $mode:expr,
        $key_template:expr,
        [$($set:expr),+],
        $action_template:expr
    ) => {{
        use itertools::Itertools;

        let mut pools = Vec::new();
        $(
            pools.push($set.elements());
        )+

        for combination in pools.iter().map(|v| v.iter()).multi_cartesian_product() {
            // Generate the key string
            let mut key_string = $key_template.to_string();
            for c in &combination {
                key_string = key_string.replacen("{}", &c.to_string(), 1);
            }

            // Generate the action string
            let mut action_string = $action_template.to_string();
            for c in &combination {
                action_string = action_string.replacen("{}", &c.to_string(), 1);
            }

            let keys = convert_str_to_events(&key_string);
            $map.insert(($mode, keys), action_string);
        }
    }};
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::*;
    use crate::{
        action::{Action, AppAction},
        input_machine::permutation_set::PermutationSet,
        mode::Mode,
    };

    #[test]
    fn test_insert_binding() {
        let mut telescope_bindings = HashMap::new();
        let binding_str = "<C-a>";
        let functionality_str = "OpenSFS";
        insert_binding!(
            telescope_bindings,
            Mode::Normal,
            binding_str,
            functionality_str
        );
        let mut expected_map = HashMap::new();
        expected_map.insert(
            (Mode::Normal, vec![KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::CONTROL,
            )]),
            "OpenSFS".to_string(),
        );
        assert_eq!(telescope_bindings, expected_map);
    }

    #[test]
    fn test_insert_permutated_functionality() {
        let mut map = HashMap::new();
        let action_template = "DisplayMessage-{}-{}";

        fn mapping_func(a: char, b: char) -> Action {
            let message = format!("{}{}", a, b);
            Action::AppAct(AppAction::DisplayMessage(message))
        }

        insert_permutated_functionality!(
            map,
            action_template,
            [PermutationSet::LowerAlpha, PermutationSet::Digits],
            mapping_func,
            a,
            b
        );

        assert!(map.len() == 260);
        let random_element = map.get("DisplayMessage-a-0").unwrap();
        assert!(random_element == &Action::AppAct(AppAction::DisplayMessage("a0".to_string())));
    }

    #[test]
    fn test_insert_permutated_binding() {
        let mut bindings_map = HashMap::new();
        insert_permutated_binding!(
            bindings_map,
            Mode::Normal,
            "<C-{}>{}",
            [PermutationSet::LowerAlpha, PermutationSet::Digits],
            "DisplayMessage-{}-{}"
        );

        assert!(bindings_map.len() == 260);
        let random_element = bindings_map.get(&(Mode::Normal, vec![
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL),
            KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
        ]));
        assert!(random_element == Some(&"DisplayMessage-a-0".to_string()));
    }
}
