use anyhow::Context;

const STACK_LEN: usize = 16;

#[derive(Debug)]
pub struct Stack {
    data: [u16; STACK_LEN],
    cur_idx: usize,
}

impl Stack {
    pub const fn new() -> Self {
        Self {
            data: [0; STACK_LEN],
            cur_idx: 0,
        }
    }

    pub fn push(&mut self, val: u16) -> anyhow::Result<()> {
        self.cur_idx += 1;
        // Index should range from 0..STACK_LEN (exclusive) to not access out-of-bounds memory
        anyhow::ensure!(self.cur_idx < STACK_LEN, "emulated program stack overflow");
        self.data[self.cur_idx] = val;
        Ok(())
    }

    pub fn pop(&mut self) -> anyhow::Result<u16> {
        let val = self.data[self.cur_idx];
        // Make sure stack counter doesn't 'underflow' (wrap around)
        // Old values are not overwritten, but shouldn't cause any problems
        self.cur_idx = self
            .cur_idx
            .checked_sub(1)
            .context("emulated program popped from empty stack")?;
        Ok(val)
    }
}
