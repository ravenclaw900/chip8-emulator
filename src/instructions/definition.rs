use crate::registers::Register;

#[derive(Debug)]
pub enum Instruction {
    ClearDisplay,
    ReturnSubroutine,
    Jump {
        addr: u16,
    },
    CallSubroutine {
        addr: u16,
    },
    SkipEq {
        reg: Register,
        num: u8,
    },
    SkipNe {
        reg: Register,
        num: u8,
    },
    SkipEqReg {
        reg1: Register,
        reg2: Register,
    },
    SkipNeReg {
        reg1: Register,
        reg2: Register,
    },
    Set {
        reg: Register,
        val: u8,
    },
    Add {
        reg: Register,
        val: u8,
    },
    SetReg {
        reg1: Register,
        reg2: Register,
    },
    Or {
        reg1: Register,
        reg2: Register,
    },
    And {
        reg1: Register,
        reg2: Register,
    },
    Xor {
        reg1: Register,
        reg2: Register,
    },
    AddReg {
        reg1: Register,
        reg2: Register,
    },
    Sub1 {
        reg1: Register,
        reg2: Register,
    },
    Sub2 {
        reg1: Register,
        reg2: Register,
    },
    Shr {
        reg1: Register,
        reg2: Register,
    },
    Shl {
        reg1: Register,
        reg2: Register,
    },
    SetIndex {
        val: u16,
    },
    JumpOffset {
        addr: u16,
    },
    Rand {
        outreg: Register,
        val: u8,
    },
    Display {
        xreg: Register,
        yreg: Register,
        height: u8,
    },
    SkipIfKey {
        keyreg: Register,
    },
    SkipIfNotKey {
        keyreg: Register,
    },
    GetDelayTimer {
        outreg: Register,
    },
    SetDelayTimer {
        inreg: Register,
    },
    SetSoundTimer {
        inreg: Register,
    },
    AddToIndex {
        inreg: Register,
    },
    WaitForKey {
        keyreg: Register,
    },
    GetFontChar {
        inreg: Register,
    },
    BinToDec {
        inreg: Register,
    },
    StoreMem {
        inreg_max: Register,
    },
    LoadMem {
        outreg_max: Register,
    },
}
