Plan for undo/redo and go back/forth functionalities

From undo/redo perspective, there is little that needs to have the undo functionality.
Seems like the new Command Pattern is better than Action.
It should be used for all actions, but as is for now, there is no action to be undone.

The future actions to be undone are of sort: delete, rename, cut, add/paste.

That being said there are plenty of actions that change the directory of the file explorer.
This, rather than being undone/redone could be going back/forth to switch between the different directories.
Then, each directory has got its own undo/redo stack.

The plan therefore is the following:

- rewrite all actions to use the Command Pattern. The undo function will exist but it will not do anything for all functions so far.
- add the functionality of going back and forth between directories first.
- add a HistoryStack to the App instance.
- add functions that implement undo/redo and try to push them to the HistoryStack for the right directory.
