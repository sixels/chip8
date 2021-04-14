use std::{ops::Deref, usize};

use crate::cpu::{Status as CpuStatus, CPU};

use sdl2::{event::Event, keyboard::Keycode};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};
use sdl2::{render::Canvas, video::Window};

const WINDOW_TITLE: &'static str = "Protoshark's CHIP-8";

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SCALE: usize = 8;

const WINDOW_WIDTH: usize = SCREEN_WIDTH * SCREEN_SCALE;
const WINDOW_HEIGHT: usize = SCREEN_HEIGHT * SCREEN_SCALE;

const BG_COLOR: Color = Color::RGB(0x0F, 0x17, 0x13);
const FG_COLOR: Color = Color::RGB(0x00, 0xFA, 0x00);

struct KeypadKey(usize);

impl Deref for KeypadKey {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl KeypadKey {
    fn from_keycode(keycode: Keycode) -> Option<Self> {
        match keycode {
            Keycode::Num1 => Some(Self(0x1)),
            Keycode::Num2 => Some(Self(0x2)),
            Keycode::Num3 => Some(Self(0x3)),
            Keycode::Num4 => Some(Self(0xC)),
            Keycode::Q => Some(Self(0x4)),
            Keycode::W => Some(Self(0x5)),
            Keycode::E => Some(Self(0x6)),
            Keycode::R => Some(Self(0xD)),
            Keycode::A => Some(Self(0x7)),
            Keycode::S => Some(Self(0x8)),
            Keycode::D => Some(Self(0x9)),
            Keycode::F => Some(Self(0xE)),
            Keycode::Z => Some(Self(0xA)),
            Keycode::X => Some(Self(0x0)),
            Keycode::C => Some(Self(0xB)),
            Keycode::V => Some(Self(0xF)),
            _ => None,
        }
    }
}

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
            // center x axis
            1 / 4 * (WINDOW_WIDTH / SCREEN_SCALE - SCREEN_WIDTH) as i32,
            // center y axis
            1 / 4 * (WINDOW_HEIGHT / SCREEN_SCALE - SCREEN_HEIGHT) as i32,
            (SCREEN_WIDTH * SCREEN_SCALE) as u32,
            (SCREEN_HEIGHT * SCREEN_SCALE) as u32,
        ));

        self.canvas.clear();
        self.canvas.present();

        let mut event_pump = self
            .context
            .event_pump()
            .expect("Could not get the event pump");

        use std::time;

        let mut timer = time::Instant::now();

        'running: loop {
            if timer.elapsed() >= time::Duration::from_millis(1000 / 60) {
                // run cpu cycle
                for _ in 0..20 {
                    self.cpu.cycle();
                }

                self.cpu.keypad = 0;

                self.cpu.delay = self.cpu.delay.saturating_sub(1);
                self.cpu.sound = self.cpu.sound.saturating_sub(1);


                // grab input events
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        Event::KeyDown {
                            keycode: Some(keycode),
                            ..
                        } => {
                            let key = KeypadKey::from_keycode(keycode);
                            if let Some(key) = key {
                                let index: usize = *key.deref();
                                if let CpuStatus::WaitingKeypress(x) = self.cpu.status {
                                    self.cpu.v[x] = index as u8;
                                    self.cpu.status = CpuStatus::Running;
                                }
                                self.cpu.keypad = 1 << index
                            }
                        }
                        _ => {}
                    }
                }
                self.update_screen();

                timer = time::Instant::now()
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
