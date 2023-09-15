#![warn(clippy::pedantic, clippy::nursery, rust_2018_idioms)]

use std::time::{Duration, Instant};

use instructions::Instruction;
use log::{debug, info};
use minifb::{Window, WindowOptions};

mod chip8;
mod cli;
mod display;
mod instructions;
mod keypad;
mod registers;
mod stack;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    let args = cli::parse_args().expect("failed to parse arguments");

    let prg = std::fs::read(args.program).expect("failed to open program");

    info!("Starting emulator");

    let mut chip8 = chip8::Chip8::load_prg(&prg);

    let mut buf = [0_u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "CHIP-8 Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X16,
            ..Default::default()
        },
    )
    .expect("failed to create window");

    // We do our own limiting
    window.limit_update_rate(None);

    let mut window_timer = Instant::now();
    let mut instruction_timer = Instant::now();

    let mut display_modified = instructions::DisplayModified::Unchanged;

    while window.is_open() {
        // 60 Hz
        // Update both display and timer
        if window_timer.elapsed() >= Duration::from_micros(16600) {
            if chip8.delay_timer > 0 {
                chip8.delay_timer -= 1;
                debug!("Decrementing delay timer: {}", chip8.delay_timer);
            }

            if chip8.sound_timer > 0 {
                // TODO: make it actually play a sound
                window.set_title("🔔🔔 CHIP-8 Emulator 🔔🔔");
                chip8.sound_timer -= 1;
                debug!("Decrementing sound timer: {}", chip8.sound_timer);
            } else {
                window.set_title("CHIP-8 Emulator");
            }

            if matches!(display_modified, instructions::DisplayModified::Changed) {
                window
                    .update_with_buffer(&buf, WIDTH, HEIGHT)
                    .expect("failed to update window");
            } else {
                window.update();
            }

            window_timer = Instant::now();
        }

        let pc = chip8.pc as usize;
        // Unwrap is ok, guaranteed to be correct size
        let instruction = u16::from_be_bytes(chip8.mem[pc..pc + 2].try_into().unwrap());
        debug!("Got instruction 0x{:X}", instruction);

        chip8.pc += 2;
        let Some(instruction) = Instruction::parse(instruction) else {
            log::error!("Failed to parse instruction, skipping");
            continue;
        };
        debug!("Parsed instruction: {:?}", instruction);

        display_modified = instruction.execute(&mut chip8, &mut buf, &window, &args.colors);

        // 700 Hz
        if instruction_timer.elapsed() < Duration::from_micros(1430) {
            std::thread::sleep(Duration::from_micros(1430) - instruction_timer.elapsed());
        }

        instruction_timer = Instant::now();
    }
}
