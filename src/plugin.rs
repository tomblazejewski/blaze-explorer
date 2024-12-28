pub mod plugin_popup;
pub mod telescope;
pub trait Plugin {
    fn display_details(&self) -> String {
        String::from("Not implemented")
    }
}
