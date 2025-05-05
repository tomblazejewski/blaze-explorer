use core::panic;
use std::collections::HashMap;
use std::path::PathBuf;

use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use super::explorer_table::{ExplorerTable, FileData, GlobalStyling};
use crate::explorer_helpers::{
    calculate_distance, convert_sequence_to_string, delegate_to_focused,
};
use crate::history_stack::directory_history::DirectoryHistory;
use crate::mode::Mode;

#[derive(Clone, Debug, PartialEq)]
pub enum ParentRelationship {
    SomeParent(usize, usize),
    NoParent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Split {
    Horizontal(usize, usize),
    Vertical(usize, usize),
    Single(ExplorerTable),
}

pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExplorerManager {
    pub explorers: HashMap<usize, ExplorerNode>,
    pub focused_id: usize,
    pub next_id: usize,
    pub last_layout: HashMap<usize, Rect>,
}

impl Default for ExplorerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerManager {
    pub fn new() -> Self {
        let mut explorer_map = HashMap::new();
        explorer_map.insert(0, ExplorerNode::new(0));
        Self {
            explorers: explorer_map,
            focused_id: 0,
            next_id: 1,
            last_layout: HashMap::new(),
        }
    }

    pub fn get_new_id(&mut self) -> usize {
        let new_id = self.next_id;
        self.next_id += 1;
        new_id
    }
    pub fn split_vertically_action(&mut self) {
        let id_0 = self.get_new_id();
        let id_1 = self.get_new_id();
        let focused_node = self.explorers.get_mut(&self.focused_id).unwrap();
        let (mut node_0, node_1) = focused_node.split_vertically(id_0, id_1);
        node_0.focused = false;
        self.explorers.insert(id_0, node_0);
        self.explorers.insert(id_1, node_1);
        self.focused_id = id_1;
    }

    pub fn split_horizontally_action(&mut self) {
        let id_0 = self.get_new_id();
        let id_1 = self.get_new_id();
        let focused_node = self.explorers.get_mut(&self.focused_id).unwrap();
        let (mut node_0, node_1) = focused_node.split_horizontally(id_0, id_1);
        node_0.focused = false;
        self.explorers.insert(id_0, node_0);
        self.explorers.insert(id_1, node_1);
        self.focused_id = id_1;
    }

