/// FNV-1a. Used to detect accidental desyncs, not to resist tampering.
pub(crate) struct Hasher(u64);

impl Hasher {
    pub fn new() -> Self {
        Hasher(0xcbf2_9ce4_8422_2325)
    }

    /// All values are widened to u64 so 32-bit (wasm) and 64-bit hosts agree.
    pub fn write(&mut self, value: u64) {
        for byte in value.to_le_bytes() {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    pub fn write_i32(&mut self, value: i32) {
        self.write(u64::from(value as u32));
    }

    pub fn write_str(&mut self, value: &str) {
        self.write(value.len() as u64);
        for byte in value.bytes() {
            self.write(u64::from(byte));
        }
    }

    pub fn finish(&self) -> u64 {
        self.0
    }
}
