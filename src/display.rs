use crate::{cli, WIDTH};

#[derive(Clone, Copy)]
pub enum Mode {
    SetFalse,
    Toggle,
}

// Returns if pixel was toggled from true to false
pub fn write_to_buffer(
    buf: &mut [u32],
    x: usize,
    y: usize,
    mode: Mode,
    colors: &cli::Colors,
) -> bool {
    assert!((0..64).contains(&x), "x coordinate out of bounds");
    assert!((0..32).contains(&y), "y coordinate out of bounds");

    let mut collision = false;

    let offset = (y * WIDTH) + x;
    let color = match mode {
        Mode::SetFalse => colors.background,
        Mode::Toggle => {
            let current_state = buf[offset];
            if current_state == colors.background {
                colors.foreground
            } else {
                collision = true;
                colors.background
            }
        }
    };

    buf[offset] = color;

    collision
}
