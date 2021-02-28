use crate::cpu::CPU;

use sdl2::{event::Event, keyboard::Keycode};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};
use sdl2::{render::Canvas, video::Window};

const WINDOW_TITLE: &'static str = "Protoshark's CHIP-8";

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SCALE: usize = 5;

const WINDOW_WIDTH: usize = 720;
const WINDOW_HEIGHT: usize = 400;

const BG_COLOR: Color = Color::RGB(0x0F, 0x17, 0x13);
const FG_COLOR: Color = Color::RGB(0x00, 0xFA, 0x00);

pub struct SDL<'c> {
    context: sdl2::Sdl,
    canvas: Canvas<Window>,
    cpu: &'c mut CPU,
}

impl<'c> SDL<'c> {
    pub fn new(cpu: &'c mut CPU) -> Self {
        let context = sdl2::init().expect("Could not initialize the sdl2 context");

        let video = context.video().expect("Could not load the video backend");
        let window = video
            .window(WINDOW_TITLE, (WINDOW_WIDTH) as u32, (WINDOW_HEIGHT) as u32)
            .position_centered()
            .vulkan()
            .borderless()
            .build()
            .expect("Could not create a new window");

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .expect("Could not get any canvas from the window");

        Self {
            context,
            canvas,
            cpu,
        }
    }

    pub fn run(&mut self) {
        // setup the canvas
        self.canvas
            .set_scale(SCREEN_SCALE as f32, SCREEN_SCALE as f32)
            .unwrap();
        self.canvas.set_viewport(Rect::new(
            // x padding
            5,
            // center y axis
            (WINDOW_HEIGHT / (2 * SCREEN_SCALE) - SCREEN_HEIGHT / 2) as i32,
            (SCREEN_WIDTH * SCREEN_SCALE) as u32,
            (SCREEN_HEIGHT * SCREEN_SCALE) as u32,
        ));

        self.canvas.clear();
        self.canvas.present();

        let mut event_pump = self
            .context
            .event_pump()
            .expect("Could not get the event pump");

        'running: loop {
            // run cpu cycle
            for _ in 0..500 {
                self.cpu.cycle();
            }

            self.update_screen();

            // grab input events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }
    }

    fn update_screen(&mut self) {
        self.canvas.clear();

        self.canvas.set_draw_color(FG_COLOR);
        for i in 0..(64 * 32) {
            let x = i % 64;
            let y = i / 64;

            if self.cpu.bus.borrow().vram[i] == 1 {
                self.canvas
                    .draw_point(Point::new(x as i32, y as i32))
                    .unwrap()
            }
        }
        self.canvas.set_draw_color(BG_COLOR);
        self.canvas.present();
    }
}
