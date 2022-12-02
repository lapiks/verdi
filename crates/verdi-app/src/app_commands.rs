use verdi_game::prelude::GameState;

use crate::{
    commands::Command, 
    prelude::App
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
        Shutdown::name()
    }

    fn desc(&self) -> &'static str {
        "run the game"
    }

    fn execute(&self, app: &mut App) {
        if app.game_state == GameState::Paused {
            app.game_state = GameState::Running;
        }
        else if app.game_state == GameState::Stopped {
            app.game_state = GameState::Start;
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
        Shutdown::name()
    }

    fn desc(&self) -> &'static str {
        "stop the game"
    }

    fn execute(&self, app: &mut App) {
        app.game_state = GameState::Stopped;
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
        Shutdown::name()
    }

    fn desc(&self) -> &'static str {
        "pause the game"
    }

    fn execute(&self, app: &mut App) {
        app.game_state = GameState::Paused;
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
        Shutdown::name()
    }

    fn desc(&self) -> &'static str {
        "open 3D modeler"
    }

    fn execute(&self, app: &mut App) {

    }
}

impl ShowModeler {
    pub fn name() -> &'static str {
        "show_modeler"
    }
}