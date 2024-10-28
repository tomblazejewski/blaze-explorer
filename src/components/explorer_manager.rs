use core::panic;
use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::cmp::{Eq, PartialEq};
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::Weak;
use std::usize;
use std::{path::PathBuf, rc::Rc};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use super::explorer_table::ExplorerTable;
use crate::components::Component;
use crate::mode::Mode;

macro_rules! delegate_to_focused {
    ($self:ident, $method:ident $(, $args:expr )* ) => {
        match $self.split {
            Split::Horizontal(ref mut top, ref mut bottom) => {
                if top.focused {
                    top.$method($($args.clone()),*)
                } else {
                    bottom.$method($($args),*)
                }
            }
            Split::Vertical(ref mut left, ref mut right) => {
                if left.focused {
                    left.$method($($args.clone()),*)
                } else {
                    right.$method($($args),*)
                }
            }
            Split::Single(ref mut table) => {
                table.$method($($args),*)
            }
        }
    };
}

#[derive(Clone, Debug)]
pub enum ParentRelationship {
    SomeParent(usize, usize),
    NoParent,
}

#[derive(Debug, Clone)]
pub enum Split {
    Horizontal(usize, usize),
    Vertical(usize, usize),
    Single(ExplorerTable),
}

#[derive(PartialEq, Eq, Clone)]
pub enum SplitPreference {
    Horizontal,
    Vertical,
}

pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}

pub enum SplitResult {
    SomeResult(usize, SplitPreference, VecDeque<usize>, usize),
    NoResult,
}

pub struct ExplorerDirector {
    pub explorers: HashMap<usize, ExplorerNode>,
    pub focused_id: usize,
    pub next_id: usize,
}

impl ExplorerDirector {
    pub fn new() -> Self {
        let mut explorer_map = HashMap::new();
        explorer_map.insert(0, ExplorerNode::new(0));
        Self {
            explorers: explorer_map,
            focused_id: 0,
            next_id: 1,
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
        let (node_0, node_1) = focused_node.split_vertically(id_0, id_1);
        self.explorers.insert(id_0, node_0);
        self.explorers.insert(id_1, node_1);
        self.focused_id = id_1;
    }

    pub fn split_horizontally_action(&mut self) {
        let id_0 = self.get_new_id();
        let id_1 = self.get_new_id();
        let focused_node = self.explorers.get_mut(&self.focused_id).unwrap();
        let (node_0, node_1) = focused_node.split_horizontally(id_0, id_1);
        self.explorers.insert(id_0, node_0);
        self.explorers.insert(id_1, node_1);
        self.focused_id = id_1;
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let mut draw_map: HashMap<usize, Rect> = HashMap::new();
        self.get_drawable(frame, area, 0, &mut draw_map);
        for (key, value) in draw_map.iter() {
            let table = self.explorers.get_mut(&key).unwrap();
            match &mut table.split {
                Split::Single(table) => {
                    let _ = table.draw(frame, *value);
                }
                _ => {}
            }
        }
    }
    pub fn get_drawable(
        &self,
        frame: &mut Frame,
        area: Rect,
        id: usize,
        draw_map: &mut HashMap<usize, Rect>,
    ) {
        let explorer = self.explorers.get(&id).unwrap();
        let split = &explorer.split;
        match split {
            Split::Horizontal(id_0, id_1) => {
                let component_areas = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);
                self.get_drawable(frame, component_areas[0], id_0.clone(), draw_map);
                self.get_drawable(frame, component_areas[1], id_1.clone(), draw_map);
            }
            Split::Vertical(id_0, id_1) => {
                let component_areas = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);
                self.get_drawable(frame, component_areas[0], id_0.clone(), draw_map);
                self.get_drawable(frame, component_areas[1], id_1.clone(), draw_map);
            }
            Split::Single(_) => {
                draw_map.insert(id, area);
            }
        }
    }
}

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
        self.split = Split::Vertical(id_0, id_1);
        let explorer_table = match &self.split {
            Split::Single(table) => table.clone(),
            _ => panic!("Impossible!"),
        };
        let mut node_0 = ExplorerNode::new_with_explorer(id_0, explorer_table.clone());
        node_0.parent = ParentRelationship::SomeParent(self.id, 0);
        let mut node_1 = ExplorerNode::new_with_explorer(id_1, explorer_table);
        node_1.parent = ParentRelationship::SomeParent(self.id, 1);
        (node_0, node_1)
    }
    pub fn split_horizontally(&mut self, id_0: usize, id_1: usize) -> (Self, Self) {
        self.split = Split::Horizontal(id_0, id_1);
        let explorer_table = match &self.split {
            Split::Single(table) => table.clone(),
            _ => panic!("Impossible!"),
        };
        let mut node_0 = ExplorerNode::new_with_explorer(id_0, explorer_table.clone());
        node_0.parent = ParentRelationship::SomeParent(self.id, 0);
        let mut node_1 = ExplorerNode::new_with_explorer(id_1, explorer_table);
        node_1.parent = ParentRelationship::SomeParent(self.id, 1);
        (node_0, node_1)
    }
}
#[derive(Clone, Debug)]
pub struct ExplorerManager {
    focused: bool,
    split: Split,
    parent: ParentRelationship,
}

