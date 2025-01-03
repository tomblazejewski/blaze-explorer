use crate::{
    action::{Action, PopupAction},
    line_entry::LineEntry,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub contents: String,
    pub prefix: String,
    pub suffix: String,
}

impl LineEntry for Query {
    fn pop_contents(&mut self) -> String {
        self.contents.drain(..).collect()
    }

    fn append_char(&mut self, c: char) {
        self.contents.push(c);
    }

    fn clear_contents(&mut self) {
        self.contents.clear();
    }

    fn drop_char(&mut self) {
        self.contents.pop();
    }

    fn remove_char(&mut self) -> Option<Action> {
        self.contents.pop();
        Some(Action::PopupAct(PopupAction::UpdateSearchQuery(
            self.contents.clone(),
        )))
    }

    fn get_contents(&self) -> String {
        format!("{}{}{}", self.prefix, self.contents, self.suffix)
    }

    fn set_contents(&mut self, _contents: String) {
        // Worry about this implementation later on
        panic!("Should not be called")
    }
}

impl Query {
    pub fn default() -> Self {
        Self {
            contents: String::new(),
            prefix: String::new(),
            suffix: String::new(),
        }
    }
    pub fn new(prefix: String, suffix: String) -> Self {
        Self {
            contents: String::new(),
            prefix,
            suffix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_contents() {
        let query = Query::new("prefix".to_string(), "suffix".to_string());
        assert_eq!(query.get_contents(), "prefixsuffix");
    }

    #[test]
    fn test_append_char() {
        let mut query = Query::default();
        query.append_char('a');
        assert_eq!(query.get_contents(), "a");
    }

    #[test]
    fn test_pop_contents() {
        let mut query = Query::new("prefix".to_string(), "suffix".to_string());
        query.append_char('a');
        assert_eq!(query.pop_contents(), "a");
    }

    #[test]
    fn test_drop_char() {
        let mut query = Query::new("prefix".to_string(), "suffix".to_string());
        query.append_char('a');
        query.drop_char();
        assert_eq!(query.get_contents(), "prefixsuffix");
    }

    #[test]
    fn test_remove_char() {
        let mut query = Query::new("prefix".to_string(), "suffix".to_string());
        query.append_char('a');
        query.remove_char();
        assert_eq!(query.get_contents(), "prefixsuffix");
    }

    #[test]
    fn test_clear_contents() {
        let mut query = Query::new("prefix".to_string(), "suffix".to_string());
        query.append_char('a');
        query.clear_contents();
        assert_eq!(query.get_contents(), "prefixsuffix");
    }
}
