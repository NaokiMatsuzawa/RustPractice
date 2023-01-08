use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq)]
pub enum GridID{
    BOMB,
    NUMBER(u32),

    UNOPEN,
}

#[derive(Clone)]
struct FieldGrid {
    grid_id: GridID, //todo.後でラベル付をする
    is_open: bool,
}

impl FieldGrid {
    fn new() -> FieldGrid {
        FieldGrid {
            grid_id: GridID::UNOPEN,
            is_open: false,
        }
    }

    pub fn set_grid_id(&mut self, id: GridID) {
        self.grid_id = id;
    }
}

pub struct FieldBoard {
    grids: Vec<FieldGrid>,
    width: usize,
    height: usize,
}

impl FieldBoard {
    pub fn new(width: usize, height: usize, bomb_num: u32) -> FieldBoard {
        let grid_num = width * height;
        let grids = vec![FieldGrid::new(); grid_num];

        let mut field = FieldBoard {
            grids,
            width,
            height,
        };
        field.make_field(bomb_num);
        field
    }

    fn make_field(&mut self, bomb_num : u32){
        let grid_num = self.width * self.height;


        let cw: usize = self.width + 2;
        let ch: usize = self.height + 2;
        let mut grids_only_id = vec![vec![GridID::NUMBER(0); self.width]; self.height];
        let mut grids_complement = vec![vec![GridID::NUMBER(0); cw]; ch];

        let mut nums: Vec<usize> = (0..grid_num).collect();
        let mut rand = rand::thread_rng();
        nums.shuffle(&mut rand);

        for i in 0..bomb_num as usize {
            let grid_index = nums.get(i).unwrap();
            let x = grid_index % self.width;
            let y = grid_index / self.width;
            grids_only_id[y][x] = GridID::BOMB;
            grids_complement[y + 1][x + 1] = GridID::BOMB;
        }

        let field_vec = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for y in 0..self.height {
            for x in 0..self.width {
                if grids_only_id[y][x] == GridID::BOMB {
                    continue;
                }
                let y_sub: i32 = y as i32 + 1;
                let x_sub: i32 = x as i32 + 1;
                let mut surround_bomb_num = 0;
                for vector in &field_vec {
                    if grids_complement[(y_sub + vector.0) as usize][(x_sub + vector.1) as usize]
                        == GridID::BOMB
                    {
                        surround_bomb_num += 1;
                    }
                }
                grids_only_id[y][x] = GridID::NUMBER(surround_bomb_num);
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                self.grids
                    .get_mut(y * self.width + x)
                    .unwrap()
                    .set_grid_id(grids_only_id[y][x]);
            }
        }
    }

    pub fn is_gameover(&self) -> bool {
        for grid in &self.grids {
            if grid.grid_id == GridID::BOMB && grid.is_open == true {
                return true;
            }
        }
        false
    }

    pub fn is_clear(&self) -> bool {
        for grid in &self.grids {
            if grid.grid_id != GridID::BOMB && !grid.is_open {
                return false;
            }
        }
        true
    }

    pub fn request_open(&mut self, x: usize, y: usize) {
        if x >= self.width {
            return;
        }
        if y >= self.height {
            return;
        }
        let mut grid = self.get_grid_mut(x, y);
        if grid.is_open {
            return;
        }

        grid.is_open = true;

        if grid.grid_id == GridID::NUMBER(0) {
            if x > 0 {
                self.request_open(x - 1, y);
            }
            if x < self.width - 1 {
                self.request_open(x + 1, y);
            }
            if y > 0 {
                self.request_open(x, y - 1);
            }
            if y < self.height - 1 {
                self.request_open(x, y + 1);
            }
            if x > 0 && y > 0{
                self.request_open(x - 1, y - 1);
            }
            if x < self.width - 1  && y > 0{
                self.request_open(x + 1, y - 1);
            }
            if x > 0 && y < self.height - 1{
                self.request_open(x - 1, y + 1);
            }
            if x < self.width - 1 && y < self.height - 1{
                self.request_open(x + 1, y + 1);
            }
        }
    }

    fn get_grid_mut(&mut self, x:usize, y:usize) ->&mut FieldGrid{
        let index = x + y * self.width;
        self.grids.get_mut(index).expect("out of range : GRIDS")
    }

    fn get_grid(&self, x: usize, y: usize) -> &FieldGrid {
        let index = x + y * self.width;
        self.grids.get(index).expect("out of range : GRIDS")
    }

    pub fn is_open(&self, x: usize, y: usize) -> bool {
        self.get_grid(x, y).is_open
    }

    pub fn get_grid_id(&self, x: usize, y: usize) -> GridID {
        if self.is_open(x, y) {
            return self.get_grid(x, y).grid_id;
        }
        GridID::UNOPEN
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