impl ExplorerManager {
    pub fn new() -> Self {
        Self {
            focused: true,
            split: Split::Single(ExplorerTable::new()),
            parent: ParentRelationship::NoParent,
        }
    }

    pub fn new_child(&self, index: usize) -> Self {
        let self_reference = RefCell::new(self.to_owned());
        let explorer_table = match &self.split {
            Split::Single(table) => table.clone(),
            _ => panic!("Impossible!"),
        };
        Self {
            focused: false,
            split: Split::Single(explorer_table),
            parent: ParentRelationship::SomeParent(index, Rc::downgrade(&Rc::new(self_reference))),
        }
    }
    pub fn new_child_focused(&self, index: usize) -> Self {
        let self_reference = RefCell::new(self.to_owned());
        let explorer_table = match &self.split {
            Split::Single(table) => table.clone(),
            _ => panic!("Impossible!"),
        };
        Self {
            focused: true,
            split: Split::Single(explorer_table),
            parent: ParentRelationship::SomeParent(index, Rc::downgrade(&Rc::new(self_reference))),
        }
    }

    pub fn split_vertically_action(&mut self) {
        match self.split {
            Split::Horizontal(ref mut top, ref mut bottom) => match top.focused {
                true => top.split_vertically_action(),
                false => bottom.split_vertically_action(),
            },
            Split::Vertical(ref mut left, ref mut right) => match left.focused {
                true => left.split_vertically_action(),
                false => right.split_vertically_action(),
            },
            Split::Single(ref mut table) => {
                self.split_vertically();
            }
        }
    }

