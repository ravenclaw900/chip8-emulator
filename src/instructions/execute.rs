use minifb::Window;

use crate::{chip8::Chip8, cli, keypad, registers::Register, HEIGHT, WIDTH};

use super::Instruction;

fn clear_display(display_buf: &mut [u32], colors: &cli::Colors) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            crate::display::write_to_buffer(
                display_buf,
                x,
                y,
                crate::display::Mode::SetFalse,
                colors,
            );
        }
    }
}

fn set(chip8: &mut Chip8, reg: Register, val: u8) {
    chip8.registers[reg] = val;
}

fn set_index(chip8: &mut Chip8, val: u16) {
    chip8.registers.index = val;
}

fn display(
    chip8: &mut Chip8,
    display_buf: &mut [u32],
    xreg: Register,
    yreg: Register,
    height: u8,
    colors: &cli::Colors,
) {
    let sprite_data = &chip8.mem
        [chip8.registers.index as usize..chip8.registers.index as usize + height as usize];
    // Module width and height to allow for wrapping
    let start_x = chip8.registers[xreg] as usize % WIDTH;
    let start_y = chip8.registers[yreg] as usize % HEIGHT;
    // Small routine to extract the bits of the individual values
    let sprite_data = sprite_data
        .iter()
        .map(|x| {
            // Reverse bits because returned array would otherwise be backwarks
            // (indexing from the right)
            let x = x.reverse_bits();
            // Bitwise and with shifted '1' to extract bits
            // Example: 0b011001001 & 0b00001000
            let arr: [bool; 8] = std::array::from_fn(|idx| x & (1 << idx) != 0);

            arr.into_iter().enumerate()
        })
        .enumerate();

    for (y_offset, line) in sprite_data {
        if start_y + y_offset < HEIGHT {
            for (x_offset, should_toggle) in line {
                if should_toggle && start_x + x_offset < WIDTH {
                    let collision = crate::display::write_to_buffer(
                        display_buf,
                        start_x + x_offset,
                        start_y + y_offset,
                        crate::display::Mode::Toggle,
                        colors,
                    );
                    // Set VF register to either 1 or 0, depending on whether 2 sprites collided
                    chip8.registers[Register::VF] = u8::from(collision);
                }
            }
        }
    }
}

fn jump(chip8: &mut Chip8, addr: u16) {
    chip8.pc = addr;
}

fn add(chip8: &mut Chip8, reg: Register, val: u8) {
    // Adding should wrap around
    chip8.registers[reg] = chip8.registers[reg].wrapping_add(val);
}

fn call_subroutine(chip8: &mut Chip8, addr: u16) {
    chip8.stack.push(chip8.pc);
    chip8.pc = addr;
}

fn return_subroutine(chip8: &mut Chip8) {
    let addr = chip8.stack.pop();
    chip8.pc = addr;
}

fn skip_eq(chip8: &mut Chip8, reg: Register, num: u8) {
    if chip8.registers[reg] == num {
        chip8.pc += 2;
    }
}

fn skip_ne(chip8: &mut Chip8, reg: Register, num: u8) {
    if chip8.registers[reg] != num {
        chip8.pc += 2;
    }
}

fn skip_eq_reg(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    if chip8.registers[reg1] == chip8.registers[reg2] {
        chip8.pc += 2;
    }
}

fn skip_ne_reg(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    if chip8.registers[reg1] != chip8.registers[reg2] {
        chip8.pc += 2;
    }
}

fn set_reg(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    chip8.registers[reg1] = chip8.registers[reg2];
}

fn or(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    chip8.registers[reg1] |= chip8.registers[reg2];
}

fn and(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    chip8.registers[reg1] &= chip8.registers[reg2];
}

fn xor(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    chip8.registers[reg1] ^= chip8.registers[reg2];
}

fn add_reg(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    let (result, overflow) = chip8.registers[reg1].overflowing_add(chip8.registers[reg2]);
    chip8.registers[reg1] = result;
    chip8.registers[Register::VF] = u8::from(overflow);
}

fn sub1(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    let (result, overflow) = chip8.registers[reg1].overflowing_sub(chip8.registers[reg2]);
    chip8.registers[reg1] = result;
    // Subtraction affects carry flag in the opposite way
    chip8.registers[Register::VF] = u8::from(!overflow);
}

fn sub2(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    let (result, overflow) = chip8.registers[reg2].overflowing_sub(chip8.registers[reg1]);
    chip8.registers[reg1] = result;
    // Subtraction affects carry flag in the opposite way
    chip8.registers[Register::VF] = u8::from(!overflow);
}

fn shr(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    let num = chip8.registers[reg2];
    chip8.registers[reg1] = num >> 1;
    // Check if bit that was shifted is a 1 or a 0
    let shifted = u8::from(num & 0b0000_0001 != 0);
    chip8.registers[Register::VF] = shifted;
}

fn shl(chip8: &mut Chip8, reg1: Register, reg2: Register) {
    let num = chip8.registers[reg2];
    chip8.registers[reg1] = num << 1;
    // Check if bit that was shifted is a 1 or a 0
    let shifted = u8::from(num & 0b1000_0000 != 0);
    chip8.registers[Register::VF] = shifted;
}

fn load_mem(chip8: &mut Chip8, outreg_max: Register) {
    let reg_iter = Register::iter_until(outreg_max);
    for (idx, reg) in reg_iter.enumerate() {
        chip8.registers[reg] = chip8.mem[chip8.registers.index as usize + idx];
    }
}

fn store_mem(chip8: &mut Chip8, inreg_max: Register) {
    let reg_iter = Register::iter_until(inreg_max);
    for (idx, reg) in reg_iter.enumerate() {
        chip8.mem[chip8.registers.index as usize + idx] = chip8.registers[reg];
    }
}

