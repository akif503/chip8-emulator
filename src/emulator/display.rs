use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::EventPump;
use std::time::Duration;
use rand::Rng;

// WIDTH x HEIGHT -> 64x32
const PIXELS_ALONG_X: u32 = 64;
const PIXELS_ALONG_Y: u32 = 32;

// Our Display class
pub struct Display {
    pub sdl_context: Sdl, 
    pub canvas: WindowCanvas,
    // width: u32,
    // height: u32,
    pub epixels: [[(Rect, Color); PIXELS_ALONG_X as usize]; PIXELS_ALONG_Y as usize],
    pub pixel_repr: [[u8; PIXELS_ALONG_X as usize]; PIXELS_ALONG_Y as usize], 
}

impl Display {
    pub fn new(width: u32, height: u32) -> Display {
        let sdl_context = sdl2::init()
            .expect("Couldn't initialize SDL2.");
        let video_subsystem = sdl_context.video()
            .expect("Couldn't initialize video subsystem.");
        
        // This function initializes the video subsystem, setting up a connection to the window manager, etc, 
        // and determines the available display modes and pixel formats, 
        // but does not initialize a window or graphics mode.
        let window = video_subsystem.window("Chip8 Emulator", width, height)
            .position_centered()
            .build()
            .expect("Could not initialize video system.");

        let canvas = window.into_canvas().build()
            .expect("Could not make a canvas.");

        // This function depend on constants UNIT_WIDTH & UNIT_HEIGHT
        let unit_height: i32 = (height / PIXELS_ALONG_Y) as i32;
        let unit_width: i32 = (width / PIXELS_ALONG_X) as i32;

        // Position the rectangles
        let mut epixels = [[(Rect::new(0,0,1,1), Color::GRAY); PIXELS_ALONG_X as usize]; PIXELS_ALONG_Y as usize];
        let mut pixel_repr = [[0; PIXELS_ALONG_X as usize]; PIXELS_ALONG_Y as usize];
        let mut rng = rand::thread_rng();

        for y_idx in 0..(PIXELS_ALONG_Y as usize) {
            for x_idx in 0..(PIXELS_ALONG_X as usize) {
                let x: i32 = (x_idx as i32) * unit_width;
                let y: i32 = (y_idx as i32) * unit_height;

                epixels[y_idx][x_idx] = (Rect::new(x, y, unit_width as u32, unit_height as u32), Color::GRAY);
                // pixel_repr[y_idx][x_idx] = rng.gen::<bool>() as u8;
                pixel_repr[y_idx][x_idx] = 0;
            }
        }

        Display {
            sdl_context,
            canvas,
            // width,
            // height,
            epixels,
            pixel_repr
        }
    }

    pub fn xor_sprite(&mut self, memory: &[u8], start_addr: usize, pos: (u8, u8), sprite_height: u8) -> u8 {
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        //
        // The interpreter reads n bytes from memory, starting at the address stored in I. 
        // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
        // Sprites are XORed onto the existing screen. If this causes any pixels to be erased, 
        // VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part 
        // of it is outside the coordinates of the display, it wraps around to the opposite 
        // side of the screen.

        let mut sprite: Vec<u8> = vec![Default::default(); sprite_height as usize];

        for idx in 0..sprite.len() {
            sprite[idx] = memory[start_addr + idx];
        }

        // println!("{:#x?}", sprite);
        // println!("{:#x?}", pos);

        let (start_x, start_y) = pos;

        let width = self.pixel_repr[0].len() as u8;
        let height = self.pixel_repr.len() as u8;

        let mut collision: u8 = 0; 

        for y in 0..sprite_height {
            let pos_y = ((start_y as u16 + y as u16) % (height as u16)) as u8;

            for x in 0..8 {
                let pos_x = ((start_x as u16 + (7 - x) as u16) % (width as u16)) as u8;
                // println!("{} {}", pos_x, pos_y); -----------------------------

                let cur = self.pixel_repr[pos_y as usize][pos_x as usize];
                // The following fetches the binary value at position x of the row.
                let sprite_pixel = (sprite[y as usize] & (1 << x)) >> x;

                // XOR the pixels
                self.pixel_repr[pos_y as usize][pos_x as usize] = cur ^ sprite_pixel;
                // self.pixel_repr[pos_y as usize][pos_x as usize] = sprite_pixel; 

                if collision == 0 {
                    if cur != self.pixel_repr[pos_y as usize][pos_x as usize] {
                        collision = 1;
                    }
                }
            }
        }

        return collision;
    }

    pub fn clear(&mut self) {
        for y in 0..self.pixel_repr.len() {
            for x in 0..self.pixel_repr[0].len() {
                self.pixel_repr[y][x] = 0;
            }
        }
    }

    // pub fn run(&mut self) {
    //     'running: loop {
    //         // Handle Events
    //         for event in event_pump.poll_iter() {
    //             match event {
    //                 Event::Quit {..} |
    //                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                     break 'running
    //                 },
    //                 _ => {}
    //             }
    //         }

    //         // Update 

    //         // Render Canvas
    //         self.render();

    //         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    //     }
    // }

    pub fn render(&mut self) {
        // Renders the rectangles on the screen

        for (row, repr_row) in self.epixels.iter().zip(self.pixel_repr.iter()) {
            for (epixel, pixel_val) in row.iter().zip(repr_row.iter()) {
                let (rect, _) = epixel;
                let color = if *pixel_val == 1 { Color::WHITE } else { Color::BLACK };

                self.canvas.set_draw_color(color);
                self.canvas.draw_rect(*rect).unwrap();
                self.canvas.fill_rect(*rect).unwrap();
            }
        }

        self.canvas.present();
    }
}