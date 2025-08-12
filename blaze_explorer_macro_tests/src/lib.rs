#[cfg(test)]
mod tests {
    use blaze_explorer_lib::plugin::{plugin_helpers::DummyPluginPopUp, plugin_popup::PluginPopUp};
    use blaze_explorer_macros::quit_popup;

    use super::*;

    #[test]
    fn test_quit_popup_macro() {
        use blaze_explorer_lib::app::App;
        use blaze_explorer_macros::quit_popup;
        trait DummyTrait {
            fn dummy_function(&mut self);
        }
        impl DummyTrait for App {
            #[quit_popup]
            fn dummy_function(&mut self) {
                // This function should have app.try_drop_popup() injected at the start
            }
        }

        let mut app = App::new_test().unwrap();
        let popup = DummyPluginPopUp::new();
        app.attach_popup(Box::new(popup));

        app.dummy_function();
        if let Some(popup) = app.popup {
            assert!(popup.should_quit());
        } else {
            panic!("Popup should not be None");
        }
    }
}
