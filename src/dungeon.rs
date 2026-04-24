use crate::common::{Rect, TILE_DOOR, TILE_EXIT, TILE_FLOOR, TILE_WALL};

pub struct World {
    pub map: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    houses: Vec<Rect>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        World {
            map: vec![vec![TILE_FLOOR; width]; height],
            width,
            height,
            houses: Vec::new(),
        }
    }

    pub fn generate(&mut self) {
        self.fill_rect(0, 0, self.width, self.height, TILE_FLOOR);

        let num_houses = 1 + rand::random::<usize>() % 3;
        for _ in 0..num_houses {
            self.try_place_house();
        }

        self.place_exits();
    }

    fn try_place_house(&mut self) {
        let min_size: usize = 10;
        let max_w = (self.width / 2).max(min_size + 1);
        let max_h = (self.height / 2).max(min_size + 1);

        let hw = min_size + rand::random::<usize>() % (max_w - min_size + 1);
        let hh = min_size + rand::random::<usize>() % (max_h - min_size + 1);

        if hw + 2 >= self.width || hh + 2 >= self.height {
            return;
        }

        let hx = 1 + rand::random::<usize>() % (self.width - hw - 1);
        let hy = 1 + rand::random::<usize>() % (self.height - hh - 1);

        let candidate = Rect {
            x: hx,
            y: hy,
            w: hw,
            h: hh,
        };
        let overlaps = self.houses.iter().any(|h| {
            candidate.x < h.x + h.w
                && candidate.x + candidate.w > h.x
                && candidate.y < h.y + h.h
                && candidate.y + candidate.h > h.y
        });

        if !overlaps {
            self.houses.push(candidate);
            self.build_house(hx, hy, hw, hh);
        }
    }

    fn build_house(&mut self, hx: usize, hy: usize, hw: usize, hh: usize) {
        self.draw_rect_border(hx, hy, hw, hh);
        self.fill_rect(hx + 1, hy + 1, hw - 2, hh - 2, TILE_FLOOR);

        self.bsp_split(hx + 1, hy + 1, hw - 2, hh - 2);

        self.place_entry_door(hx, hy, hw, hh);
    }

    fn bsp_split(&mut self, x: usize, y: usize, w: usize, h: usize) {
        let min_room_dim = 4;
        let can_v = w >= min_room_dim * 2 + 1;
        let can_h = h >= min_room_dim * 2 + 1;

        if !can_v && !can_h {
            return;
        }

        let vertical = if can_v && can_h {
            rand::random::<bool>()
        } else {
            can_v
        };

        if vertical {
            let split_x = x + min_room_dim + rand::random::<usize>() % (w - min_room_dim * 2);

            for row in y..y + h {
                self.map[row][split_x] = TILE_WALL;
            }
            let door_y = y + rand::random::<usize>() % h;
            self.map[door_y][split_x] = TILE_DOOR;

            self.bsp_split(x, y, split_x - x, h);
            self.bsp_split(split_x + 1, y, x + w - split_x - 1, h);
        } else {
            let split_y = y + min_room_dim + rand::random::<usize>() % (h - min_room_dim * 2);

            for col in x..x + w {
                self.map[split_y][col] = TILE_WALL;
            }
            let door_x = x + rand::random::<usize>() % w;
            self.map[split_y][door_x] = TILE_DOOR;

            self.bsp_split(x, y, w, split_y - y);
            self.bsp_split(x, split_y + 1, w, y + h - split_y - 1);
        }
    }

    fn place_entry_door(&mut self, hx: usize, hy: usize, hw: usize, hh: usize) {
        let side = rand::random::<usize>() % 4;
        let (dx, dy) = match side {
            0 => (hx + 1 + rand::random::<usize>() % (hw - 2), hy),
            1 => (hx + 1 + rand::random::<usize>() % (hw - 2), hy + hh - 1),
            2 => (hx, hy + 1 + rand::random::<usize>() % (hh - 2)),
            _ => (hx + hw - 1, hy + 1 + rand::random::<usize>() % (hh - 2)),
        };
        self.map[dy][dx] = TILE_DOOR;
    }

    fn draw_rect_border(&mut self, x: usize, y: usize, w: usize, h: usize) {
        for i in x..x + w {
            self.map[y][i] = TILE_WALL;
            self.map[y + h - 1][i] = TILE_WALL;
        }
        for i in y..y + h {
            self.map[i][x] = TILE_WALL;
            self.map[i][x + w - 1] = TILE_WALL;
        }
    }

    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, tile: char) {
        for row in y..y + h {
            for col in x..x + w {
                if row < self.height && col < self.width {
                    self.map[row][col] = tile;
                }
            }
        }
    }

    fn place_exits(&mut self) {
        let floor_tiles: Vec<(usize, usize)> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| self.map[y][x] == TILE_FLOOR)
            .collect();

        if !floor_tiles.is_empty() {
            let (x, y) = floor_tiles[rand::random::<usize>() % floor_tiles.len()];
            self.map[y][x] = TILE_EXIT;
        }
    }
}
