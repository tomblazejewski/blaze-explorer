pub mod command_history;
pub mod directory_history;
pub trait HistoryStack<T> {
    fn undo(&mut self) -> Option<T>;
    fn redo(&mut self) -> Option<T>;
    fn perform(&mut self, element: T);
    fn new() -> Self;
}
