pub(crate) mod errors;

pub trait Command {
    fn create() -> Box<Self> where Self: Sized;
    fn name() -> &'static str where Self: Sized;
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b>;
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Dispatcher {
    m_commands: std::collections::HashMap<&'static str, Box<dyn Command>>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            m_commands: std::collections::HashMap::new(),
        }
    }
    pub fn add_cmd<T: Command + 'static>(&mut self) -> &mut Dispatcher {
        let result = self.m_commands.insert(T::name(), T::create());
        if result.is_some() {
            println!("Failed to add command: {}", T::name());
        }
        self
    }
    pub fn fill_subcommands<'a, 'b>(&self, mut app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        for (_, cmd) in &self.m_commands {
            app = cmd.fill_subcommand(app);
        }
        app
    }
    pub fn run(&self, cmd_name: &str, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = self.m_commands.get(cmd_name);
        if cmd.is_none() {
            return Err(errors::ErrorString::new(format!("Command '{}' has not found. Use '--help' to print all commands", cmd_name)));
        }

        cmd.unwrap().run(args)
    }
}