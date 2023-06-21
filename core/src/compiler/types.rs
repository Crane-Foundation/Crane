#[allow(unused_imports)]
use cbvm::{
    builder::{
        bytes::{ByteStream, Byte},
    },
    bytecode::{
        data::ByteData,
        ops::ArgType::*,
        ops::Operations::*,
        types::Types::{self, *},
    },
    engine::{
        memory::{Heap, Stack},
        Engine,
    },
    byte, stream, op, constant, typed
};

pub struct Int {
    pub value: i64,
}
impl From<Int> for Byte {
    fn from(int: Int) -> Self {
        let data: u64 = int.value.try_into().unwrap_or((int.value as i128 & 0xFFFFFFFFFFFFFFFF) as u64);
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
    pub fn mkstream (&self) -> ByteStream {
        ByteStream::new().emit(op!(PUSH))
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
        stream = stream.emit(op!(ALLOC))
            .emitstream(stream!(
                (TypeReg, 0x1),
                (TypeU8, self.value.len() as u64)
            ));
        let mut bytes = self.value.as_bytes().chunks(8);
        while let Some(byte) = bytes.next() {
            let mut data: u64 = 0;
            for (i, byte) in byte.iter().enumerate() {
                data |= (*byte as u64) << (i * 8);
            }
            stream = stream.emit(op!(STORE))
                .emitstream(stream!((TypeU8, 0x1), (TypeU64, data)));
        }
        stream
    }
}