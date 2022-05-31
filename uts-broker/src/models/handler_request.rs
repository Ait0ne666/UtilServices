use serde::Deserialize;




#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Actions {
    LOG
}



#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Warning,
    Info,
    Error
}

impl Into<String> for Severity {
    fn into(self) -> String{
        match self {
            Severity::Warning => {
                "Warning".to_string()
            },
            Severity::Info => {
                "Info".to_string()
            },
            Severity::Error => {
                "Error".to_string()
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone,  PartialEq)]
pub struct LogData {
    pub severity: Severity,
    pub message: String
}



#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HandlerRequest {
    pub action: Actions,
    pub log: Option<LogData>,
}