    pub fn split_horizontally_action(&mut self) {
        match self.split {
            Split::Horizontal(ref mut top, ref mut bottom) => match top.focused {
                true => top.split_horizontally_action(),
                false => bottom.split_horizontally_action(),
            },
            Split::Vertical(ref mut left, ref mut right) => match left.focused {
                true => left.split_horizontally_action(),
                false => right.split_horizontally_action(),
            },
            Split::Single(ref mut table) => {
                self.split_horizontally();
            }
        }
    }
    pub fn split_vertically(&mut self) {
        let manager_0 = self.new_child(0);
        let manager_1 = self.new_child_focused(1);
        self.split = Split::Vertical(Box::new(manager_0), Box::new(manager_1));
    }
    pub fn split_horizontally(&mut self) {
        let manager_0 = self.new_child(0);
        let manager_1 = self.new_child_focused(1);
        self.split = Split::Horizontal(Box::new(manager_0), Box::new(manager_1));
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        match &mut self.split {
            Split::Horizontal(top, bottom) => {
                let component_areas = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);
                top.draw(frame, component_areas[0]);
                bottom.draw(frame, component_areas[1]);
            }
            Split::Vertical(left, right) => {
                let component_areas = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);
                left.draw(frame, component_areas[0]);
                right.draw(frame, component_areas[1]);
            }
            Split::Single(table) => {
                table.draw(frame, area);
            }
        }
    }

    /// Recur over splits until encountering a single split.
    /// Uses [first_index] to focus on the utmost parent of what is meant to be eventually focused
    /// on
    /// When entering a leading_split, uses the stack to pick between the two explorermanagers
    /// available
    /// Otherwise, uses the default_index
    pub fn inside(
        &mut self,
        first_index: Option<usize>,
        leading_split: SplitPreference,
        mut stack: VecDeque<usize>,
        default_index: usize,
    ) -> Option<&ExplorerTable> {
        self.focused = true;
        match first_index.is_some() {
            true => {
                let index = first_index.unwrap();
                match &mut self.split {
                    Split::Horizontal(top, bottom) => {
                        if index == 0 {
                            top.inside(None, leading_split, stack, default_index);
                        } else {
                            bottom.inside(None, leading_split, stack, default_index);
                        }
                    }
                    Split::Vertical(left, right) => {
                        if index == 0 {
                            left.inside(None, leading_split, stack, default_index);
                        } else {
                            right.inside(None, leading_split, stack, default_index);
                        }
                    }
                    Split::Single(table) => {
                        panic!("Impossible for the first index to exist and to be within a single split.")
                    }
                }
            }
            false => match leading_split {
                SplitPreference::Horizontal => match &mut self.split {
                    Split::Horizontal(ref mut top, ref mut bottom) => {
                        let which_split = stack.pop_front().unwrap_or(0);

                        if which_split == 0 {
                            top.inside(None, leading_split, stack, default_index);
                        } else {
                            bottom.inside(None, leading_split, stack, default_index);
                        }
                    }
                    Split::Vertical(ref mut left, ref mut right) => {
                        if default_index == 0 {
                            left.inside(None, leading_split, stack, default_index);
                        } else {
                            right.inside(None, leading_split, stack, default_index);
                        }
                    }
                    Split::Single(table) => {
                        //reached a single table - focus
                        table.focus();
                    }
                },
                SplitPreference::Vertical => match &mut self.split {
                    Split::Horizontal(ref mut top, ref mut bottom) => {
                        if default_index == 0 {
                            top.inside(None, leading_split, stack, default_index);
                        } else {
                            bottom.inside(None, leading_split, stack, default_index);
                        }
                    }
                    Split::Vertical(ref mut left, ref mut right) => {
                        let which_index = stack.pop_front().unwrap_or(0);
                        if which_index == 0 {
                            left.inside(None, leading_split, stack, default_index);
                        } else {
                            right.inside(None, leading_split, stack, default_index);
                        }
                    }
                    Split::Single(table) => {
                        //reached a single table - focus
                        table.focus();
                    }
                },
            },
        }
        return None;
    }

    pub fn outside(
        &mut self,
        direction: SplitDirection,
        mut stack: VecDeque<usize>,
    ) -> SplitResult {
        let (desired_index, split_preference) = match direction {
            SplitDirection::Up => (0, SplitPreference::Vertical),
            SplitDirection::Down => (1, SplitPreference::Vertical),
            SplitDirection::Left => (0, SplitPreference::Horizontal),
            SplitDirection::Right => (0, SplitPreference::Horizontal),
        };

        match &self.parent {
            ParentRelationship::SomeParent(index, ref parent) => {
                let parent = parent.upgrade();
                if let Some(parent_rc) = parent {
                    let index = *index;
                    let split = parent_rc.borrow().split.clone();
                    match (index == desired_index, split_preference.clone(), split) {
                        (true, SplitPreference::Horizontal, Split::Horizontal(top, bottom)) => {
                            //this is the right index, and the right split - horizontal.
                            //stop here and return the required values
                            let default_index = if index == 0 { 1 } else { 0 };
                            SplitResult::SomeResult(index, split_preference, stack, default_index)
                        }
                        (true, SplitPreference::Vertical, Split::Vertical(top, bottom)) => {
                            //this is the right index, and the right split - vertical.
                            //stop here and return the required values
                            let default_index = if index == 0 { 1 } else { 0 };
                            SplitResult::SomeResult(index, split_preference, stack, default_index)
                        }
                        (false, SplitPreference::Horizontal, Split::Horizontal(top, bottom)) => {
                            let mut parent_mut = RefCell::borrow_mut(&parent_rc);
                            parent_mut.outside(direction, stack)
                        }
                        (false, SplitPreference::Vertical, Split::Vertical(left, right)) => {
                            let mut parent_mut = RefCell::borrow_mut(&parent_rc);
                            parent_mut.outside(direction, stack)
                        }
                        //mismatched splits, regardless of the index matching
                        (_, SplitPreference::Vertical, Split::Horizontal(top, bottom)) => {
                            stack.push_front(index);
                            let mut parent_mut = RefCell::borrow_mut(&parent_rc);
                            parent_mut.outside(direction, stack)
                        }
                        (_, SplitPreference::Horizontal, Split::Vertical(left, right)) => {
                            stack.push_front(index);
                            let mut parent_mut = RefCell::borrow_mut(&parent_rc);
                            parent_mut.outside(direction, stack)
                        }

                        (_, _, Split::Single(table)) => {
                            panic!("Impossible for a parent to have a single split")
                        }
                    }
                } else {
                    panic!("No parent");
                }
            }
            ParentRelationship::NoParent => SplitResult::NoResult,
        }
    }

    fn focus_table(&mut self, table: &mut ExplorerTable) {
        table.focus();
    }
    fn perform_focus(&mut self, split_direction: SplitDirection) {
        self.defocus();
        let out_result = self.outside(split_direction, VecDeque::new());
        if let SplitResult::SomeResult(first_index, leading_split, stack, default_index) =
            out_result
        {
            self.inside(
                Some(first_index),
                leading_split,
                VecDeque::new(),
                default_index,
            );
        }
    }
    pub fn go_up(&mut self) {
        self.perform_focus(SplitDirection::Up)
    }
    pub fn go_down(&mut self) {
        self.perform_focus(SplitDirection::Down)
    }
    pub fn go_left(&mut self) {
        self.perform_focus(SplitDirection::Left)
    }
    pub fn go_right(&mut self) {
        self.perform_focus(SplitDirection::Right)
    }

    fn defocus(&mut self) {
        self.focused = false;
        match self.split.clone() {
            Split::Horizontal(mut top, mut bottom) => {
                top.defocus();
                bottom.defocus();
            }
            Split::Vertical(mut left, mut right) => {
                left.defocus();
                right.defocus();
            }
            Split::Single(mut table) => {
                table.unfocus();
            }
        }
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
    pub fn unfocus(&mut self) {
        delegate_to_focused!(self, unfocus);
    }

    pub fn get_selected_files(&mut self) -> Option<Vec<PathBuf>> {
        delegate_to_focused!(self, get_selected_files)
    }

    pub fn select_directory(&mut self) -> Option<PathBuf> {
        delegate_to_focused!(self, select_directory)
    }

    pub fn get_selected(&mut self) -> Option<usize> {
        delegate_to_focused!(self, get_selected)
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

    pub fn update_search_query(&mut self, query: String) {
        delegate_to_focused!(self, update_search_query, query);
    }

    pub fn next(&mut self) {
        delegate_to_focused!(self, next);
    }

    pub fn previous(&mut self) {
        delegate_to_focused!(self, previous);
    }
}
