use std::{
    arch::x86_64::_SIDD_LEAST_SIGNIFICANT,
    collections::HashSet,
    io::{self, stdin, BufWriter, Read, Write},
    os::windows::thread,
    process::Command,
    sync::Mutex,
    time::{Duration, Instant},
    vec,
};

mod guess_game;

struct DoubleBuffer {
    front: Vec<char>,
    back: Vec<char>,
    current: usize,
}

impl DoubleBuffer {
    fn new(width: usize, height: usize) -> Self {
        DoubleBuffer {
            front: vec![' '; width * height],
            back: vec![' '; width * height],
            current: 0,
        }
    }

    fn swap(&mut self) {
        self.current ^= 1;
    }

    fn get_current_buffer(&self) -> &Vec<char> {
        if self.current == 0 {
            &self.front
        } else {
            &self.back
        }
    }

    fn get_current_buffer_mut(&mut self) -> &mut Vec<char> {
        if self.current == 0 {
            &mut self.front
        } else {
            &mut self.back
        }
    }

    fn clear(&mut self) {
        let buffer = self.get_current_buffer_mut();
        buffer.iter_mut().for_each(|c| *c = ' ');
    }
}

/// A simple console renderer.
struct Canvas {
    width: usize,
    height: usize,
    buffer: DoubleBuffer,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        let border = 2;
        Canvas {
            width,
            height,
            buffer: DoubleBuffer::new(width + border, height + border),
        }
    }

    fn draw(&mut self, x: i32, y: i32, c: char) {
        // border padding
        let x = x + 1;
        let y = y + 1;

        let at = y * self.width as i32 + x;
        if at < 0 || at >= (self.width * self.height) as i32 {
            return;
        }
        let current_buffer = self.buffer.get_current_buffer_mut();
        current_buffer[at as usize] = c;
    }

    fn render(&mut self) {
        let mut stdout = BufWriter::new(io::stdout());
        // clear the screen
        write!(stdout, "\x1B[2J\x1B[1;1H").unwrap();
        // disable cursor
        write!(stdout, "\x1B[?25l").unwrap();

        let curr_buffer = self.buffer.get_current_buffer();
        for y in 0..=self.height {
            for x in 0..=self.width {
                if x == 0 || y == 0 || x == self.width || y == self.height {
                    // border
                    write!(stdout, "#").unwrap();
                } else {
                    let at = y * self.width + x;
                    write!(stdout, "{}", curr_buffer[at]).unwrap();
                }
            }
            write!(stdout, "\n").unwrap();
        }
        // self.buffer.swap();
        self.buffer.clear(); // clear the back buffer

        stdout.flush().unwrap();
    }
}

struct Engine {
    canvas: Canvas,
    input: InputManager,
    context: Context,
}

struct InputManager {
    pressed_key: HashSet<char>,
    input_receiver: std::sync::mpsc::Receiver<char>,
}

impl InputManager {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<char>();
        std::thread::spawn(move || loop {
            let mut buf = [0; 1];
            if stdin().read_exact(&mut buf).is_ok() {
                for key in buf {
                    tx.send(key as char).unwrap();
                }
            }
        });
        InputManager {
            pressed_key: HashSet::new(),
            input_receiver: rx,
        }
    }

    fn is_pressed(&self, key: char) -> bool {
        self.pressed_key.contains(&key)
    }

    fn update(&mut self) {
        while let Ok(key) = self.input_receiver.try_recv() {
            self.pressed_key.insert(key);
        }
    }

    fn clear(&mut self) {
        self.pressed_key.clear();
    }
}

#[derive(Default)]
struct Context {
    player_x: i32,
    player_y: i32,
}

impl Engine {
    fn new(canvas: Canvas, context: Context) -> Self {
        Engine {
            canvas,
            context,
            input: InputManager::new(),
        }
    }

    fn run(&mut self) {
        self.update_input();
        self.update_logic();
        self.update_render();
        self.input.clear();
    }

    fn update_input(&mut self) {
        self.input.update();
    }

    fn update_render(&mut self) {
        self.canvas
            .draw(self.context.player_x, self.context.player_y, '@');
        self.canvas.render();
    }

    fn update_logic(&mut self) {
        if self.input.is_pressed('w') {
            self.context.player_y -= 1;
        }
        if self.input.is_pressed('s') {
            self.context.player_y += 1;
        }
        if self.input.is_pressed('a') {
            self.context.player_x -= 1;
        }
        if self.input.is_pressed('d') {
            self.context.player_x += 1;
        }
    }
}

fn console_render() {
    let canvas = Canvas::new(20, 20);
    let context = Context::default();
    let mut engine = Engine::new(canvas, context);

    let fps = 30.0;
    let frame_time = Duration::from_secs_f32(1.0 / fps);

    loop {
        let start_time = Instant::now();
        engine.run();
        std::thread::sleep(start_time + frame_time - Instant::now());
    }
}
