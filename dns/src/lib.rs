use std::io::Error;

pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}
impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    // Current position in buffer
    fn pos(&self) -> usize {
        self.pos
    }

    // Step forward step size in buffer
    fn step(&mut self, steps: usize) -> Result<(), Error> {
        self.pos += steps;
        Ok(())
    }

    // Change buffer position
    fn seek(&mut self, pos: usize) -> Result<(), Error> {
        self.pos = pos;
        Ok(())
    }

    // Get a single byte, without changing buffer position
    fn get(&mut self, pos: usize) -> Result<u8, Error> {
        if pos >= 512 {
            return Err("End of buffer".into());
        }
        Ok(self.buf[pos])
    }
}

pub struct Header {
    pub id: u16,
    // Flags
    pub qr: bool,
    pub opcode: u8,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: bool,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

pub struct Question {
    pub qname: String,
    pub qtype: u8,
    pub qclass: u8,
}

pub struct Answer {}
pub struct Authority {}
pub struct Additional {}
