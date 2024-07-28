#![allow(dead_code)]
extern crate cbvm;
use crate::compiler::types;
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
#[allow(dead_code)]
pub enum ConstantType {
    Str(String),
    Int(u32),
}

type Ptr = u64;
type Streams = Vec<ByteStream>;

#[allow(dead_code)]
pub struct Constants {
    ptr: Ptr,
    stream: ByteStream,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            ptr: 0,
            stream: ByteStream::new(),
        }
    }
    pub fn add(&mut self, c: ConstantType) -> Option<()> {
        self.stream.emitstream(c.mkstream());
        Some(())
    }
}

#[allow(dead_code)]
impl ConstantType {
    pub fn mkstream(self) -> ByteStream {
        match self {
            ConstantType::Str(s) => types::Str::new(s).mkstream(),
            ConstantType::Int(i) => ByteStream::new()
                .emit(op!(PUSH))
                .emitstream(stream!((TypeU64, i as u64))),
        }
    }
}
