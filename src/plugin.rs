pub mod plugin_popup;
pub trait Plugin {
    fn display_details(&self) -> String {
        String::from("Not implemented")
    }
}
