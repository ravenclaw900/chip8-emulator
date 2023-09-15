use crate::registers::Register;

use super::Instruction;

struct InstructionParts {
    full: u16,
    cat: u8,
    x: Register,
    y: Register,
    n: u8,
    nn: u8,
    nnn: u16,
}

impl Instruction {
    pub fn parse(instruction: u16) -> Option<Self> {
        // Various parts of the instruction
        // (All unwraps are safe because of bitmasks)
        let parts = InstructionParts {
            full: instruction,
            // Get first nibble
            cat: (instruction >> 12).try_into().unwrap(),
            // Get second nibble
            x: Register::from_u16((instruction >> 8) & 0x0F)?,
            // Get third nibble
            y: Register::from_u16((instruction >> 4) & 0x00F)?,
            // Get fourth nibble
            n: (instruction & 0x000F).try_into().unwrap(),
            // Get second byte (third and fourth nibble)
            nn: (instruction & 0x00FF).try_into().unwrap(),
            // Get last three nibbles
            nnn: instruction & 0x0FFF,
        };

        Self::parse_instruction(&parts)
    }

    fn parse_instruction(parts: &InstructionParts) -> Option<Self> {
        Some(match *parts {
            InstructionParts { full: 0x00E0, .. } => Self::ClearDisplay,
            InstructionParts { full: 0x00EE, .. } => Self::ReturnSubroutine,
            InstructionParts { cat: 0x1, nnn, .. } => Self::Jump { addr: nnn },
            InstructionParts { cat: 0x2, nnn, .. } => Self::CallSubroutine { addr: nnn },
            InstructionParts {
                cat: 0x3, x, nn, ..
            } => Self::SkipEq { reg: x, num: nn },
            InstructionParts {
                cat: 0x4, x, nn, ..
            } => Self::SkipNe { reg: x, num: nn },
            InstructionParts {
                cat: 0x5,
                x,
                y,
                n: 0x0,
                ..
            } => Self::SkipEqReg { reg1: x, reg2: y },
            InstructionParts {
                cat: 0x9,
                x,
                y,
                n: 0x0,
                ..
            } => Self::SkipNeReg { reg1: x, reg2: y },
            InstructionParts {
                cat: 0x6, x, nn, ..
            } => Self::Set { reg: x, val: nn },
            InstructionParts {
                cat: 0x7, x, nn, ..
            } => Self::Add { reg: x, val: nn },
            // Delegate to specific login instruction function
            InstructionParts { cat: 0x8, .. } => Self::parse_logic_instruction(parts)?,
            InstructionParts { cat: 0xA, nnn, .. } => Self::SetIndex { val: nnn },
            InstructionParts { cat: 0xB, nnn, .. } => Self::JumpOffset { addr: nnn },
            InstructionParts {
                cat: 0xC, x, nn, ..
            } => Self::Rand { outreg: x, val: nn },
            InstructionParts {
                cat: 0xD, x, y, n, ..
            } => Self::Display {
                xreg: x,
                yreg: y,
                height: n,
            },
            InstructionParts {
                cat: 0xE,
                x,
                nn: 0x9E,
                ..
            } => Self::SkipIfKey { keyreg: x },
            InstructionParts {
                cat: 0xE,
                x,
                nn: 0xA1,
                ..
            } => Self::SkipIfNotKey { keyreg: x },
            // Delegate to misc 0xF category instruction function
            InstructionParts { cat: 0xF, .. } => Self::parse_category_f_instruction(parts)?,
            _ => {
                log::error!("invalid instruction {:X}", parts.full);
                return None;
            }
        })
    }

    fn parse_logic_instruction(parts: &InstructionParts) -> Option<Self> {
        // All instructions in this are category 0x8, so no need to check that
        Some(match *parts {
            InstructionParts { x, y, n: 0x0, .. } => Self::SetReg { reg1: x, reg2: y },
            InstructionParts { x, y, n: 0x1, .. } => Self::Or { reg1: x, reg2: y },
            InstructionParts { x, y, n: 0x2, .. } => Self::And { reg1: x, reg2: y },
            InstructionParts { x, y, n: 0x3, .. } => Self::Xor { reg1: x, reg2: y },
            InstructionParts { x, y, n: 0x4, .. } => Self::AddReg { reg1: x, reg2: y },
            InstructionParts { x, y, n: 0x5, .. } => Self::Sub1 { reg1: x, reg2: y },
            InstructionParts { x, y, n: 0x7, .. } => Self::Sub2 { reg1: x, reg2: y },
            InstructionParts { x, y, n: 0x6, .. } => Self::Shr { reg1: x, reg2: y },
            InstructionParts { x, y, n: 0xE, .. } => Self::Shl { reg1: x, reg2: y },
            _ => {
                log::error!("invalid logic instruction {:X}", parts.full);
                return None;
            }
        })
    }

    fn parse_category_f_instruction(parts: &InstructionParts) -> Option<Self> {
        // All instructions in this are category 0xF, so no need to check that
        Some(match *parts {
            InstructionParts { x, nn: 0x07, .. } => Self::GetDelayTimer { outreg: x },
            InstructionParts { x, nn: 0x15, .. } => Self::SetDelayTimer { inreg: x },
            InstructionParts { x, nn: 0x18, .. } => Self::SetSoundTimer { inreg: x },
            InstructionParts { x, nn: 0x1E, .. } => Self::AddToIndex { inreg: x },
            InstructionParts { x, nn: 0x0A, .. } => Self::WaitForKey { keyreg: x },
            InstructionParts { x, nn: 0x29, .. } => Self::GetFontChar { inreg: x },
            InstructionParts { x, nn: 0x33, .. } => Self::BinToDec { inreg: x },
            InstructionParts { x, nn: 0x55, .. } => Self::StoreMem { inreg_max: x },
            InstructionParts { x, nn: 0x65, .. } => Self::LoadMem { outreg_max: x },
            _ => {
                log::error!("invalid category F instruction {:X}", parts.full);
                return None;
            }
        })
    }
}
