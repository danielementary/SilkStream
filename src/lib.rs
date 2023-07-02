use std::fmt;

pub struct Datagram {
    data: [u8; 1024],
}

impl Datagram {
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn new(message: &[u8]) -> Datagram {
        assert!(message.len() <= 1024, "Data is too large");

        let mut data = [0u8; 1024];
        data[..message.len()].copy_from_slice(message);

        Datagram { data }
    }
}

impl fmt::Display for Datagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = String::from_utf8_lossy(&self.data);
        write!(f, "{}", message)
    }
}
