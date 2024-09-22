use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    wrapper: R,
    size: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapper: wrapped,
            size: 0,
            reads: 1, // On creation, we read entire content....
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapper
    }

    pub fn bytes_through(&self) -> usize {
        self.size
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read = self.wrapper.read(buf);
        match read {
            Ok(0) => Ok(0),
            Ok(n) => {
                self.size += n;
                self.reads += 1;
                read
            }
            Err(e) => Err(e),
        }
    }
}

pub struct WriteStats<W> {
    wrapper: W,
    bytes: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapper: W) -> WriteStats<W> {
        WriteStats {
            wrapper,
            bytes: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapper
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let write = self.wrapper.write(buf);
        match write {
            Ok(0) => Ok(0),
            Ok(n) => {
                self.bytes += n;
                self.writes += 1;
                write
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }
}
