#[cfg(feature = "half")]
use half::f16;

pub trait ByteWriter {
    /// Consumes the writer to return the bytes inside<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193u16, 22u16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u16_be(my_simple_tuple.0);
    /// writer.write_u16_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u16_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u16_be(), my_simple_tuple.1);
    /// ```
    fn to_vec(self) -> Vec<u8>;

    /// Consumes the writer to return the bytes inside<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193u16, 22u16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u16_be(my_simple_tuple.0);
    /// writer.write_u16_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.as_slice();
    /// let mut reader = FastByteReader::new(buffer);
    ///
    /// assert_eq!(reader.read_u16_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u16_be(), my_simple_tuple.1);
    /// ```
    fn as_slice(&self) -> &[u8];

    /// Consumes the writer to return the bytes inside<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193u16, 22u16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u16_be(my_simple_tuple.0);
    /// writer.write_u16_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.as_slice();
    /// let mut reader = FastByteReader::new(buffer);
    ///
    /// assert_eq!(reader.read_u16_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u16_be(), my_simple_tuple.1);
    /// ```
    fn as_mut_vec(&mut self) -> &mut Vec<u8>;
}

pub trait NativeEndianByteWriter: ByteWriter {
    /// For writing a byte in native-endian order<br/>
    /// It is not recommended to use this unless the value in particular is quite literally one byte
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u8, 22u8);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u8_ne(my_simple_tuple.0);
    /// writer.write_u8_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u8_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u8_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u8_ne(&mut self, value: u8) {
        self.as_mut_vec().push(value)
    }

    /// For writing a `u16` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u16, 22u16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u16_ne(my_simple_tuple.0);
    /// writer.write_u16_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u16_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u16_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u16_ne(&mut self, value: u16) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `u32` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u32, 22u32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u32_ne(my_simple_tuple.0);
    /// writer.write_u32_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u32_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u32_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u32_ne(&mut self, value: u32) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `u64` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u64, 22u64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u64_ne(my_simple_tuple.0);
    /// writer.write_u64_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u64_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u64_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u64_ne(&mut self, value: u64) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `u128` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u128, 22u128);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u128_ne(my_simple_tuple.0);
    /// writer.write_u128_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u128_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u128_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u128_ne(&mut self, value: u128) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `usize` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193usize, 22usize);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_usize_ne(my_simple_tuple.0);
    /// writer.write_usize_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_usize_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_usize_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_usize_ne(&mut self, value: usize) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing an `i8` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (19i8, 22i8);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i8_ne(my_simple_tuple.0);
    /// writer.write_i8_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i8_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i8_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i8_ne(&mut self, value: i8) {
        self.as_mut_vec().push(value as u8)
    }

    /// For writing a `i16` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193i16, 22i16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i16_ne(my_simple_tuple.0);
    /// writer.write_i16_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i16_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i16_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i16_ne(&mut self, value: i16) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `i32` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193i32, 22i32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i32_ne(my_simple_tuple.0);
    /// writer.write_i32_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i32_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i32_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i32_ne(&mut self, value: i32) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `i64` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193i64, 22i64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i64_ne(my_simple_tuple.0);
    /// writer.write_i64_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i64_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i64_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i64_ne(&mut self, value: i64) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `i128` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193i128, 22i128);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i128_ne(my_simple_tuple.0);
    /// writer.write_i128_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i128_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i128_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i128_ne(&mut self, value: i128) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `isize` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193isize, 22isize);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_isize_ne(my_simple_tuple.0);
    /// writer.write_isize_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_isize_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_isize_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_isize_ne(&mut self, value: isize) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    #[cfg(feature = "half")]
    /// For writing a `f16` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use half::f16;
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (f16::from_f32_const(133.0f32), f16::from_f32_const(13.0f32));
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f16_ne(my_simple_tuple.0);
    /// writer.write_f16_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f16_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f16_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f16_ne(&mut self, value: f16) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `f32` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193.32f32, 22.13f32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f32_ne(my_simple_tuple.0);
    /// writer.write_f32_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f32_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f32_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f32_ne(&mut self, value: f32) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `f64` in native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193.6f64, 22.3f64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f64_ne(my_simple_tuple.0);
    /// writer.write_f64_ne(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f64_ne(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f64_ne(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f64_ne(&mut self, value: f64) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a set of native-endian bytes into native-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::NativeEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, NativeEndianByteWriter};
    ///
    /// let my_simple_tuple = (193.6f64.to_ne_bytes(), 22.3f64.to_ne_bytes());
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_bytes_ne(&my_simple_tuple.0);
    /// writer.write_bytes_ne(&my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_n_ne(my_simple_tuple.0.len()), my_simple_tuple.0);
    /// assert_eq!(reader.read_n_ne(my_simple_tuple.0.len()), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_bytes_ne(&mut self, value: &[u8]) {
        self.as_mut_vec().extend(value)
    }
}

