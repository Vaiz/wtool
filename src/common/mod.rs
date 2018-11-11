pub trait Command {
    fn create() -> Box<Self> where Self: Sized;
    fn name() -> &'static str where Self: Sized;
    fn run(&self);
}

pub struct Dispatcher {
    m_commands : std::collections::HashMap<&'static str, Box<Command>>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher{
            m_commands : std::collections::HashMap::new(),
        }
    }
    pub fn add_cmd<T: Command + 'static>(&mut self) {
        assert!(self.m_commands.insert(T::name(), T::create()).is_none());
    }
    pub fn run(&self, cmd : &str) {
        let cmd = self.m_commands.get(cmd).unwrap();
        cmd.run();
    }
}