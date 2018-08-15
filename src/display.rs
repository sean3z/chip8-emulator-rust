use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub static FONTS: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Display {
  canvas: Canvas<Window>,
}

impl Display {
  pub fn new() -> Display {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            WIDTH,
            HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    Display {
      canvas: canvas
    }
  }

  pub fn clear(&mut self) {
    println!("{}", "clear");
  }

  pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
    for j in 0..sprite.len() {
      let row = sprite[j];
      for i in 0..8 {
        let value = row >> (7 - i) & 0x01;
        if value == 1 {
          let xi = (x + i) % WIDTH as usize;
          let yj = (y + j) % WIDTH as usize;
          let _ = self.canvas.fill_rect(Rect::new(xi as i32, yj as i32, 20, 20));
        }
      }
    };

    self.canvas.present();

    true
  }
}