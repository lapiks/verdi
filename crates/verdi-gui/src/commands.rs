pub trait Command {
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
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
        "pint available commands"
    }
}

impl Help {
    pub fn name() -> &'static str {
        "help"
    }
}