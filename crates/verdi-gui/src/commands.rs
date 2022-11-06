pub trait Command {
    fn name() -> &'static str;
    fn desc() -> &'static str;
}

pub struct Load;

impl Command for Load {
    fn name() -> &'static str {
        "load"
    }

    fn desc() -> &'static str {
        "load a game folder"
    }
}

pub struct Help;

impl Command for Help {
    fn name() -> &'static str {
        "help"
    }

    fn desc() -> &'static str {
        "pint available commands"
    }
}