
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowErrorType {
    Initialization,
    Color
}

impl std::fmt::Display for WindowErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowErrorType::Initialization => write!(f, "Initialization"),
            WindowErrorType::Color => write!(f, "Color")
        }
    }
}

#[derive(Debug, Clone)]
pub struct WindowError {
    error_type: WindowErrorType,
    error_message: String
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Window {:?} Error: {:?}", self.error_type, self.error_message)
    }
}

impl WindowError {
    pub fn new(err_type: WindowErrorType, message: String) -> Self {
        WindowError {
            error_type: err_type,
            error_message: message
        }
    }
}
