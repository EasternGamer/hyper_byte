use crate::readers::traits::{
    BigEndianByteReader, ByteReader, LittleEndianByteReader, NativeEndianByteReader,
};
use crate::writers::traits::{
    BigEndianByteWriter, ByteWriter, LittleEndianByteWriter, NativeEndianByteWriter,
};

pub struct NetworkStream {
    src: Vec<u8>,
    index: usize,
}

impl NetworkStream {
    pub fn new(src: Vec<u8>) -> Self {
        Self { src, index: 0 }
    }
}

impl ByteReader for NetworkStream {
    fn byte_array(&mut self) -> &[u8] {
        &self.src[self.index..]
    }

    unsafe fn advance(&mut self, advancement: usize, _: usize) {
        self.index += advancement;
    }
}

impl BigEndianByteReader for NetworkStream {}

impl ByteWriter for NetworkStream {
    fn to_vec(self) -> Vec<u8> {
        self.src
    }

    fn as_slice(&self) -> &[u8] {
        &self.src
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.src
    }
}

impl BigEndianByteWriter for NetworkStream {}

pub struct LittleStream {
    src: Vec<u8>,
    index: usize,
}

impl LittleStream {
    pub fn new(src: Vec<u8>) -> Self {
        Self { src, index: 0 }
    }
}

impl ByteReader for LittleStream {
    fn byte_array(&mut self) -> &[u8] {
        &self.src[self.index..]
    }

    unsafe fn advance(&mut self, advancement: usize, _: usize) {
        self.index += advancement;
    }
}

impl LittleEndianByteReader for LittleStream {}

impl ByteWriter for LittleStream {
    fn to_vec(self) -> Vec<u8> {
        self.src
    }

    fn as_slice(&self) -> &[u8] {
        &self.src
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.src
    }
}

impl LittleEndianByteWriter for LittleStream {}

pub struct NativeStream {
    src: Vec<u8>,
    index: usize,
}

impl NativeStream {
    pub fn new(src: Vec<u8>) -> Self {
        Self { src, index: 0 }
    }
}

impl ByteReader for NativeStream {
    fn byte_array(&mut self) -> &[u8] {
        &self.src[self.index..]
    }

    unsafe fn advance(&mut self, advancement: usize, _: usize) {
        self.index += advancement;
    }
}

impl NativeEndianByteReader for NativeStream {}

impl ByteWriter for NativeStream {
    fn to_vec(self) -> Vec<u8> {
        self.src
    }

    fn as_slice(&self) -> &[u8] {
        &self.src
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.src
    }
}

impl NativeEndianByteWriter for NativeStream {}

pub struct HyperStream {
    src: Vec<u8>,
    index: usize,
}

impl HyperStream {
    pub fn new(src: Vec<u8>) -> Self {
        Self { src, index: 0 }
    }
}

impl ByteReader for HyperStream {
    fn byte_array(&mut self) -> &[u8] {
        &self.src[self.index..]
    }

    unsafe fn advance(&mut self, advancement: usize, _: usize) {
        self.index += advancement;
    }
}

impl BigEndianByteReader for HyperStream {}
impl LittleEndianByteReader for HyperStream {}
impl NativeEndianByteReader for HyperStream {}

impl ByteWriter for HyperStream {
    fn to_vec(self) -> Vec<u8> {
        self.src
    }

    fn as_slice(&self) -> &[u8] {
        &self.src
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.src
    }
}

impl BigEndianByteWriter for HyperStream {}
impl LittleEndianByteWriter for HyperStream {}
impl NativeEndianByteWriter for HyperStream {}
