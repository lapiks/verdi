pub trait Command {
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn execute(&self);
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

    fn execute(&self) {
        
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

    fn execute(&self) {
        
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

    fn execute(&self) {
        
    }
}

impl Shutdown {
    pub fn name() -> &'static str {
        "shutdown"
    }
}