// Stuffen staffen (made with Claude).
pub trait _NumToBytes {
    type Bytes: AsRef<[u8]>;
    fn to_le_bytes(self) -> Self::Bytes;
}

// Implement for common integer types
impl _NumToBytes for u8 {
    type Bytes = [u8; 1];
    fn to_le_bytes(self) -> [u8; 1] {
        [self]
    }
}
impl _NumToBytes for u16 {
    type Bytes = [u8; 2];
    fn to_le_bytes(self) -> [u8; 2] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for u32 {
    type Bytes = [u8; 4];
    fn to_le_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for u64 {
    type Bytes = [u8; 8];
    fn to_le_bytes(self) -> [u8; 8] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for u128 {
    type Bytes = [u8; 16];
    fn to_le_bytes(self) -> [u8; 16] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for i8 {
    type Bytes = [u8; 1];
    fn to_le_bytes(self) -> [u8; 1] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for i16 {
    type Bytes = [u8; 2];
    fn to_le_bytes(self) -> [u8; 2] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for i32 {
    type Bytes = [u8; 4];
    fn to_le_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for i64 {
    type Bytes = [u8; 8];
    fn to_le_bytes(self) -> [u8; 8] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for i128 {
    type Bytes = [u8; 16];
    fn to_le_bytes(self) -> [u8; 16] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for f32 {
    type Bytes = [u8; 4];
    fn to_le_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}
impl _NumToBytes for f64 {
    type Bytes = [u8; 8];
    fn to_le_bytes(self) -> [u8; 8] {
        self.to_le_bytes()
    }
}

pub fn _slice_to_bytes<T>(slice: &[T]) -> Vec<u8>
where
    T: _NumToBytes + Copy,
{
    slice
        .iter()
        .flat_map(|&item| item.to_le_bytes().as_ref().to_vec())
        .collect()
}