    pub fn delete_split(&mut self) -> bool {
        //get the focused_id node and seek its parent
        if self.focused_id == 0 {
            return true;
        }
        let focused_node = self.explorers.get(&self.focused_id).unwrap();
        let parent_id = match focused_node.parent {
            ParentRelationship::SomeParent(an_id, _) => an_id,
            ParentRelationship::NoParent => return true,
        };
        let parent_node = self.explorers.get(&parent_id).unwrap();
        let other_id = match parent_node.split {
            Split::Horizontal(id_0, id_1) => {
                if id_0 == self.focused_id {
                    id_1
                } else {
                    id_0
                }
            }
            Split::Vertical(id_0, id_1) => {
                if id_0 == self.focused_id {
                    id_1
                } else {
                    id_0
                }
            }

            Split::Single(_) => panic!("Impossible!"),
        };

        let other_node = self.explorers.get(&other_id).unwrap();

        let other_split = other_node.split.clone();
        match other_split {
            Split::Single(_) => {}
            Split::Horizontal(id_0, id_1) => {
                let explorer_1 = self.explorers.get_mut(&id_0).unwrap();
                explorer_1.parent = ParentRelationship::SomeParent(parent_id, 0);
                let explorer_2 = self.explorers.get_mut(&id_1).unwrap();
                explorer_2.parent = ParentRelationship::SomeParent(parent_id, 1);
            }
            Split::Vertical(id_0, id_1) => {
                let explorer_1 = self.explorers.get_mut(&id_0).unwrap();
                explorer_1.parent = ParentRelationship::SomeParent(parent_id, 0);
                let explorer_2 = self.explorers.get_mut(&id_1).unwrap();
                explorer_2.parent = ParentRelationship::SomeParent(parent_id, 1);
            }
        }
        let parent_node = self.explorers.get_mut(&parent_id).unwrap();
        parent_node.split = other_split;
        let mut current_node = parent_node.clone();
        self.explorers.remove(&self.focused_id);
        self.explorers.remove(&other_id);
        self.focused_id = parent_id;
        loop {
            match current_node.split {
                Split::Single(_) => {
                    self.focused_id = current_node.id;
                    // focus that ExplorerTable
                    self.focus();
                    break;
                }

                Split::Horizontal(id_0, _) => {
                    current_node = self.explorers.get(&id_0).unwrap().clone();
                }
                Split::Vertical(id_0, _) => {
                    current_node = self.explorers.get(&id_0).unwrap().clone();
                }
            }
        }
        false
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, sequence: Vec<KeyEvent>) {
        let string_seq = convert_sequence_to_string(sequence);
        let mut draw_map: HashMap<usize, Rect> = HashMap::new();
        self.get_drawable(frame, area, 0, &mut draw_map);
        self.last_layout = draw_map.clone();
        let _: Vec<_> = draw_map
            .iter()
            .map(|(key, value)| {
                let table = self.explorers.get_mut(key).unwrap();
                if let Split::Single(table) = &mut table.split {
                    let _ = table.draw(frame, *value, string_seq.clone());
                }
            })
            .collect();
    }
    pub fn get_drawable(
        &self,
        _frame: &mut Frame,
        area: Rect,
        id: usize,
        draw_map: &mut HashMap<usize, Rect>,
    ) {
        let explorer = self.explorers.get(&id).unwrap();
        let split = &explorer.split;
        match split {
            Split::Horizontal(id_0, id_1) => {
                let component_areas = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);
                self.get_drawable(_frame, component_areas[0], *id_0, draw_map);
                self.get_drawable(_frame, component_areas[1], *id_1, draw_map);
            }
            Split::Vertical(id_0, id_1) => {
                let component_areas = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);
                self.get_drawable(_frame, component_areas[0], *id_0, draw_map);
                self.get_drawable(_frame, component_areas[1], *id_1, draw_map);
            }
            Split::Single(_) => {
                draw_map.insert(id, area);
            }
        }
    }

    pub fn move_focus(&mut self, direction: SplitDirection) {
        if let Some(target_id) = self.get_neighbouring_node(direction) {
            let current_node = self.explorers.get_mut(&self.focused_id).unwrap();
            if let Split::Single(table) = &mut current_node.split {
                table.unfocus();
            }
            current_node.focused = false;

            let focused_node = self.explorers.get_mut(&target_id).unwrap();
            if let Split::Single(table) = &mut focused_node.split {
                table.focus();
            }
            focused_node.focused = true;
            self.focused_id = target_id;
        }
    }

    fn get_neighbouring_node(&self, direction: SplitDirection) -> Option<usize> {
        let rect = self.last_layout.get(&self.focused_id)?;
        let (current_x, current_y) = (rect.x, rect.y);

        let candidates: HashMap<usize, f32> = self
            .last_layout
            .iter()
            .filter_map(|(id, r)| {
                let relevant = match direction {
                    SplitDirection::Up => r.y < current_y,
                    SplitDirection::Down => r.y > current_y,
                    SplitDirection::Left => r.x < current_x,
                    SplitDirection::Right => r.x > current_x,
                };
                if relevant {
                    Some((
                        *id,
                        calculate_distance(
                            current_x.into(),
                            current_y.into(),
                            r.x.into(),
                            r.y.into(),
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect();

        candidates
            .into_iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(id, _)| id)
    }

    pub fn update_path(&mut self, path: PathBuf, filename: Option<String>) {
        delegate_to_focused!(self, update_path, path, filename);
    }

    pub fn get_current_path(&mut self) -> PathBuf {
        delegate_to_focused!(self, get_current_path)
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        delegate_to_focused!(self, switch_mode, mode);
    }

    pub fn focus(&mut self) {
        delegate_to_focused!(self, focus);
    }
    pub fn unfocus(&mut self) {}

    pub fn get_selected_files(&mut self) -> Option<Vec<PathBuf>> {
        delegate_to_focused!(self, get_selected_files)
    }

    pub fn select_directory(&mut self) -> Option<PathBuf> {
        delegate_to_focused!(self, select_directory)
    }

    pub fn get_selected(&mut self) -> Option<usize> {
        delegate_to_focused!(self, get_selected)
    }

    pub fn get_selected_string(&mut self) -> Option<String> {
        delegate_to_focused!(self, get_selected_string)
    }

    pub fn get_search_phrase(&mut self) -> Option<String> {
        delegate_to_focused!(self, get_search_phrase)
    }

    pub fn show_in_folder(&mut self, path: PathBuf) {
        delegate_to_focused!(self, show_in_folder, path);
    }

    pub fn next_search_result(&mut self) {
        delegate_to_focused!(self, next_search_result);
    }

    pub fn clear_search_query(&mut self) {
        delegate_to_focused!(self, clear_search_query);
    }

    pub fn next(&mut self) {
        delegate_to_focused!(self, next);
    }

    pub fn previous(&mut self) {
        delegate_to_focused!(self, previous);
    }

    pub fn jump_to_id(&mut self, id: usize) {
        delegate_to_focused!(self, jump_to_id, id);
    }
    pub fn refresh_contents(&mut self) {
        delegate_to_focused!(self, refresh_contents);
    }

    pub fn toggle_mark(&mut self) {
        delegate_to_focused!(self, toggle_mark);
    }

    pub fn get_marked_ids(&mut self) -> Option<Vec<usize>> {
        delegate_to_focused!(self, get_marked_ids)
    }

    pub fn get_affected_paths(&mut self) -> Option<Vec<PathBuf>> {
        delegate_to_focused!(self, get_affected_paths)
    }

    pub fn reset_marked_rows(&mut self) {
        delegate_to_focused!(self, reset_marked_rows);
    }

    pub fn find_elements(&self, query: &str) -> Vec<FileData> {
        match &self.explorers.get(&self.focused_id).unwrap().split {
            Split::Single(table) => table.find_elements(query),
            _ => panic!("Impossible!"),
        }
    }

    pub fn set_highlighting_rule(&mut self, highlighting_rule: GlobalStyling) {
        delegate_to_focused!(self, set_highlighting_rule, highlighting_rule);
    }
    pub fn set_plugin_display(&mut self, plugin_display: Option<String>) {
        delegate_to_focused!(self, set_plugin_display, plugin_display);
    }

    pub fn get_directory_history(&mut self) -> &mut DirectoryHistory {
        delegate_to_focused!(self, get_directory_history)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExplorerNode {
    pub id: usize,
    pub focused: bool,
    pub parent: ParentRelationship,
    pub split: Split,
}
impl ExplorerNode {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            focused: true,
            parent: ParentRelationship::NoParent,
            split: Split::Single(ExplorerTable::new()),
        }
    }

    pub fn new_with_explorer(id: usize, table: ExplorerTable) -> Self {
        Self {
            id,
            focused: true,
            parent: ParentRelationship::NoParent,
            split: Split::Single(table),
        }
    }

    pub fn split_vertically(&mut self, id_0: usize, id_1: usize) -> (Self, Self) {
        let explorer_table = match &self.split {
            Split::Single(table) => table.clone(),
            _ => panic!("Impossible!"),
        };
        self.split = Split::Vertical(id_0, id_1);
        let mut unfocused_explorer = explorer_table.clone();
        unfocused_explorer.unfocus();
        let mut node_0 = ExplorerNode::new_with_explorer(id_0, unfocused_explorer);
        node_0.parent = ParentRelationship::SomeParent(self.id, 0);
        let mut node_1 = ExplorerNode::new_with_explorer(id_1, explorer_table);
        node_1.parent = ParentRelationship::SomeParent(self.id, 1);
        (node_0, node_1)
    }
    pub fn split_horizontally(&mut self, id_0: usize, id_1: usize) -> (Self, Self) {
        let explorer_table = match &self.split {
            Split::Single(table) => table.clone(),
            _ => panic!("Impossible!"),
        };
        self.split = Split::Horizontal(id_0, id_1);
        let mut unfocused_explorer = explorer_table.clone();
        unfocused_explorer.unfocus();
        let mut node_0 = ExplorerNode::new_with_explorer(id_0, unfocused_explorer);
        node_0.parent = ParentRelationship::SomeParent(self.id, 0);
        let mut node_1 = ExplorerNode::new_with_explorer(id_1, explorer_table);
        node_1.parent = ParentRelationship::SomeParent(self.id, 1);
        (node_0, node_1)
    }
}

mod tests {
    use super::*;

    use ratatui::Frame;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;
    use ratatui::prelude::*;

    /// Trait to allow ExplorerManager to draw itself in a test environment
    pub trait DrawableTest {
        fn draw_test(&mut self, width: u16, height: u16);
    }
    impl DrawableTest for ExplorerManager {
        fn draw_test(&mut self, width: u16, height: u16) {
            let backend = TestBackend::new(width, height);
            let mut terminal = Terminal::new(backend).unwrap();
            terminal
                .draw(|frame: &mut Frame| {
                    let area = Rect::new(0, 0, width, height);
                    self.draw(frame, area, vec![]);
                })
                .unwrap();
        }
    }
    #[test]
    fn test_get_neighbouring_node() {
        let mut manager = ExplorerManager::new();
        manager.split_horizontally_action();
        // should be split in top and bottom now and focused on bottom
        // ids: 1, 2, focused on 2
        manager.split_vertically_action();
        // should be split in three now - top, bottom left, bottom right, and focused on bottom right
        // ids: 1, 3, 4, focused on 4
        manager.split_horizontally_action();
        // should be split in four now - top, bottom left, bottom right top, bottom right
        // bottom
        // ids: 1, 3, 5, 6 focused on 6
        // Need to call manager.draw() here - in order to create layout
        manager.draw_test(80, 24);
        assert_eq!(manager.focused_id, 6);
        assert_eq!(manager.get_neighbouring_node(SplitDirection::Up), Some(5));
        assert_eq!(manager.get_neighbouring_node(SplitDirection::Down), None);
        assert_eq!(manager.get_neighbouring_node(SplitDirection::Left), Some(3));
        assert_eq!(manager.get_neighbouring_node(SplitDirection::Right), None);
    }

    #[test]
    fn test_move_focus_right() {
        let mut manager = ExplorerManager::new();
        manager.split_vertically_action(); // now IDs 1 and 2, focused on 2
        manager.draw_test(80, 24); // ensures last_layout is populated

        assert_eq!(manager.focused_id, 2);
        manager.move_focus(SplitDirection::Left);
        assert_eq!(manager.focused_id, 1);
    }
}
