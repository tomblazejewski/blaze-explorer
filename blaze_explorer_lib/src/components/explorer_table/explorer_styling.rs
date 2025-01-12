use super::GlobalStyling;

/// Determines how the curson behaves when in the Visual mode.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualFunction {
    Trailing(u8), // Scrolling up/down highlights the rows on the go
    Toggle,       // Scrolling does not highlight, can toggle current selection
    None,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExplorerStyle {
    highlighting_rule: GlobalStyling,
    visual_function: VisualFunction,
}

impl ExplorerStyle {
    pub fn new(highlighting_rule: GlobalStyling, visual_function: VisualFunction) -> Self {
        Self {
            highlighting_rule,
            visual_function,
        }
    }

    pub fn highlighting_rule(&self) -> GlobalStyling {
        self.highlighting_rule.clone()
    }

    pub fn visual_function(&self) -> VisualFunction {
        self.visual_function.clone()
    }

    pub fn set_visual_function(&mut self, visual_function: VisualFunction) {
        self.visual_function = visual_function;
    }

    pub fn set_highlighting_rule(&mut self, highlighting_rule: GlobalStyling) {
        self.highlighting_rule = highlighting_rule;
    }
}

impl Default for ExplorerStyle {
    fn default() -> Self {
        Self {
            highlighting_rule: GlobalStyling::None,
            visual_function: VisualFunction::None,
        }
    }
}
