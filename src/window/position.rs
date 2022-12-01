

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Coordinates {
    fn from(value: (i32, i32)) -> Self {
        Coordinates { x: value.0, y: value.1 }
    }
}


#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Boundary {
    position: Coordinates,
    extent: Coordinates
}

impl std::fmt::Display for Boundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x: {}, y: {}, width: {}, height: {}",
            self.position.x,
            self.position.y,
            self.extent.x,
            self.extent.y
        )
    }
}

impl From<(i32, i32, i32, i32)> for Boundary {
    fn from(value: (i32, i32, i32, i32)) -> Self {
        let position = (value.0, value.1);
        let extent = (value.2, value.3);

        Boundary {
            position: Coordinates::from(position),
            extent: Coordinates::from(extent)
        }
    }
}