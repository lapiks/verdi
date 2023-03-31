use verdi_system::prelude::SystemState;

use crate::{
    app::App,
    commands::Command, 
};

#[derive(Clone)]
pub struct Load {
    pub folder: String,
}

impl Command for Load {
    fn name(&self) -> &'static str {
        Load::name()
    }

    fn desc(&self) -> &'static str {
        "load a game folder"
    }

    fn execute(&self, app: &mut App) {
        app.load_game(self.folder.clone());
    }
}

impl Load {
    pub fn name() -> &'static str {
        "load"
    }
}

#[derive(Clone)]
pub struct Help;

impl Command for Help {
    fn name(&self) -> &'static str {
        Help::name()
    }

    fn desc(&self) -> &'static str {
        "print available commands"
    }

    fn execute(&self, app: &mut App) {
        
    }
}

impl Help {
    pub fn name() -> &'static str {
        "help"
    }
}

#[derive(Clone)]
pub struct Shutdown;

impl Command for Shutdown {
    fn name(&self) -> &'static str {
        Shutdown::name()
    }

    fn desc(&self) -> &'static str {
        "exits the program"
    }

    fn execute(&self, app: &mut App) {
        app.shutdown();
    }
}

impl Shutdown {
    pub fn name() -> &'static str {
        "shutdown"
    }
}

#[derive(Clone)]
pub struct Run;

impl Command for Run {
    fn name(&self) -> &'static str {
        Run::name()
    }

    fn desc(&self) -> &'static str {
        "run the game"
    }

    fn execute(&self, app: &mut App) {
        if let Some(game) = app.get_game_mut() {
            if game.state == SystemState::Paused {
                game.state = SystemState::Running;
            }
            else if game.state == SystemState::Loaded {
                game.state = SystemState::Starting;
            }
        }
    }
}

impl Run {
    pub fn name() -> &'static str {
        "run"
    }
}

#[derive(Clone)]
pub struct Stop;

impl Command for Stop {
    fn name(&self) -> &'static str {
        Stop::name()
    }

    fn desc(&self) -> &'static str {
        "stop the game"
    }

    fn execute(&self, app: &mut App) {
        if let Some(game) = app.get_game_mut() {
            game.state = SystemState::Stopped;
        }
    }
}

impl Stop {
    pub fn name() -> &'static str {
        "stop"
    }
}

#[derive(Clone)]
pub struct Paused;

impl Command for Paused {
    fn name(&self) -> &'static str {
        Paused::name()
    }

    fn desc(&self) -> &'static str {
        "pause the game"
    }

    fn execute(&self, app: &mut App) {
        if let Some(game) = app.get_game_mut() {
            game.state = SystemState::Paused;
        }
    }
}

impl Paused {
    pub fn name() -> &'static str {
        "pause"
    }
}

#[derive(Clone)]
pub struct ShowModeler;

impl Command for ShowModeler {
    fn name(&self) -> &'static str {
        ShowModeler::name()
    }

    fn desc(&self) -> &'static str {
        "open the 3D modeler"
    }

    fn execute(&self, app: &mut App) {

    }
}

impl ShowModeler {
    pub fn name() -> &'static str {
        "show_modeler"
    }
}

#[derive(Clone)]
pub struct ShowEditor;

impl Command for ShowEditor {
    fn name(&self) -> &'static str {
        ShowEditor::name()
    }

    fn desc(&self) -> &'static str {
        "open the world editor"
    }

    fn execute(&self, app: &mut App) {
        app.show_editor = true;
        if let Some(editor) = app.get_editor_mut() {
            if editor.state == SystemState::Loaded {
                editor.state = SystemState::Starting;
            }
        }
    }
}

impl ShowEditor {
    pub fn name() -> &'static str {
        "show_editor"
    }
}