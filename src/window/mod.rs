
#[derive(Debug, Clone, PartialEq)]
pub struct Dimensions {
    pub x: i32,
    pub y: i32
}

impl Default for Dimensions {
    fn default() -> Self {
        Dimensions { x: 0, y: 0 }
    }
}

impl std::fmt::Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Dimensions {
    fn from(value: (i32, i32)) -> Self {
        Dimensions { x: value.0, y: value.1 }
    }
}


pub struct Window {
    handle: *mut i8
}

impl Window {
    pub fn new() -> Self {
        
    }
}
