use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::time::Duration;

// WIDTH x HEIGHT -> 64x32
const PIXELS_ALONG_X: u32 = 64;
const PIXELS_ALONG_Y: u32 = 32;

// For simplicity we'll assume width and height are multiples of our final mapping
const WIDTH: u32 = 768;
const HEIGHT: u32 = 576;

// Unit rectangle
const UNIT_WIDTH: i32 = (WIDTH / PIXELS_ALONG_X) as i32;
const UNIT_HEIGHT: i32 = (HEIGHT / PIXELS_ALONG_Y) as i32;

fn render(canvas: &mut WindowCanvas, rects: [[(Rect, Color); PIXELS_ALONG_X as usize]; PIXELS_ALONG_Y as usize]) {

    // let color = Color::RGB(51, 51, 51);
    // canvas.set_draw_color(color);

    // let mut unravelled: [Rect; (PIXELS_ALONG_Y * PIXELS_ALONG_X) as usize] = [Rect::new(0, 0, 1, 1); (PIXELS_ALONG_Y * PIXELS_ALONG_X) as usize];

    // let mut idx: u32;
    for y_idx in 0..PIXELS_ALONG_Y {
        for x_idx in 0..PIXELS_ALONG_X {
            // idx = y_idx*PIXELS_ALONG_X + x_idx;
            //unravelled[idx as usize] = rects[y_idx as usize][x_idx as usize];
            let (rect, color) = rects[y_idx as usize][x_idx as usize];
            canvas.set_draw_color(color);
            canvas.draw_rect(rect).unwrap();
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}

fn main() {
    println!("uW x uH: ({}, {})", UNIT_WIDTH, UNIT_HEIGHT);

    // Initial Checks
    // Check if width & height are accurate

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Could not initialize video system.");

    let mut canvas = window.into_canvas().build()
        .expect("Could not make a canvas.");

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Handle Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Update 
        //let unit = Rect::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32, 100, 100);
        let mut rects: [[(Rect, Color); PIXELS_ALONG_X as usize]; PIXELS_ALONG_Y as usize] = [[(Rect::new(0,0,1,1), Color::GRAY); PIXELS_ALONG_X as usize]; PIXELS_ALONG_Y as usize];
        for y_idx in 0..PIXELS_ALONG_Y {
            for x_idx in 0..PIXELS_ALONG_X {
                let x: i32 = (x_idx as i32) * UNIT_WIDTH;
                let y: i32 = (y_idx as i32) * UNIT_HEIGHT;

                rects[y_idx as usize][x_idx as usize] = (Rect::new(x, y, UNIT_WIDTH as u32, UNIT_HEIGHT as u32), Color::GRAY);
            }
        }

        // Render Canvas
        render(&mut canvas, rects);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}