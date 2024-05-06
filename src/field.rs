pub mod field_module {
    use std::fmt::{Display, Formatter};

    pub enum TileType {
        Bomb,
        Empty,
    }

    impl Display for TileType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                TileType::Bomb => write!(f, "*"),
                TileType::Empty => write!(f, "@")
            }
        }
    }

    pub struct Tile {
        pub uncovered: bool,
        pub tile_type: TileType,
        pub key: String,
    }

    impl Tile {
        pub fn new(tile_type: TileType, key: String) -> Self {
            Tile { uncovered: false, tile_type, key }
        }

        pub fn set_uncovered(mut self) -> Self {
            self.uncovered = true;
            self
        }

        pub fn to_string(&self) -> String {
            format!("{}", self.tile_type)
        }
    }
}
