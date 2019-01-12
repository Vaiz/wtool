pub trait Command {
    fn create() -> Box<Self> where Self: Sized;
    fn name() -> &'static str where Self: Sized;
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b>;
    fn run(&self, args: Option<&clap::ArgMatches>);
}

pub struct Dispatcher {
    m_commands: std::collections::HashMap<&'static str, Box<Command>>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            m_commands: std::collections::HashMap::new(),
        }
    }
    pub fn add_cmd<T: Command + 'static>(&mut self) {
        assert!(self.m_commands.insert(T::name(), T::create()).is_none());
    }
    pub fn fill_subcommands<'a, 'b>(&self, mut app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        for (_, cmd) in &self.m_commands {
            app = cmd.fill_subcommand(app);
        }
        app
    }
    pub fn run(&self, cmd: &str, args: Option<&clap::ArgMatches>) {
        let cmd = self.m_commands.get(cmd).unwrap();
        cmd.run(args);
    }
}