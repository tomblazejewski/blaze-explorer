use crate::{components::Component, input_machine::InputMachine};

pub enum PopUp<T>
where
    T: Component,
{
    None,
    PopUp(PopUpWindow<T>),
}
struct PopUpWindow<T>
where
    T: Component,
{
    input_machine: InputMachine,
    component: T,
}