pub trait LittleEndianByteWriter: ByteWriter {
    /// For writing a byte in little-endian order<br/>
    /// It is not recommended to use this unless the value in particular is quite literally one byte
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u8, 22u8);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u8_le(my_simple_tuple.0);
    /// writer.write_u8_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u8_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u8_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u8_le(&mut self, value: u8) {
        self.as_mut_vec().push(value.to_le())
    }

    /// For writing a `u16` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u16, 22u16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u16_le(my_simple_tuple.0);
    /// writer.write_u16_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u16_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u16_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u16_le(&mut self, value: u16) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `u32` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u32, 22u32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u32_le(my_simple_tuple.0);
    /// writer.write_u32_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u32_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u32_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u32_le(&mut self, value: u32) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `u64` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u64, 22u64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u64_le(my_simple_tuple.0);
    /// writer.write_u64_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u64_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u64_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u64_le(&mut self, value: u64) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `u128` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193u128, 22u128);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u128_le(my_simple_tuple.0);
    /// writer.write_u128_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u128_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u128_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u128_le(&mut self, value: u128) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `usize` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193usize, 22usize);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_usize_le(my_simple_tuple.0);
    /// writer.write_usize_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_usize_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_usize_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_usize_le(&mut self, value: usize) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing an `i8` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (19i8, 22i8);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i8_le(my_simple_tuple.0);
    /// writer.write_i8_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i8_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i8_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i8_le(&mut self, value: i8) {
        self.as_mut_vec().push((value as u8).to_le())
    }

    /// For writing a `i16` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193i16, 22i16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i16_le(my_simple_tuple.0);
    /// writer.write_i16_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i16_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i16_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i16_le(&mut self, value: i16) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `i32` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193i32, 22i32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i32_le(my_simple_tuple.0);
    /// writer.write_i32_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i32_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i32_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i32_le(&mut self, value: i32) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `i64` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193i64, 22i64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i64_le(my_simple_tuple.0);
    /// writer.write_i64_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i64_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i64_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i64_le(&mut self, value: i64) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `i128` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193i128, 22i128);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i128_le(my_simple_tuple.0);
    /// writer.write_i128_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i128_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i128_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i128_le(&mut self, value: i128) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `isize` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193isize, 22isize);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_isize_le(my_simple_tuple.0);
    /// writer.write_isize_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_isize_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_isize_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_isize_le(&mut self, value: isize) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    #[cfg(feature = "half")]
    /// For writing a `f16` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use half::f16;
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (f16::from_f32_const(133.0f32), f16::from_f32_const(13.0f32));
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f16_le(my_simple_tuple.0);
    /// writer.write_f16_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f16_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f16_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f16_le(&mut self, value: f16) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `f32` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193.32f32, 22.13f32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f32_le(my_simple_tuple.0);
    /// writer.write_f32_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f32_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f32_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f32_le(&mut self, value: f32) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a `f64` in little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193.6f64, 22.3f64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f64_le(my_simple_tuple.0);
    /// writer.write_f64_le(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f64_le(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f64_le(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f64_le(&mut self, value: f64) {
        self.as_mut_vec().extend(value.to_le_bytes())
    }

    /// For writing a set of native-endian bytes into little-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::LittleEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{ByteWriter, LittleEndianByteWriter};
    ///
    /// let my_simple_tuple = (193.6f64.to_ne_bytes(), 22.3f64.to_ne_bytes());
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_bytes_le(&my_simple_tuple.0);
    /// writer.write_bytes_le(&my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_n_le(my_simple_tuple.0.len()), my_simple_tuple.0);
    /// assert_eq!(reader.read_n_le(my_simple_tuple.0.len()), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_bytes_le(&mut self, value: &[u8]) {
        self.as_mut_vec().extend(value.iter().map(|x| x.to_le()))
    }
}

