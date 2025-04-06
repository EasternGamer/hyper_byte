#[cfg(feature = "half")]
use half::f16;

pub trait ByteReader {
    /// Returns a reference to the current underlying byte-slice.
    fn byte_array(&mut self) -> &[u8];

    /// Advances the byte reader by a certain amount, and uses the new size as its expected size
    /// # Safety
    /// To make it safe, ensure advancement and new size are with respect to the array used and are
    /// within proportion of each other
    /// # Arguments
    /// * `advancement`: the amount of bytes to "skip over"
    /// * `new_size`: the new size of the array after "skipping over"
    unsafe fn advance(&mut self, advancement: usize, new_size: usize);

    /// Skips a `[u8]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_u8(&mut self) {
        self.skip_n(size_of::<u8>());
    }

    /// Skips an `[i16]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_u16(&mut self) {
        self.skip_n(size_of::<u16>());
    }

    /// Skips an `[i32]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_u32(&mut self) {
        self.skip_n(size_of::<u32>());
    }

    /// Skips an `[u64]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_u64(&mut self) {
        self.skip_n(size_of::<u64>());
    }

    /// Skips an `[u128]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_u128(&mut self) {
        self.skip_n(size_of::<u128>());
    }

    /// Skips an `[usize]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_usize(&mut self) {
        self.skip_n(size_of::<usize>());
    }

    /// Skips an `[i8]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_i8(&mut self) {
        self.skip_n(size_of::<i8>());
    }

    /// Skips an `[i16]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_i16(&mut self) {
        self.skip_n(size_of::<i16>());
    }

    /// Skips an `[i32]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_i32(&mut self) {
        self.skip_n(size_of::<i32>());
    }

    /// Skips an `[i64]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_i64(&mut self) {
        self.skip_n(size_of::<i64>());
    }

    /// Skips an `[i128]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_i128(&mut self) {
        self.skip_n(size_of::<i128>());
    }

    /// Skips an `[isize]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_isize(&mut self) {
        self.skip_n(size_of::<isize>());
    }

    #[cfg(feature = "half")]
    /// Skips a `[f16]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_f16(&mut self) {
        self.skip_n(size_of::<f16>());
    }

    /// Skips a `[f32]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_f32(&mut self) {
        self.skip_n(size_of::<f32>());
    }

    /// Skips a `[f64]` number of bytes from the byte array, advancing the readers forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_f64(&mut self) {
        self.skip_n(size_of::<f64>());
    }

    /// Skips a custom number of bytes from the byte array, advancing the readers forward by the specified [`byte_size`] of bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn skip_n(&mut self, byte_size: usize) {
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                self.advance(byte_size, new_length as usize);
            }
        } else {
            panic!("Attempted to skip bytes of an array without space in the array.");
        }
    }
}

