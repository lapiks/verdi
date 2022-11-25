use crate::app::App;

pub trait Command {
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn execute(&self, cmd: &str, app: &mut App);
}

pub struct Load {

}

impl Command for Load {
    fn name(&self) -> &'static str {
        Load::name()
    }

    fn desc(&self) -> &'static str {
        "load a game folder"
    }

    fn execute(&self, cmd: &str, app: &mut App) {
        app.load_game(cmd);
    }
}

impl Load {
    pub fn name() -> &'static str {
        "load"
    }
}

pub struct Help;

impl Command for Help {
    fn name(&self) -> &'static str {
        Help::name()
    }

    fn desc(&self) -> &'static str {
        "print available commands"
    }

    fn execute(&self, cmd: &str, app: &mut App) {
        
    }
}

impl Help {
    pub fn name() -> &'static str {
        "help"
    }
}

pub struct Shutdown;

impl Command for Shutdown {
    fn name(&self) -> &'static str {
        Shutdown::name()
    }

    fn desc(&self) -> &'static str {
        "exits the program"
    }

    fn execute(&self, cmd: &str, app: &mut App) {
        
    }
}

impl Shutdown {
    pub fn name() -> &'static str {
        "shutdown"
    }
}