use std::io;
use std::io::Write;

use zeroize::Zeroizing;

#[derive(Default)]
pub struct WipingBuffer {
    bytes: Zeroizing<Vec<u8>>,
}

impl WipingBuffer {
    pub(crate) fn into_bytes(self) -> Zeroizing<Vec<u8>> {
        self.bytes
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        let required = self.bytes.len() + additional;

        if required > self.bytes.capacity() {
            let mut grown =
                Zeroizing::new(Vec::with_capacity(required.max(self.bytes.capacity() * 2)));

            grown.extend_from_slice(&self.bytes);
            self.bytes = grown;
        }
    }
}

impl Write for WipingBuffer {
    fn write(&mut self, written: &[u8]) -> io::Result<usize> {
        self.reserve(written.len());
        self.bytes.extend_from_slice(written);

        Ok(written.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::WipingBuffer;

    #[test]
    fn keeps_every_written_byte_while_it_grows() {
        let mut buffer = WipingBuffer::default();
        let chunk: Vec<u8> = (0..=u8::MAX).collect();

        for _ in 0..16 {
            buffer
                .write_all(&chunk)
                .expect("the buffer accepts every write");
        }

        buffer.flush().expect("the buffer flushes");

        assert_eq!(*buffer.into_bytes(), chunk.repeat(16));
    }
}
