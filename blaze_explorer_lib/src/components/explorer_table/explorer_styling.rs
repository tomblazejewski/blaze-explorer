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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explorer_style() {
        let gs = GlobalStyling::HighlightSearch("abc".into());
        let vf = VisualFunction::Toggle;

        let mut explorer_style = ExplorerStyle::new(gs.clone(), vf.clone());

        assert_eq!(explorer_style.highlighting_rule(), gs);
        assert_eq!(explorer_style.visual_function(), vf);

        let gs_2 = GlobalStyling::None;
        let vf_2 = VisualFunction::None;

        explorer_style.set_highlighting_rule(gs_2.clone());
        explorer_style.set_visual_function(vf_2.clone());
        assert_eq!(explorer_style.highlighting_rule(), gs_2);
        assert_eq!(explorer_style.visual_function(), vf_2);
    }
}