pub trait NativeEndianByteReader: ByteReader {
    /// Reads a native-endian [`u8`] from the byte array, advancing the readers forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u8_ne(&mut self) -> u8 {
        let byte_size_needed = size_of::<u8>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u8_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u8 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u16`] from the byte array, advancing the readers forward by 2 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u16_ne(&mut self) -> u16 {
        let byte_size_needed = size_of::<u16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u16_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u16 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u32_ne(&mut self) -> u32 {
        let byte_size_needed = size_of::<u32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u32_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u32 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u64_ne(&mut self) -> u64 {
        let byte_size_needed = size_of::<u64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u64_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u64 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u128`] from the byte array, advancing the readers forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u128_ne(&mut self) -> u128 {
        let byte_size_needed = size_of::<u128>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u128_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u128 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian unsigned native-endian usize from the byte array, advancing the readers forward by [`size_of::<usize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_usize_ne(&mut self) -> usize {
        let byte_size_needed = size_of::<usize>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_usize_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read usize of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i8`] from the byte array, advancing the readers forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i8_ne(&mut self) -> i8 {
        let byte_size_needed = size_of::<i8>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i8_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i8 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i16`] from the byte array, advancing the readers forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i16_ne(&mut self) -> i16 {
        let byte_size_needed = size_of::<i16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i16_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i16 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i32_ne(&mut self) -> i32 {
        let byte_size_needed = size_of::<i32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i32_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i32 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i64_ne(&mut self) -> i64 {
        let byte_size_needed = size_of::<i64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i64_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i64 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i128`] from the byte array, advancing the readers forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i128_ne(&mut self) -> i128 {
        let byte_size_needed = size_of::<i128>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i128_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i128 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`isize`] from the byte array, advancing the readers forward by [`size_of::<isize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_isize_ne(&mut self) -> isize {
        let byte_size_needed = size_of::<isize>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_isize_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read isize of native-endian of an array without enough space within the array.");
        }
    }

    #[cfg(feature = "half")]
    /// Reads a native-endian [`f16`] from the byte array, advancing the readers forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f16_ne(&mut self) -> f16 {
        let byte_size_needed = size_of::<f16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f16_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f16 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`f32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f32_ne(&mut self) -> f32 {
        let byte_size_needed = size_of::<f32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f32_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f32 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`f64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f64_ne(&mut self) -> f64 {
        let byte_size_needed = size_of::<f64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f64_ne(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f64 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a custom number of bytes from the byte array, advancing the readers forward by the specified [`byte_size`] of bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_n_ne(&mut self, byte_size: usize) -> Vec<u8> {
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let bytes: Vec<u8> = byte_array[..byte_size].to_vec();
                self.advance(byte_size, new_length as usize);
                bytes
            }
        } else {
            panic!("Attempted to read custom number bytes of an array without space in the array.");
        }
    }
}

pub trait LittleEndianByteReader: ByteReader {
    /// Reads a little-endian [`u8`] from the byte array, advancing the readers forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u8_le(&mut self) -> u8 {
        let byte_size_needed = size_of::<u8>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u8_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u8 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u16`] from the byte array, advancing the readers forward by 2 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u16_le(&mut self) -> u16 {
        let byte_size_needed = size_of::<u16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u16_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u16 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u32_le(&mut self) -> u32 {
        let byte_size_needed = size_of::<u32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u32_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u32 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u64_le(&mut self) -> u64 {
        let byte_size_needed = size_of::<u64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u64_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u64 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u128`] from the byte array, advancing the readers forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u128_le(&mut self) -> u128 {
        let byte_size_needed = size_of::<u128>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u128_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u128 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian unsigned little-endian usize from the byte array, advancing the readers forward by [`size_of::<usize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_usize_le(&mut self) -> usize {
        let byte_size_needed = size_of::<usize>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_usize_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read usize of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i8`] from the byte array, advancing the readers forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i8_le(&mut self) -> i8 {
        let byte_size_needed = size_of::<i8>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i8_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i8 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i16`] from the byte array, advancing the readers forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i16_le(&mut self) -> i16 {
        let byte_size_needed = size_of::<i16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i16_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i16 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i32_le(&mut self) -> i32 {
        let byte_size_needed = size_of::<i32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i32_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i32 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i64_le(&mut self) -> i64 {
        let byte_size_needed = size_of::<i64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i64_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i64 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i128`] from the byte array, advancing the readers forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i128_le(&mut self) -> i128 {
        let byte_size_needed = size_of::<i128>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i128_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i128 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`isize`] from the byte array, advancing the readers forward by [`size_of::<isize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_isize_le(&mut self) -> isize {
        let byte_size_needed = size_of::<isize>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_isize_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read isize of little-endian of an array without enough space within the array.");
        }
    }

    #[cfg(feature = "half")]
    /// Reads a little-endian [`f16`] from the byte array, advancing the readers forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f16_le(&mut self) -> f16 {
        let byte_size_needed = size_of::<f16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f16_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f16 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`f32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f32_le(&mut self) -> f32 {
        let byte_size_needed = size_of::<f32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f32_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f32 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`f64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f64_le(&mut self) -> f64 {
        let byte_size_needed = size_of::<f64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f64_le(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f64 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a custom number of bytes from the byte array, advancing the readers forward by the specified [`byte_size`] of bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_n_le(&mut self, byte_size: usize) -> Vec<u8> {
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let bytes: Vec<u8> = byte_array[..byte_size].iter().map(|x| x.to_le()).collect();
                self.advance(byte_size, new_length as usize);
                bytes
            }
        } else {
            panic!("Attempted to read custom number bytes of an array without space in the array.");
        }
    }
}

pub trait BigEndianByteReader: ByteReader {
    /// Reads a big-endian [`u8`] from the byte array, advancing the readers forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u8_be(&mut self) -> u8 {
        let byte_size_needed = size_of::<u8>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u8_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u8 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`u16`] from the byte array, advancing the readers forward by 2 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u16_be(&mut self) -> u16 {
        let byte_size_needed = size_of::<u16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u16_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u16 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`u32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u32_be(&mut self) -> u32 {
        let byte_size_needed = size_of::<u32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u32_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u32 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`u64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u64_be(&mut self) -> u64 {
        let byte_size_needed = size_of::<u64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u64_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u64 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`u128`] from the byte array, advancing the readers forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_u128_be(&mut self) -> u128 {
        let byte_size_needed = size_of::<u128>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u128_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u128 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian unsigned big-endian usize from the byte array, advancing the readers forward by [`size_of::<usize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_usize_be(&mut self) -> usize {
        let byte_size_needed = size_of::<usize>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_usize_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read usize of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i8`] from the byte array, advancing the readers forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i8_be(&mut self) -> i8 {
        let byte_size_needed = size_of::<i8>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i8_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i8 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i16`] from the byte array, advancing the readers forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i16_be(&mut self) -> i16 {
        let byte_size_needed = size_of::<i16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i16_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i16 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i32_be(&mut self) -> i32 {
        let byte_size_needed = size_of::<i32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i32_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i32 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i64_be(&mut self) -> i64 {
        let byte_size_needed = size_of::<i64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i64_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i64 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i128`] from the byte array, advancing the readers forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_i128_be(&mut self) -> i128 {
        let byte_size_needed = size_of::<i128>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i128_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i128 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`isize`] from the byte array, advancing the readers forward by [`size_of::<isize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_isize_be(&mut self) -> isize {
        let byte_size_needed = size_of::<isize>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_isize_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read isize of big-endian of an array without enough space within the array.");
        }
    }

    #[cfg(feature = "half")]
    /// Reads a big-endian [`f16`] from the byte array, advancing the readers forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f16_be(&mut self) -> f16 {
        let byte_size_needed = size_of::<f16>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f16_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f16 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`f32`] from the byte array, advancing the readers forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f32_be(&mut self) -> f32 {
        let byte_size_needed = size_of::<f32>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f32_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f32 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`f64`] from the byte array, advancing the readers forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_f64_be(&mut self) -> f64 {
        let byte_size_needed = size_of::<f64>();
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f64_be(byte_array);
                self.advance(byte_size_needed, new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f64 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a custom number of bytes from the byte array, advancing the readers forward by the specified [`byte_size`] of bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    fn read_n_be(&mut self, byte_size: usize) -> Vec<u8> {
        let byte_array = self.byte_array();
        let new_length = (byte_array.len() as isize) - byte_size as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let bytes: Vec<u8> = byte_array[..byte_size].iter().map(|x| x.to_be()).collect();
                self.advance(byte_size, new_length as usize);
                bytes
            }
        } else {
            panic!("Attempted to read custom number bytes of an array without space in the array.");
        }
    }
}
