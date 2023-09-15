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

    pub fn push(&mut self, val: u16) {
        // Index should range from 0..STACK_LEN (exclusive)
        // Incremented at end of previous run, so if push is attempted and pointer is above max len,
        // then stack will overflow (should crash here, non-recoverable error)
        assert!(self.cur_idx < STACK_LEN, "emulated program stack overflow");
        self.data[self.cur_idx] = val;
        self.cur_idx += 1;
    }

    pub fn pop(&mut self) -> u16 {
        // Make sure stack counter doesn't 'underflow' (wrap around)
        // Should crash, as program might expect non-existent value
        self.cur_idx = self
            .cur_idx
            .checked_sub(1)
            .expect("emulated program popped from empty stack");
        // Old values are not overwritten, but shouldn't cause any problems
        self.data[self.cur_idx]
    }
}
