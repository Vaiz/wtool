#[derive(Debug)]
pub struct ErrorString {
    msg: String,
}

impl ErrorString {
    pub fn new(msg: String) -> Box<Self> {
        Box::new(Self { msg })
    }
}

impl std::fmt::Display for ErrorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ErrorString {}


#[derive(Debug)]
pub struct ErrorStr {
    msg: &'static str,
}

impl ErrorStr {
    pub fn new(msg: &'static str) -> Box<Self> {
        Box::new(Self { msg })
    }
}

impl std::fmt::Display for ErrorStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ErrorStr {}