pub trait BigEndianByteWriter: ByteWriter {
    /// For writing a byte in big-endian order<br/>
    /// It is not recommended to use this unless the value in particular is quite literally one byte
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193u8, 22u8);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u8_be(my_simple_tuple.0);
    /// writer.write_u8_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u8_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u8_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u8_be(&mut self, value: u8) {
        self.as_mut_vec().push(value.to_be())
    }

    /// For writing a `u16` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193u16, 22u16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u16_be(my_simple_tuple.0);
    /// writer.write_u16_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u16_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u16_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u16_be(&mut self, value: u16) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `u32` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193u32, 22u32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u32_be(my_simple_tuple.0);
    /// writer.write_u32_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u32_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u32_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u32_be(&mut self, value: u32) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `u64` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193u64, 22u64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u64_be(my_simple_tuple.0);
    /// writer.write_u64_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u64_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u64_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u64_be(&mut self, value: u64) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `u128` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193u128, 22u128);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_u128_be(my_simple_tuple.0);
    /// writer.write_u128_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_u128_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_u128_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_u128_be(&mut self, value: u128) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `usize` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193usize, 22usize);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_usize_be(my_simple_tuple.0);
    /// writer.write_usize_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_usize_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_usize_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_usize_be(&mut self, value: usize) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing an `i8` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (19i8, 22i8);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i8_be(my_simple_tuple.0);
    /// writer.write_i8_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i8_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i8_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i8_be(&mut self, value: i8) {
        self.as_mut_vec().push((value as u8).to_be())
    }

    /// For writing a `i16` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193i16, 22i16);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i16_be(my_simple_tuple.0);
    /// writer.write_i16_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i16_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i16_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i16_be(&mut self, value: i16) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `i32` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193i32, 22i32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i32_be(my_simple_tuple.0);
    /// writer.write_i32_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i32_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i32_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i32_be(&mut self, value: i32) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `i64` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193i64, 22i64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i64_be(my_simple_tuple.0);
    /// writer.write_i64_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i64_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i64_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i64_be(&mut self, value: i64) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `i128` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193i128, 22i128);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_i128_be(my_simple_tuple.0);
    /// writer.write_i128_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_i128_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_i128_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_i128_be(&mut self, value: i128) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `isize` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193isize, 22isize);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_isize_be(my_simple_tuple.0);
    /// writer.write_isize_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_isize_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_isize_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_isize_be(&mut self, value: isize) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    #[cfg(feature = "half")]
    /// For writing a `f16` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use half::f16;
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (f16::from_f32_const(133.0f32), f16::from_f32_const(13.0f32));
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f16_be(my_simple_tuple.0);
    /// writer.write_f16_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f16_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f16_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f16_be(&mut self, value: f16) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `f32` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193.32f32, 22.13f32);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f32_be(my_simple_tuple.0);
    /// writer.write_f32_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f32_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f32_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f32_be(&mut self, value: f32) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a `f64` in big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193.6f64, 22.3f64);
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_f64_be(my_simple_tuple.0);
    /// writer.write_f64_be(my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_f64_be(), my_simple_tuple.0);
    /// assert_eq!(reader.read_f64_be(), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_f64_be(&mut self, value: f64) {
        self.as_mut_vec().extend(value.to_be_bytes())
    }

    /// For writing a set of native-endian bytes into big-endian order<br/>
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    /// use hyper_byte::readers::traits::BigEndianByteReader;
    /// use hyper_byte::writer::FastByteWriter;
    /// use hyper_byte::writers::traits::{BigEndianByteWriter, ByteWriter};
    ///
    /// let my_simple_tuple = (193.6f64.to_ne_bytes(), 22.3f64.to_ne_bytes());
    ///
    /// let mut writer = FastByteWriter::new();
    /// writer.write_bytes_be(&my_simple_tuple.0);
    /// writer.write_bytes_be(&my_simple_tuple.1);
    ///
    /// let buffer = writer.to_vec();
    /// let mut reader = FastByteReader::new(&buffer);
    ///
    /// assert_eq!(reader.read_n_be(my_simple_tuple.0.len()), my_simple_tuple.0);
    /// assert_eq!(reader.read_n_be(my_simple_tuple.0.len()), my_simple_tuple.1);
    /// ```
    #[inline(always)]
    fn write_bytes_be(&mut self, value: &[u8]) {
        self.as_mut_vec().extend(value.iter().map(|x| x.to_be()))
    }
}
