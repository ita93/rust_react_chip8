//Screen 64*32;
const WIDTH_SIZE: usize = 64;
const HEIGH_SIZE: usize = 32;

pub struct Display{
    memory: [bool; HEIGH_SIZE * WIDTH_SIZE],
}

impl Display {
    pub fn new() -> Self{
        Self{
            memory: [false; HEIGH_SIZE * WIDTH_SIZE],
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        if x < WIDTH_SIZE && y < HEIGH_SIZE{
            return Some(self.memory[x + y * WIDTH_SIZE]);
        }
        None
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        if x < WIDTH_SIZE && y < HEIGH_SIZE {
            self.memory[x + y * WIDTH_SIZE] = value;
        }
    }

    pub fn cls(&mut self){
        for j in 0..HEIGH_SIZE{
            for i in 0..WIDTH_SIZE{
                self.set_pixel(i, j, false);
            }
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool{
        let sprite_rows = sprite.len();
        let mut collision = false;
        for j in 0..sprite_rows{
            let row = sprite[j];
            for i in 0..8 {
                let pixel_val = if row >> (7 - i) & 0x01 == 1 {true} else {false};
                let xi = (x + i) % WIDTH_SIZE;
                let yj = (y + j) % HEIGH_SIZE;
                match self.get_pixel(xi, yj){
                    Some(val) =>
                    {
                        collision = val;
                        self.set_pixel(xi, yj, pixel_val ^ val);
                    }
                    _ => {},
                }
            }
        }
        collision
    }
}