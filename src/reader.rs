use crate::readers::traits::*;
use std::ptr::slice_from_raw_parts;

/// Cheap byte readers, which does not hold your hand. If you mess up, it will panic.
///
/// # Examples
/// ```
/// use hyper_byte::reader::FastByteReader;
/// use hyper_byte::readers::traits::NativeEndianByteReader;
///
/// let slice = [0u8; 32];
/// let mut readers = FastByteReader::new(&slice);
/// let x = readers.read_f64_ne();
/// let y = readers.read_f64_ne();
/// ```
pub struct FastByteReader<'reader> {
    byte_array: &'reader [u8],
}

impl ByteReader for FastByteReader<'_> {
    fn byte_array(&mut self) -> &[u8] {
        self.byte_array
    }

    unsafe fn advance(&mut self, advancement: usize, new_size: usize) {
        self.byte_array =
            &*slice_from_raw_parts(self.byte_array.as_ptr().add(advancement), new_size);
    }
}

impl BigEndianByteReader for FastByteReader<'_> {}
impl LittleEndianByteReader for FastByteReader<'_> {}
impl NativeEndianByteReader for FastByteReader<'_> {}

impl<'reader> FastByteReader<'reader> {
    /// Cheap byte readers, which does not hold your hand. If you mess up, it will panic.
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    ///
    /// let slice = [0u8; 32];
    /// let mut readers = FastByteReader::new(&slice);
    /// let x = readers.read_f64_ne();
    /// let y = readers.read_f64_ne();
    /// ```
    pub const fn new(byte_array: &'reader [u8]) -> Self {
        Self { byte_array }
    }
}

/// Cheap network-focused (big-endian) byte reader, which does not hold your hand. If you mess up, it will panic.
///
/// # Examples
/// ```
/// use hyper_byte::reader::NetworkReader;
/// use hyper_byte::readers::traits::BigEndianByteReader;
///
/// let slice = [0u8; 32];
/// let mut readers = NetworkReader::new(&slice);
/// let x = readers.read_f64_be();
/// let y = readers.read_f64_be();
/// ```
pub struct NetworkReader<'reader> {
    byte_array: &'reader [u8],
}

impl ByteReader for NetworkReader<'_> {
    fn byte_array(&mut self) -> &[u8] {
        self.byte_array
    }

    unsafe fn advance(&mut self, advancement: usize, new_size: usize) {
        self.byte_array =
            &*slice_from_raw_parts(self.byte_array.as_ptr().add(advancement), new_size);
    }
}

impl BigEndianByteReader for NetworkReader<'_> {}

impl<'reader> NetworkReader<'reader> {
    /// Cheap byte readers, which does not hold your hand. If you mess up, it will panic.
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::reader::NetworkReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    ///
    /// let slice = [0u8; 32];
    /// let mut readers = NetworkReader::new(&slice);
    /// let x = readers.read_f64_be();
    /// let y = readers.read_f64_be();
    /// ```
    pub const fn new(byte_array: &'reader [u8]) -> Self {
        Self { byte_array }
    }
}

/// Cheap little-endian byte reader, which does not hold your hand. If you mess up, it will panic.
///
/// # Examples
/// ```
/// use hyper_byte::reader::NetworkReader;
/// use hyper_byte::readers::traits::BigEndianByteReader;
///
/// let slice = [0u8; 32];
/// let mut readers = NetworkReader::new(&slice);
/// let x = readers.read_f64_be();
/// let y = readers.read_f64_be();
/// ```
pub struct LittleReader<'reader> {
    byte_array: &'reader [u8],
}

impl ByteReader for LittleReader<'_> {
    fn byte_array(&mut self) -> &[u8] {
        self.byte_array
    }

    unsafe fn advance(&mut self, advancement: usize, new_size: usize) {
        self.byte_array =
            &*slice_from_raw_parts(self.byte_array.as_ptr().add(advancement), new_size);
    }
}

impl LittleEndianByteReader for LittleReader<'_> {}

impl<'reader> LittleReader<'reader> {
    /// Cheap byte readers, which does not hold your hand. If you mess up, it will panic.
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::reader::LittleReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    ///
    /// let slice = [0u8; 32];
    /// let mut readers = LittleReader::new(&slice);
    /// let x = readers.read_f64_le();
    /// let y = readers.read_f64_le();
    /// ```
    pub const fn new(byte_array: &'reader [u8]) -> Self {
        Self { byte_array }
    }
}

/// Cheap native-endian byte reader, which does not hold your hand. If you mess up, it will panic.
///
/// # Examples
/// ```
/// use hyper_byte::reader::NativeReader;
/// use hyper_byte::readers::traits::NativeEndianByteReader;
///
/// let slice = [0u8; 32];
/// let mut readers = NativeReader::new(&slice);
/// let x = readers.read_f64_ne();
/// let y = readers.read_f64_ne();
/// ```
pub struct NativeReader<'reader> {
    byte_array: &'reader [u8],
}

impl ByteReader for NativeReader<'_> {
    fn byte_array(&mut self) -> &[u8] {
        self.byte_array
    }

    unsafe fn advance(&mut self, advancement: usize, new_size: usize) {
        self.byte_array =
            &*slice_from_raw_parts(self.byte_array.as_ptr().add(advancement), new_size);
    }
}

impl NativeEndianByteReader for NativeReader<'_> {}

impl<'reader> NativeReader<'reader> {
    /// Cheap byte readers, which does not hold your hand. If you mess up, it will panic.
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::reader::LittleReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    ///
    /// let slice = [0u8; 32];
    /// let mut readers = LittleReader::new(&slice);
    /// let x = readers.read_f64_le();
    /// let y = readers.read_f64_le();
    /// ```
    pub const fn new(byte_array: &'reader [u8]) -> Self {
        Self { byte_array }
    }
}
