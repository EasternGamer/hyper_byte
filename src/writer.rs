use crate::writers::traits::*;

pub struct FastByteWriter {
    byte_array: Vec<u8>,
}

impl Default for FastByteWriter {
    fn default() -> Self {
        Self::new()
    }
}
impl ByteWriter for FastByteWriter {
    fn to_vec(self) -> Vec<u8> {
        self.byte_array
    }

    fn as_slice(&self) -> &[u8] {
        self.byte_array.as_slice()
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        self.byte_array.as_mut()
    }
}
impl BigEndianByteWriter for FastByteWriter {}
impl LittleEndianByteWriter for FastByteWriter {}
impl NativeEndianByteWriter for FastByteWriter {}

impl FastByteWriter {
    /// Simple and fast byte writer, backed by a `Vec<u8>`
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::BigEndianByteWriter;
    ///
    /// let my_simple_tuple = (193f64, 2217f64);
    ///
    /// let slice = [0u8; 32];
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f64_be(my_simple_tuple.0);
    /// writer.write_f64_be(my_simple_tuple.1);
    ///
    /// ```
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            byte_array: Vec::new(),
        }
    }

    /// Custom buffer implementation
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::BigEndianByteWriter;
    ///
    /// let my_simple_tuple = (193f64, 2217f64);
    ///
    /// let buffer = Vec::with_capacity(32);
    /// let mut writer = FastByteWriter::from(buffer);
    /// writer.write_f64_be(my_simple_tuple.0);
    /// writer.write_f64_be(my_simple_tuple.1);
    ///
    /// ```
    #[inline(always)]
    pub const fn from(buffer: Vec<u8>) -> Self {
        Self { byte_array: buffer }
    }
}

pub struct NetworkWriter {
    byte_array: Vec<u8>,
}

impl Default for NetworkWriter {
    fn default() -> Self {
        Self::new()
    }
}
impl ByteWriter for NetworkWriter {
    fn to_vec(self) -> Vec<u8> {
        self.byte_array
    }

    fn as_slice(&self) -> &[u8] {
        self.byte_array.as_slice()
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        self.byte_array.as_mut()
    }
}
impl BigEndianByteWriter for NetworkWriter {}

impl NetworkWriter {
    /// Simple byte writer
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::writer::NetworkWriter;
    /// use hyper_byte::writers::traits::BigEndianByteWriter;
    ///
    /// let my_simple_tuple = (193f64, 2217f64);
    ///
    /// let slice = [0u8; 32];
    /// let mut writer = NetworkWriter::new();
    /// writer.write_f64_be(my_simple_tuple.0);
    /// writer.write_f64_be(my_simple_tuple.1);
    ///
    /// ```
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            byte_array: Vec::new(),
        }
    }

    /// Custom buffer implementation
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::writer::NetworkWriter;
    /// use hyper_byte::writers::traits::BigEndianByteWriter;
    ///
    /// let my_simple_tuple = (193f64, 2217f64);
    ///
    /// let buffer = Vec::with_capacity(32);
    /// let mut writer = NetworkWriter::from(buffer);
    /// writer.write_f64_be(my_simple_tuple.0);
    /// writer.write_f64_be(my_simple_tuple.1);
    ///
    /// ```
    #[inline(always)]
    pub const fn from(buffer: Vec<u8>) -> Self {
        Self { byte_array: buffer }
    }
}

pub struct LittleWriter {
    byte_array: Vec<u8>,
}

impl Default for LittleWriter {
    fn default() -> Self {
        Self::new()
    }
}
impl ByteWriter for LittleWriter {
    fn to_vec(self) -> Vec<u8> {
        self.byte_array
    }

    fn as_slice(&self) -> &[u8] {
        self.byte_array.as_slice()
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        self.byte_array.as_mut()
    }
}
impl LittleEndianByteWriter for LittleWriter {}

impl LittleWriter {
    /// Simple byte writer
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::writer::LittleWriter;
    /// use hyper_byte::writers::traits::LittleEndianByteWriter;
    ///
    /// let my_simple_tuple = (193f64, 2217f64);
    ///
    /// let slice = [0u8; 32];
    /// let mut writer = LittleWriter::new();
    /// writer.write_f64_le(my_simple_tuple.0);
    /// writer.write_f64_le(my_simple_tuple.1);
    ///
    /// ```
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            byte_array: Vec::new(),
        }
    }

    /// Custom buffer implementation
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::writer::LittleWriter;
    /// use hyper_byte::writers::traits::LittleEndianByteWriter;
    ///
    /// let my_simple_tuple = (193f64, 2217f64);
    ///
    /// let buffer = Vec::with_capacity(32);
    /// let mut writer = LittleWriter::from(buffer);
    /// writer.write_f64_le(my_simple_tuple.0);
    /// writer.write_f64_le(my_simple_tuple.1);
    ///
    /// ```
    #[inline(always)]
    pub const fn from(buffer: Vec<u8>) -> Self {
        Self { byte_array: buffer }
    }
}

pub struct NativeWriter {
    byte_array: Vec<u8>,
}

impl Default for NativeWriter {
    fn default() -> Self {
        Self::new()
    }
}
impl ByteWriter for NativeWriter {
    fn to_vec(self) -> Vec<u8> {
        self.byte_array
    }

    fn as_slice(&self) -> &[u8] {
        self.byte_array.as_slice()
    }

    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        self.byte_array.as_mut()
    }
}
impl NativeEndianByteWriter for NativeWriter {}

impl NativeWriter {
    /// Simple byte writer
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::writer::NativeWriter;
    /// use hyper_byte::writers::traits::NativeEndianByteWriter;
    ///
    /// let my_simple_tuple = (193f64, 2217f64);
    ///
    /// let slice = [0u8; 32];
    /// let mut writer = NativeWriter::new();
    /// writer.write_f64_ne(my_simple_tuple.0);
    /// writer.write_f64_ne(my_simple_tuple.1);
    ///
    /// ```
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            byte_array: Vec::new(),
        }
    }

    /// Custom buffer implementation
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::writer::NativeWriter;
    /// use hyper_byte::writers::traits::NativeEndianByteWriter;
    ///
    /// let my_simple_tuple = (193f64, 2217f64);
    ///
    /// let buffer = Vec::with_capacity(32);
    /// let mut writer = NativeWriter::from(buffer);
    /// writer.write_f64_ne(my_simple_tuple.0);
    /// writer.write_f64_ne(my_simple_tuple.1);
    ///
    /// ```
    #[inline(always)]
    pub const fn from(buffer: Vec<u8>) -> Self {
        Self { byte_array: buffer }
    }
}