fn bin_to_dec(chip8: &mut Chip8, inreg: Register) {
    let num = chip8.registers[inreg];
    let digit1 = num / 100;
    let digit2 = num / 10 % 10;
    let digit3 = num % 10;

    let index = chip8.registers.index as usize;

    chip8.mem[index] = digit1;
    chip8.mem[index + 1] = digit2;
    chip8.mem[index + 2] = digit3;
}

fn add_to_index(chip8: &mut Chip8, inreg: Register) {
    chip8.registers.index += u16::from(chip8.registers[inreg]);
}

fn rand(chip8: &mut Chip8, outreg: Register, val: u8) {
    let rand = fastrand::u8(u8::MIN..u8::MAX) & val;
    chip8.registers[outreg] = rand;
}

fn get_delay_timer(chip8: &mut Chip8, outreg: Register) {
    chip8.registers[outreg] = chip8.delay_timer;
}

fn set_delay_timer(chip8: &mut Chip8, inreg: Register) {
    chip8.delay_timer = chip8.registers[inreg];
}

fn wait_for_key(chip8: &mut Chip8, window: &Window, keyreg: Register) {
    match keypad::get_pressed_key(window) {
        Some(key) => chip8.registers[keyreg] = key,
        None => chip8.pc -= 2,
    }
}

fn get_font_char(chip8: &mut Chip8, inreg: Register) {
    let key_offset = u16::from(chip8.registers[inreg]);
    // Key data starts at 0x50 and each key takes up 0x5 bytes
    let addr = 0x50 + key_offset * 0x5;
    chip8.registers.index = addr;
}

fn jump_offset(chip8: &mut Chip8, addr: u16) {
    chip8.pc = addr + u16::from(chip8.registers[Register::V0]);
}

fn set_sound_timer(chip8: &mut Chip8, inreg: Register) {
    chip8.sound_timer = chip8.registers[inreg];
}

fn skip_if_key(chip8: &mut Chip8, window: &Window, keyreg: Register) {
    // If key is invalid, assume it isn't pressed
    if keypad::is_key_pressed(window, chip8.registers[keyreg]).unwrap_or(false) {
        chip8.pc += 2;
    }
}

fn skip_if_not_key(chip8: &mut Chip8, window: &Window, keyreg: Register) {
    // If key is invalid, assume it isn't pressed
    if !keypad::is_key_pressed(window, chip8.registers[keyreg]).unwrap_or(false) {
        chip8.pc += 2;
    }
}

pub enum DisplayModified {
    Unchanged,
    Changed,
}

impl Instruction {
    pub fn execute(
        &self,
        chip8: &mut Chip8,
        display_buf: &mut [u32],
        window: &Window,
        colors: &cli::Colors,
    ) -> DisplayModified {
        let mut modified = DisplayModified::Unchanged;

        match *self {
            Self::ClearDisplay => {
                clear_display(display_buf, colors);
                modified = DisplayModified::Changed;
            }
            Self::Set { reg, val } => set(chip8, reg, val),
            Self::SetIndex { val } => set_index(chip8, val),
            Self::Display { xreg, yreg, height } => {
                display(chip8, display_buf, xreg, yreg, height, colors);
                modified = DisplayModified::Changed;
            }
            Self::Jump { addr } => jump(chip8, addr),
            Self::Add { reg, val } => add(chip8, reg, val),
            Self::CallSubroutine { addr } => call_subroutine(chip8, addr),
            Self::ReturnSubroutine => return_subroutine(chip8),
            Self::SkipEq { reg, num } => skip_eq(chip8, reg, num),
            Self::SkipNe { reg, num } => skip_ne(chip8, reg, num),
            Self::SkipEqReg { reg1, reg2 } => skip_eq_reg(chip8, reg1, reg2),
            Self::SkipNeReg { reg1, reg2 } => skip_ne_reg(chip8, reg1, reg2),
            Self::SetReg { reg1, reg2 } => set_reg(chip8, reg1, reg2),
            Self::Or { reg1, reg2 } => or(chip8, reg1, reg2),
            Self::And { reg1, reg2 } => and(chip8, reg1, reg2),
            Self::Xor { reg1, reg2 } => xor(chip8, reg1, reg2),
            Self::AddReg { reg1, reg2 } => add_reg(chip8, reg1, reg2),
            Self::Sub1 { reg1, reg2 } => sub1(chip8, reg1, reg2),
            Self::Sub2 { reg1, reg2 } => sub2(chip8, reg1, reg2),
            Self::Shr { reg1, reg2 } => shr(chip8, reg1, reg2),
            Self::Shl { reg1, reg2 } => shl(chip8, reg1, reg2),
            Self::LoadMem { outreg_max } => load_mem(chip8, outreg_max),
            Self::StoreMem { inreg_max } => store_mem(chip8, inreg_max),
            Self::BinToDec { inreg } => bin_to_dec(chip8, inreg),
            Self::AddToIndex { inreg } => add_to_index(chip8, inreg),
            Self::Rand { outreg, val } => rand(chip8, outreg, val),
            Self::GetDelayTimer { outreg } => get_delay_timer(chip8, outreg),
            Self::SetDelayTimer { inreg } => set_delay_timer(chip8, inreg),
            Self::WaitForKey { keyreg } => wait_for_key(chip8, window, keyreg),
            Self::GetFontChar { inreg } => get_font_char(chip8, inreg),
            Self::JumpOffset { addr } => jump_offset(chip8, addr),
            Self::SetSoundTimer { inreg } => set_sound_timer(chip8, inreg),
            Self::SkipIfKey { keyreg } => skip_if_key(chip8, window, keyreg),
            Self::SkipIfNotKey { keyreg } => skip_if_not_key(chip8, window, keyreg),
        }

        modified
    }
}
