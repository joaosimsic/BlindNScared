use crate::common::{MAP_HEIGHT, MAP_WIDTH};
use rand::Rng;

pub const TILE_VOID: u8 = 1 << 0;
pub const TILE_FLOOR: u8 = 1 << 1;
pub const TILE_WALL: u8 = 1 << 2;
pub const TILE_ALTAR: u8 = 1 << 3;
pub const ALL_TILES: u8 = TILE_VOID | TILE_FLOOR | TILE_WALL | TILE_ALTAR;

#[derive(Clone, Copy)]
pub struct Cell {
    pub possible: u8,
    pub collapsed: bool,
    pub final_type: u8,
}

impl Cell {
    pub const fn new() -> Self {
        Cell { possible: ALL_TILES, collapsed: false, final_type: 0 }
    }
}

#[derive(Clone, Copy)]
pub struct Coord {
    pub y: usize,
    pub x: usize,
}

pub struct World {
    pub map: [[Cell; MAP_WIDTH]; MAP_HEIGHT],
    pub uncollapsed: Vec<Coord>,
}

pub fn allowed_neighbors(t: u8) -> u8 {
    match t {
        TILE_ALTAR => TILE_FLOOR,
        TILE_FLOOR => TILE_WALL | TILE_FLOOR | TILE_ALTAR,
        TILE_WALL => TILE_VOID | TILE_FLOOR | TILE_WALL,
        TILE_VOID => TILE_VOID | TILE_WALL,
        _ => 0,
    }
}

fn pick_random_tile(possible: u8) -> u8 {
    let count = possible.count_ones();
    if count == 0 {
        return 0;
    }
    let choice = rand::thread_rng().gen_range(0..count);
    let mut current = 0;
    for i in 0..4 {
        let bit = 1u8 << i;
        if possible & bit != 0 {
            if current == choice {
                return bit;
            }
            current += 1;
        }
    }
    0
}

impl World {
    pub fn new() -> Self {
        let mut uncollapsed = Vec::with_capacity(MAP_HEIGHT * MAP_WIDTH);
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                uncollapsed.push(Coord { y, x });
            }
        }
        World {
            map: [[Cell::new(); MAP_WIDTH]; MAP_HEIGHT],
            uncollapsed,
        }
    }

    pub fn collapse_cell(&mut self, y: usize, x: usize) {
        let chosen = pick_random_tile(self.map[y][x].possible);
        self.map[y][x].collapsed = true;
        self.map[y][x].final_type = chosen;
        self.map[y][x].possible = chosen;

        if let Some(pos) = self
            .uncollapsed
            .iter()
            .position(|c| c.y == y && c.x == x)
        {
            self.uncollapsed.swap_remove(pos);
        }
    }

    pub fn propagate(&mut self, y: usize, x: usize) {
        let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dy, dx) in dirs {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny < 0 || ny >= MAP_HEIGHT as isize || nx < 0 || nx >= MAP_WIDTH as isize {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;

            if self.map[ny][nx].collapsed {
                continue;
            }

            let allowed = allowed_neighbors(self.map[y][x].final_type);
            let before = self.map[ny][nx].possible;
            let after = before & allowed;

            if after != before {
                self.map[ny][nx].possible = after;
                if after.count_ones() == 1 {
                    self.collapse_cell(ny, nx);
                    self.propagate(ny, nx);
                }
            }
        }
    }

    pub fn step(&mut self) -> bool {
        if self.uncollapsed.is_empty() {
            return false;
        }

        let mut min_entropy = u32::MAX;
        let mut best_idx: Option<usize> = None;

        for (i, c) in self.uncollapsed.iter().enumerate() {
            let count = self.map[c.y][c.x].possible.count_ones();
            if count > 0 && count < min_entropy {
                min_entropy = count;
                best_idx = Some(i);
            }
        }

        let idx = match best_idx {
            Some(i) => i,
            None => return false,
        };
        let Coord { y, x } = self.uncollapsed[idx];

        self.collapse_cell(y, x);
        self.propagate(y, x);

        true
    }

    pub fn generate(&mut self) {
        while self.step() {}
    }
}
