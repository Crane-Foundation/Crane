#[allow(unused_imports)]
use cbvm::{
    builder::bytes::{Byte, ByteStream},
    byte,
    bytecode::{
        data::ByteData,
        ops::ArgType::*,
        ops::Operations::*,
        types::Types::{self, *},
    },
    constant,
    engine::{
        memory::{Heap, Stack},
        Engine,
    },
    op, stream, typed,
};

pub struct Int {
    pub value: i64,
}
impl From<Int> for Byte {
    fn from(int: Int) -> Self {
        let data: u64 = int
            .value
            .try_into()
            .unwrap_or((int.value as i128 & 0xFFFFFFFFFFFFFFFF) as u64);
        Byte {
            pos: 0,
            tp: TypeI64,
            data: Box::from(data),
        }
    }
}
impl Int {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

pub struct Float {
    pub value: f64,
}
impl From<Float> for Byte {
    fn from(float: Float) -> Self {
        let data: u64 = float.value.to_bits();
        Byte {
            pos: 0,
            tp: TypeF64,
            data: Box::from(data),
        }
    }
}

impl Float {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn mkstream(&self) -> ByteStream {
        ByteStream::new()
            .emit(op!(PUSH))
            .emitstream(stream!((TypeF64, self.value.to_bits())))
    }
}

pub struct Str {
    pub value: String,
}

impl From<Str> for Byte {
    fn from(string: Str) -> Self {
        let data: u64 = string.value.as_ptr() as u64;
        Byte {
            pos: 0,
            tp: TypeAddr,
            data: Box::from(data),
        }
    }
}

impl Str {
    pub fn new(value: String) -> Self {
        Self { value }
    }
    pub fn mkstream(&self) -> ByteStream {
        let mut stream = ByteStream::new();
        stream = stream
            .emit(op!(ALLOC))
            .emitstream(stream!((TypeReg, 0x1), (TypeU8, self.value.len() as u64)));
        let bytes = self.value.as_bytes().chunks(8);
        for byte in bytes {
            let mut data: u64 = 0;
            for (i, byte) in byte.iter().enumerate() {
                data |= (*byte as u64) << (i * 8);
            }
            stream = stream
                .emit(op!(STORE))
                .emitstream(stream!((TypeU8, 0x1), (TypeU64, data)));
        }
        stream
    }
}
pub trait CraneType {
    type New;
    fn mkstream(&self) -> ByteStream;
    fn new<T>(value: T) -> Self;
}

pub struct Bool {
    pub data: u8,
}
impl From<bool> for Bool {
    fn from(b: bool) -> Bool {
        Bool { data: b as u8 }
    }
}
#[allow(dead_code)]
impl Bool {
    pub fn new(d: bool) -> Self {
        Bool { data: d as u8 }
    }
    pub fn mkstream(&self) -> ByteStream {
        ByteStream::new()
            .emit(op!(PUSH))
            .emitstream(stream!((TypeU8, self.data as u64)))
    }
}
