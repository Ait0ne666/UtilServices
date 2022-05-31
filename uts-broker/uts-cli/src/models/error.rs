use std::error::Error;
use std::fmt;



#[derive(Debug)]
pub struct CliError {
    details: String
}

impl CliError {
    fn new(msg: &str) -> CliError {
        CliError{details: msg.to_string()}
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        &self.details
    }
}



impl From<diesel::result::Error> for CliError {
    fn from(e: diesel::result::Error) -> Self {
        Self { details: e.to_string() }
    }
}



impl From<&str> for CliError {
    fn from(e: &str) -> Self {
        Self { details: e.to_string() }
    }
}


impl From<String> for CliError {
    fn from(e: String) -> Self {
        Self { details: e }
    }
}