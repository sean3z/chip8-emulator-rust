use sdl::video;
use sdl::Rect;

pub struct Display {
    gfx: [[u8; 64]; 32],
    draw_flag: bool,
    screen: video::Surface
}

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: isize = 20;

impl Display {
    pub fn new() -> Display {
        Display {
            gfx: [[0; WIDTH]; HEIGHT],
            draw_flag: true,
            screen: video::set_video_mode(WIDTH as isize * SCALE, HEIGHT as isize * SCALE, 8,
                                          &[video::SurfaceFlag::HWSurface],
                                          &[video::VideoFlag::DoubleBuf]).unwrap()
        }
    }

    pub fn clear(&mut self) {
        self.gfx = [[0; WIDTH]; HEIGHT];
        self.draw_flag = true;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0u8;
        let n = sprite.len() as usize;
        let mut yj: usize;
        let mut xi: usize;

        for j in 0..n {
            for i in 0..8 {
                yj = (y + j) % HEIGHT;
                xi = (x + i) % WIDTH;

                if (sprite[j] & (0x80 >> i)) != 0 {
                    if self.gfx[yj][xi] == 1 { collision = 1 }
                    self.gfx[yj][xi] ^= 1;
                }
            }
        }

        self.draw_flag = true;
        collision
    }

    pub fn draw_screen(&mut self) {
        if !self.draw_flag { return }
        let mut pixel: u8;
        let sc = SCALE as u16;
        let pt = |p: usize| { (p as i16) * (SCALE as i16) };

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                pixel = if self.gfx[y][x] != 0 { 255 } else { 0 };
                self.screen.fill_rect(Some(Rect { x: pt(x), y: pt(y), w: sc, h: sc}),
                video::RGB(pixel, pixel, pixel));
            }
        }

        self.screen.flip();
        self.draw_flag = false;
    }
}