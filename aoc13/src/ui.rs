#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::time::{Duration, Instant};

use anyhow::Result;
use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use intcode::Computer;

const WIDTH: u32 = 44;
const HEIGHT: u32 = 25;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    tiles: HashMap<(i16, i16), TileType>,
    score: i64,
    ball: (i16, i16),
    paddle: (i16, i16),
}

#[derive(Debug, Eq, PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
    Score(i64),
}

#[derive(Debug)]
struct Tile {
    x: i64,
    y: i64,
    tile_type: TileType,
}

async fn read_tile(stream: &async_std::sync::Receiver<i64>) -> Option<Tile> {
    let x = stream.recv().await?;
    let y = stream.recv().await?;
    let tile_id = stream.recv().await?;
    let tile_type = match tile_id {
        0 => TileType::Empty,
        1 => TileType::Wall,
        2 => TileType::Block,
        3 => TileType::HorizontalPaddle,
        4 => TileType::Ball,
        score if x == -1 && y == 0 => TileType::Score(score),
        _ => panic!("Invalid type type {}", tile_id),
    };
    Some(Tile { x, y, tile_type })
}

// #[derive(Debug, Clone)]
// struct GameIO {
//     output: Sender<i64>,
//     input: Receiver<i64>,
// }

// #[async_trait]
// impl intcode::IO for GameIO {
//     async fn input(&self) -> Result<i64> {
//         async_std::task::sleep(Duration::from_secs(1)).await;
//         let value = self.input.load(Ordering::Relaxed).into();
//         println!("Input {}", value);
//         Ok(value)
//     }

//     async fn output(&mut self, data: i64) -> Result<()> {
//         self.output.send(data).await;
//         Ok(())
//     }
// }

fn spawn_computer(sender: EventLoopProxy<Tile>) -> Result<impl Fn(i64) -> ()> {
    // let (tx, output) = channel(100);
    // let input = Arc::new(AtomicI8::new(0));
    // let io = GameIO { output: tx, input };

    let mut program = intcode::parse_program(include_str!("../input/input.txt"))?;
    program[0] = 2;
    println!("{}", program[0]);
    let mut computer = Computer::from_mem(program);
    let output = computer.create_output_channel();
    let input = computer.create_input_channel();

    computer.spawn();
    async_std::task::spawn(async move {
        while let Some(tile) = read_tile(&output).await {
            if let Err(err) = sender.send_event(tile) {
                eprintln!("Error sending tile: {}", err);
            }
        }
        println!("HALTED");
    });

    // async_std::task::spawn(async move {
    //     loop {
    //         input.send(0).await;
    //         async_std::task::sleep(Duration::from_secs(1)).await;
    //     }
    // });

    Ok(move |value| {
        let input = input.clone();
        async_std::task::spawn(async move { input.send(value).await });
    })
}

pub fn run() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event();

    let send_input = spawn_computer(event_loop.create_proxy())?;

    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Pong")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut hidpi_factor = window.hidpi_factor();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, surface);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();
    let mut time = Instant::now();
    let mut dt = Duration::new(0, 0);

    let ai = crate::ai::Ai::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                world.draw(pixels.get_frame());
                pixels.render();
            }
            Event::UserEvent(tile) => {
                world.new_tile(tile);
                window.request_redraw();
            }
            event => {
                // Handle input events
                if input.update(event) {
                    // Close events
                    if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    // let joystick = if input.key_held(VirtualKeyCode::Left) {
                    //     -1
                    // } else if input.key_held(VirtualKeyCode::Right) {
                    //     1
                    // } else {
                    //     0
                    // };

                    // Adjust high DPI factor
                    if let Some(factor) = input.hidpi_changed() {
                        hidpi_factor = factor;
                    }

                    // Resize the window
                    if let Some(size) = input.window_resized() {
                        let size = size.to_physical(hidpi_factor);
                        let width = size.width.round() as u32;
                        let height = size.height.round() as u32;

                        pixels.resize(width, height);
                    }

                    let now = Instant::now();
                    dt += now.duration_since(time);
                    time = now;

                    let step = Duration::from_millis(50);
                    while dt > step {
                        send_input(ai.get_next(world.paddle, world.ball) as i64);
                        dt -= step;
                    }

                    // Update internal state and request a redraw
                    window.request_redraw();
                }
            }
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            score: 0,
            ball: (0, 0),
            paddle: (0, 0),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn new_tile(&mut self, tile: Tile) {
        if let TileType::Ball = tile.tile_type {
            self.ball = (tile.x as i16, tile.y as i16);
        }
        if let TileType::HorizontalPaddle = tile.tile_type {
            self.paddle = (tile.x as i16, tile.y as i16);
        }

        match tile.tile_type {
            TileType::Score(score) => {
                self.score = score;
                println!("Score: {}", score);
            }
            _ => {
                self.tiles
                    .insert((tile.x as i16, tile.y as i16), tile.tile_type);
            }
        };

        // println!(
        //     "{}",
        //     self.tiles
        //         .values()
        //         .filter(|typ| typ == &&TileType::Block)
        //         .count()
        // );
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            use TileType::*;
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let rgba = match self.tiles.get(&(x, y)) {
                None | Some(Empty) => [0xff, 0xff, 0xff, 0xff],
                Some(Wall) => [0x00, 0x00, 0x00, 0xff],
                Some(Block) => [0xcc, 0xcc, 0xcc, 0xff],
                Some(HorizontalPaddle) => [0x00, 0x00, 0xff, 0xff],
                Some(Ball) => [0xff, 0x00, 0x00, 0xff],
                Some(Score(_)) => panic!("Got a score-tile"),
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
