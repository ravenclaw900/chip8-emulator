use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl Register {
    pub fn iter_until(max: Self) -> impl Iterator<Item = Self> {
        let variants = [
            Self::V0,
            Self::V1,
            Self::V2,
            Self::V3,
            Self::V4,
            Self::V5,
            Self::V6,
            Self::V7,
            Self::V8,
            Self::V9,
            Self::VA,
            Self::VB,
            Self::VC,
            Self::VD,
            Self::VE,
            Self::VF,
        ];
        let index = variants.into_iter().position(|x| x == max).unwrap();
        variants.into_iter().take(index + 1)
    }

    pub fn from_u16(val: u16) -> Option<Self> {
        Some(match val {
            0 => Self::V0,
            1 => Self::V1,
            2 => Self::V2,
            3 => Self::V3,
            4 => Self::V4,
            5 => Self::V5,
            6 => Self::V6,
            7 => Self::V7,
            8 => Self::V8,
            9 => Self::V9,
            10 => Self::VA,
            11 => Self::VB,
            12 => Self::VC,
            13 => Self::VD,
            14 => Self::VE,
            15 => Self::VF,
            _ => {
                log::error!("invalid register index");
                return None;
            }
        })
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    pub index: u16,
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
}

impl Registers {
    pub fn new() -> Self {
        // Should init everything to 0
        Self::default()
    }
}

impl Index<Register> for Registers {
    type Output = u8;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::V0 => &self.v0,
            Register::V1 => &self.v1,
            Register::V2 => &self.v2,
            Register::V3 => &self.v3,
            Register::V4 => &self.v4,
            Register::V5 => &self.v5,
            Register::V6 => &self.v6,
            Register::V7 => &self.v7,
            Register::V8 => &self.v8,
            Register::V9 => &self.v9,
            Register::VA => &self.va,
            Register::VB => &self.vb,
            Register::VC => &self.vc,
            Register::VD => &self.vd,
            Register::VE => &self.ve,
            Register::VF => &self.vf,
        }
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::V0 => &mut self.v0,
            Register::V1 => &mut self.v1,
            Register::V2 => &mut self.v2,
            Register::V3 => &mut self.v3,
            Register::V4 => &mut self.v4,
            Register::V5 => &mut self.v5,
            Register::V6 => &mut self.v6,
            Register::V7 => &mut self.v7,
            Register::V8 => &mut self.v8,
            Register::V9 => &mut self.v9,
            Register::VA => &mut self.va,
            Register::VB => &mut self.vb,
            Register::VC => &mut self.vc,
            Register::VD => &mut self.vd,
            Register::VE => &mut self.ve,
            Register::VF => &mut self.vf,
        }
    }
}
