use crate::maze::{MAP_WIDTH, MAP_HEIGHT, MAZE_EASY};

pub struct TileInstance {
    pub x: usize,
    pub y: usize,
    pub kind: TileKind,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileKind {
    Ground,
    Wall,
}

pub fn generate_tile_instances() -> Vec<TileInstance> {
    let mut tiles = Vec::new();
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let kind = match MAZE_EASY[y][x] {
                1 => TileKind::Wall,
                _ => TileKind::Ground,
            };
            tiles.push(TileInstance { x, y, kind });
        }
    }
    tiles
}