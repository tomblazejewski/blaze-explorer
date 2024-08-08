use ratatui::crossterm::event::KeyEvent;

#[derive(Clone, Debug)]
pub enum Action {
    Quit,
    ChangeDirectory(String), //switch to a directory specified
    ParentDirectory,         //go up
    Key(KeyEvent),           //this is just used by key_tracker to know what was clicked
    Noop,                    //do nothing (or wait)
    SelectUp,                //go up by one entry
    SelectDown,              //go down by one entry
    EscapeSequence,          //cancel all keys entered in sequence
    SelectDirectory, //attempt to open a directory under cursor (note this itself does not cause to
    //CHANGE)
    ClearAndKey(KeyEvent), //clear the key sequence and enter a key
    Linger(u32),           //keep the key displayed for n next frames
}
