use bevy::math::IVec2;

#[derive(Debug, Clone)]
pub struct Map {
    walls: Vec<IVec2>,
    trash: Vec<IVec2>,

    start: IVec2,
    exit: IVec2,

    width: usize,
    height: usize,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            walls: Vec::new(),
            trash: Vec::new(),

            start: (0, 0).into(),

            // FIXME: should be optional
            exit: (4, 4).into(),

            width: 5,
            height: 5,
        }
    }
}

impl Map {
    pub fn parse(str: &str) -> Result<Self, String> {
        let mut walls = Vec::new();
        let mut trash = Vec::new();
        let mut start = (0, 0).into();
        let mut exit = (1, 1).into();

        let str = str.trim();

        let width = str.lines().next().ok_or("String is empty.")?.len();
        let height = str.lines().count();

        for (row_idx, row) in str.lines().enumerate() {
            for (col_idx, char) in row.chars().enumerate() {
                let pt = (col_idx as i32, height as i32 - row_idx as i32 - 1).into();
                match char {
                    '#' => walls.push(pt),
                    'T' => trash.push(pt),
                    'S' => start = pt,
                    'E' => exit = pt,
                    '.' => {}
                    c => {
                        return Err(format!("Unrecognized character in map string: '{c}'"));
                    }
                }
            }
        }

        Ok(Self {
            walls,
            trash,

            start,
            exit,

            width,
            height,
        })
    }

    /// returns (width, height)
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn start(&self) -> IVec2 {
        self.start
    }

    pub fn exit(&self) -> IVec2 {
        self.exit
    }

    pub fn walls(&self) -> &[IVec2] {
        &self.walls
    }

    pub fn trash(&self) -> &[IVec2] {
        &self.trash
    }

    pub fn has_space(&self, pt: impl Into<IVec2>) -> bool {
        let pt: IVec2 = pt.into();

        pt.x >= 0
            && pt.y >= 0
            && pt.x < self.width as i32
            && pt.y < self.height as i32
            && !self.walls.contains(&pt)
    }
}

#[cfg(test)]
impl Map {
    pub const EMPTY_3X3: &str = r"
...
.ST
...";

    pub const BIG_LOOP_5X6: &str = r"
#####
#..E#
#T#.#
#.#T#
#.S.#
#####
";

    pub const ROOM_4X4: &str = r"
####
#.E#
#S.#
####";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_map_from_str() {
        let map = Map::parse(Map::BIG_LOOP_5X6).unwrap();

        assert_eq!(map.dimensions(), (5, 6));
        assert_eq!(map.start(), (2, 1).into());
        assert_eq!(map.exit(), (3, 4).into());

        assert!(map.trash().contains(&(1, 3).into()));
        assert!(map.trash().contains(&(3, 2).into()));
    }

    #[test]
    fn test_has_space() {
        let map = Map::parse(Map::BIG_LOOP_5X6).unwrap();

        assert!(map.has_space((1, 2)));
        assert!(!map.has_space((2, 3)));

        // out of bounds
        assert!(!map.has_space((-1, 0)));
        assert!(!map.has_space((1, -1)));
        assert!(!map.has_space((10, 1)));
        assert!(!map.has_space((1, 10)));
    }
}
