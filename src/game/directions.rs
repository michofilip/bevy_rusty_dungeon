#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum GridDirection {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl GridDirection {
    pub const ALL: [Self; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];

    pub const CARDINAL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn is_cardinal(&self) -> bool {
        Self::CARDINAL.contains(self)
    }

    pub fn rotate_clockwise_45(&self) -> Self {
        match self {
            Self::North => Self::NorthEast,
            Self::NorthEast => Self::East,
            Self::East => Self::SouthEast,
            Self::SouthEast => Self::South,
            Self::South => Self::SouthWest,
            Self::SouthWest => Self::West,
            Self::West => Self::NorthWest,
            Self::NorthWest => Self::North,
        }
    }

    pub fn rotate_clockwise_90(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::NorthEast => Self::SouthEast,
            Self::East => Self::South,
            Self::SouthEast => Self::SouthWest,
            Self::South => Self::West,
            Self::SouthWest => Self::NorthWest,
            Self::West => Self::North,
            Self::NorthWest => Self::NorthEast,
        }
    }

    pub fn rotate_counterclockwise_45(&self) -> Self {
        match self {
            Self::North => Self::NorthWest,
            Self::NorthEast => Self::North,
            Self::East => Self::NorthEast,
            Self::SouthEast => Self::East,
            Self::South => Self::SouthEast,
            Self::SouthWest => Self::South,
            Self::West => Self::SouthWest,
            Self::NorthWest => Self::West,
        }
    }

    pub fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::NorthEast => Self::NorthWest,
            Self::East => Self::North,
            Self::SouthEast => Self::NorthEast,
            Self::South => Self::East,
            Self::SouthWest => Self::SouthEast,
            Self::West => Self::South,
            Self::NorthWest => Self::SouthWest,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
        }
    }
}
