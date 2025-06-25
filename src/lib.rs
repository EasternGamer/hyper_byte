pub mod reader;
pub mod readers;
pub mod writer;
pub mod writers;

#[cfg(feature = "half")]
use half::f16;

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 8-bit integer using big-endianness.
/// # Safety
/// To make it safe and does not cause memory errors, you must ensure the input has at least 1 byte prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u8`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u8_be;
///
/// let mut slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u8_be(&slice[0..1]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u8_be(&slice[0..0]) };
///
///
/// pub fn read_u8_safe_be(array: &[u8], index: &mut usize) -> u16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u8_be(&array[*index..(*index+1)]) };
///     *index += 1;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u8_be(bytes: &[u8]) -> u8 {
    unsafe { u8::from_be_bytes(*(bytes.as_ptr() as *const [u8; 1])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 16-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u16_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u16_be(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u16_be(&slice[9..9]) };
///
///
/// pub fn read_u16_safe_be(array: &[u8], index: &mut usize) -> u16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u16_be(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u16_be(bytes: &[u8]) -> u16 {
    unsafe { u16::from_be_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 32-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u32_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u32_be(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u32_be(&slice[9..9]) };
///
///
/// pub fn read_u32_safe_be(array: &[u8], index: &mut usize) -> u32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u32_be(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u32_be(bytes: &[u8]) -> u32 {
    unsafe { u32::from_be_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 64-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u64_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u64_be(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u64_be(&slice[9..9]) };
///
///
/// pub fn read_u64_safe_be(array: &[u8], index: &mut usize) -> u64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u64_be(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u64_be(bytes: &[u8]) -> u64 {
    unsafe { u64::from_be_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 128-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 16 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u128`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u128_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u128_be(&slice[0..16]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u128_be(&slice[9..9]) };
///
///
/// pub fn read_u128_safe_be(array: &[u8], index: &mut usize) -> u128 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u128_be(&array[*index..(*index+16)]) };
///     *index += 16;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u128_be(bytes: &[u8]) -> u128 {
    unsafe { u128::from_be_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an usize integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least usize'd bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`usize`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_usize_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_usize_be(&slice[0..size_of::<usize>()]) };
/// // NOT SAFE
/// let second_value = unsafe { read_usize_be(&slice[9..9]) };
///
///
/// pub fn read_usize_safe_be(array: &[u8], index: &mut usize) -> usize {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_usize_be(&array[*index..(*index+size_of::<usize>())]) };
///     *index += size_of::<usize>();
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_usize_be(bytes: &[u8]) -> usize {
    unsafe { usize::from_be_bytes(*(bytes.as_ptr() as *const [u8; size_of::<usize>()])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 8-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 1 byte prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i8`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i8_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i8_be(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i8_be(&slice[9..9]) };
///
///
/// pub fn read_i8_safe_be(array: &[u8], index: &mut usize) -> i8 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i8_be(&array[*index..(*index+1)]) };
///     *index += 1;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i8_be(bytes: &[u8]) -> i8 {
    unsafe { i8::from_be_bytes(*(bytes.as_ptr() as *const [u8; 1])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 16-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i16_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i16_be(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i16_be(&slice[9..9]) };
///
///
/// pub fn read_i16_safe_be(array: &[u8], index: &mut usize) -> i16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i16_be(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i16_be(bytes: &[u8]) -> i16 {
    unsafe { i16::from_be_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 32-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i32_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i32_be(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i32_be(&slice[9..9]) };
///
///
/// pub fn read_i32_safe_be(array: &[u8], index: &mut usize) -> i32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i32_be(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i32_be(bytes: &[u8]) -> i32 {
    unsafe { i32::from_be_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 64-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i64_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i64_be(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i64_be(&slice[9..9]) };
///
///
/// pub fn read_i64_safe_be(array: &[u8], index: &mut usize) -> i64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i64_be(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i64_be(bytes: &[u8]) -> i64 {
    unsafe { i64::from_be_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 128-bit integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 16 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i128`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i128_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i128_be(&slice[0..16]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i128_be(&slice[9..9]) };
///
///
/// pub fn read_i128_safe_be(array: &[u8], index: &mut usize) -> i128 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i128_be(&array[*index..(*index+16)]) };
///     *index += 16;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i128_be(bytes: &[u8]) -> i128 {
    unsafe { i128::from_be_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an isize integer using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least isize'd bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`isize`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_isize_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_isize_be(&slice[0..size_of::<isize>()]) };
/// // NOT SAFE
/// let second_value = unsafe { read_isize_be(&slice[9..9]) };
///
///
/// pub fn read_isize_safe_be(array: &[u8], index: &mut usize) -> isize {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_isize_be(&array[*index..(*index+size_of::<isize>())]) };
///     *index += size_of::<isize>();
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_isize_be(bytes: &[u8]) -> isize {
    unsafe { isize::from_be_bytes(*(bytes.as_ptr() as *const [u8; size_of::<isize>()])) }
}

#[cfg(feature = "half")]
/// Unsafe, near zero cost transmutation of a byte array slice into a 16-bit floating point using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use half::f16;
/// use hyper_byte::read_f16_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f16_be(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f16_be(&slice[9..9]) };
///
///
/// pub fn read_f16_safe_be(array: &[u8], index: &mut usize) -> f16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f16_be(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f16_be(bytes: &[u8]) -> f16 {
    unsafe { f16::from_be_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a 32-bit floating point using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_f32_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f32_be(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f32_be(&slice[9..9]) };
///
///
/// pub fn read_f32_safe_be(array: &[u8], index: &mut usize) -> f32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f32_be(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f32_be(bytes: &[u8]) -> f32 {
    unsafe { f32::from_be_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a 64-bit floating point using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_f64_be;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f64_be(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f64_be(&slice[9..9]) };
///
///
/// pub fn read_f64_safe_be(array: &[u8], index: &mut usize) -> f64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f64_be(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f64_be(bytes: &[u8]) -> f64 {
    unsafe { f64::from_be_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 8-bit integer using little-endianness.
/// # Safety
/// To make it safe and does not cause memory errors, you must ensure the input has at least 1 byte prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u8`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u8_le;
///
/// let mut slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u8_le(&slice[0..1]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u8_le(&slice[0..0]) };
///
///
/// pub fn read_u8_safe_le(array: &[u8], index: &mut usize) -> u16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u8_le(&array[*index..(*index+1)]) };
///     *index += 1;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u8_le(bytes: &[u8]) -> u8 {
    unsafe { u8::from_le_bytes(*(bytes.as_ptr() as *const [u8; 1])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 16-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u16_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u16_le(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u16_le(&slice[9..9]) };
///
///
/// pub fn read_u16_safe_le(array: &[u8], index: &mut usize) -> u16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u16_le(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u16_le(bytes: &[u8]) -> u16 {
    unsafe { u16::from_le_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 32-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u32_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u32_le(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u32_le(&slice[9..9]) };
///
///
/// pub fn read_u32_safe_le(array: &[u8], index: &mut usize) -> u32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u32_le(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u32_le(bytes: &[u8]) -> u32 {
    unsafe { u32::from_le_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 64-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u64_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u64_le(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u64_le(&slice[9..9]) };
///
///
/// pub fn read_u64_safe_le(array: &[u8], index: &mut usize) -> u64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u64_le(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u64_le(bytes: &[u8]) -> u64 {
    unsafe { u64::from_le_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 128-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 16 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u128`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u128_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u128_le(&slice[0..16]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u128_le(&slice[9..9]) };
///
///
/// pub fn read_u128_safe_le(array: &[u8], index: &mut usize) -> u128 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u128_le(&array[*index..(*index+16)]) };
///     *index += 16;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u128_le(bytes: &[u8]) -> u128 {
    unsafe { u128::from_le_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an usize integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least usize'd bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`usize`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_usize_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_usize_le(&slice[0..size_of::<usize>()]) };
/// // NOT SAFE
/// let second_value = unsafe { read_usize_le(&slice[9..9]) };
///
///
/// pub fn read_usize_safe_le(array: &[u8], index: &mut usize) -> usize {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_usize_le(&array[*index..(*index+size_of::<usize>())]) };
///     *index += size_of::<usize>();
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_usize_le(bytes: &[u8]) -> usize {
    unsafe { usize::from_le_bytes(*(bytes.as_ptr() as *const [u8; size_of::<usize>()])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 8-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 1 byte prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i8`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i8_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i8_le(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i8_le(&slice[9..9]) };
///
///
/// pub fn read_i8_safe_le(array: &[u8], index: &mut usize) -> i8 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i8_le(&array[*index..(*index+1)]) };
///     *index += 1;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i8_le(bytes: &[u8]) -> i8 {
    unsafe { i8::from_le_bytes(*(bytes.as_ptr() as *const [u8; 1])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 16-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i16_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i16_le(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i16_le(&slice[9..9]) };
///
///
/// pub fn read_i16_safe_le(array: &[u8], index: &mut usize) -> i16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i16_le(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i16_le(bytes: &[u8]) -> i16 {
    unsafe { i16::from_le_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 32-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i32_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i32_le(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i32_le(&slice[9..9]) };
///
///
/// pub fn read_i32_safe_le(array: &[u8], index: &mut usize) -> i32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i32_le(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i32_le(bytes: &[u8]) -> i32 {
    unsafe { i32::from_le_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 64-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i64_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i64_le(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i64_le(&slice[9..9]) };
///
///
/// pub fn read_i64_safe_le(array: &[u8], index: &mut usize) -> i64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i64_le(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i64_le(bytes: &[u8]) -> i64 {
    unsafe { i64::from_le_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 128-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 16 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i128`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i128_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i128_le(&slice[0..16]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i128_le(&slice[9..9]) };
///
///
/// pub fn read_i128_safe_le(array: &[u8], index: &mut usize) -> i128 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i128_le(&array[*index..(*index+16)]) };
///     *index += 16;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i128_le(bytes: &[u8]) -> i128 {
    unsafe { i128::from_le_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an isize integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least isize'd bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`isize`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_isize_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_isize_le(&slice[0..size_of::<isize>()]) };
/// // NOT SAFE
/// let second_value = unsafe { read_isize_le(&slice[9..9]) };
///
///
/// pub fn read_isize_safe_be(array: &[u8], index: &mut usize) -> isize {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_isize_le(&array[*index..(*index+size_of::<isize>())]) };
///     *index += size_of::<isize>();
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_isize_le(bytes: &[u8]) -> isize {
    unsafe { isize::from_le_bytes(*(bytes.as_ptr() as *const [u8; size_of::<isize>()])) }
}

#[cfg(feature = "half")]
/// Unsafe, near zero cost transmutation of a byte array slice into a 16-bit floating point using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use half::f16;
/// use hyper_byte::read_f16_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f16_le(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f16_le(&slice[9..9]) };
///
///
/// pub fn read_f16_safe_le(array: &[u8], index: &mut usize) -> f16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f16_le(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f16_le(bytes: &[u8]) -> f16 {
    unsafe { f16::from_le_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a 32-bit floating point using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_f32_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f32_le(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f32_le(&slice[9..9]) };
///
///
/// pub fn read_f32_safe_le(array: &[u8], index: &mut usize) -> f32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f32_le(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f32_le(bytes: &[u8]) -> f32 {
    unsafe { f32::from_le_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a 64-bit floating point using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_f64_le;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f64_le(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f64_le(&slice[9..9]) };
///
///
/// pub fn read_f64_safe_le(array: &[u8], index: &mut usize) -> f64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f64_le(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f64_le(bytes: &[u8]) -> f64 {
    unsafe { f64::from_le_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 8-bit integer using little-endianness.
/// # Safety
/// To make it safe and does not cause memory errors, you must ensure the input has at least 1 byte prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u8`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u8_ne;
///
/// let mut slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u8_ne(&slice[0..1]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u8_ne(&slice[0..0]) };
///
///
/// pub fn read_u8_safe_ne(array: &[u8], index: &mut usize) -> u16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u8_ne(&array[*index..(*index+1)]) };
///     *index += 1;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u8_ne(bytes: &[u8]) -> u8 {
    unsafe { u8::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 1])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 16-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u16_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u16_ne(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u16_ne(&slice[9..9]) };
///
///
/// pub fn read_u16_safe_ne(array: &[u8], index: &mut usize) -> u16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u16_ne(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u16_ne(bytes: &[u8]) -> u16 {
    unsafe { u16::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 32-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u32_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u32_ne(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u32_ne(&slice[9..9]) };
///
///
/// pub fn read_u32_safe_ne(array: &[u8], index: &mut usize) -> u32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u32_ne(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u32_ne(bytes: &[u8]) -> u32 {
    unsafe { u32::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 64-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u64_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u64_ne(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u64_ne(&slice[9..9]) };
///
///
/// pub fn read_u64_safe_ne(array: &[u8], index: &mut usize) -> u64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u64_ne(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u64_ne(bytes: &[u8]) -> u64 {
    unsafe { u64::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 128-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 16 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`u128`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_u128_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_u128_ne(&slice[0..16]) };
/// // NOT SAFE
/// let second_value = unsafe { read_u128_ne(&slice[9..9]) };
///
///
/// pub fn read_u128_safe_ne(array: &[u8], index: &mut usize) -> u128 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_u128_ne(&array[*index..(*index+16)]) };
///     *index += 16;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_u128_ne(bytes: &[u8]) -> u128 {
    unsafe { u128::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an usize integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least usize'd bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`usize`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_usize_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_usize_ne(&slice[0..size_of::<usize>()]) };
/// // NOT SAFE
/// let second_value = unsafe { read_usize_ne(&slice[9..9]) };
///
///
/// pub fn read_usize_safe_ne(array: &[u8], index: &mut usize) -> usize {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_usize_ne(&array[*index..(*index+size_of::<usize>())]) };
///     *index += size_of::<usize>();
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_usize_ne(bytes: &[u8]) -> usize {
    unsafe { usize::from_ne_bytes(*(bytes.as_ptr() as *const [u8; size_of::<usize>()])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 8-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 1 byte prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i8`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i8_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i8_ne(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i8_ne(&slice[9..9]) };
///
///
/// pub fn read_i8_safe_ne(array: &[u8], index: &mut usize) -> i8 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i8_ne(&array[*index..(*index+1)]) };
///     *index += 1;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i8_ne(bytes: &[u8]) -> i8 {
    unsafe { i8::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 1])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 16-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i16_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i16_ne(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i16_ne(&slice[9..9]) };
///
///
/// pub fn read_i16_safe_ne(array: &[u8], index: &mut usize) -> i16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i16_ne(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i16_ne(bytes: &[u8]) -> i16 {
    unsafe { i16::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 32-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i32_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i32_ne(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i32_ne(&slice[9..9]) };
///
///
/// pub fn read_i32_safe_ne(array: &[u8], index: &mut usize) -> i32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i32_ne(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i32_ne(bytes: &[u8]) -> i32 {
    unsafe { i32::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 64-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i64_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i64_ne(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i64_ne(&slice[9..9]) };
///
///
/// pub fn read_i64_safe_ne(array: &[u8], index: &mut usize) -> i64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i64_ne(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i64_ne(bytes: &[u8]) -> i64 {
    unsafe { i64::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a signed 128-bit integer using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 16 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`i128`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_i128_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_i128_ne(&slice[0..16]) };
/// // NOT SAFE
/// let second_value = unsafe { read_i128_ne(&slice[9..9]) };
///
///
/// pub fn read_i128_safe_ne(array: &[u8], index: &mut usize) -> i128 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_i128_ne(&array[*index..(*index+16)]) };
///     *index += 16;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_i128_ne(bytes: &[u8]) -> i128 {
    unsafe { i128::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into an isize integer using native-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least isize'd bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`isize`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_isize_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_isize_ne(&slice[0..size_of::<isize>()]) };
/// // NOT SAFE
/// let second_value = unsafe { read_isize_ne(&slice[9..9]) };
///
///
/// pub fn read_isize_safe_be(array: &[u8], index: &mut usize) -> isize {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_isize_ne(&array[*index..(*index+size_of::<isize>())]) };
///     *index += size_of::<isize>();
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_isize_ne(bytes: &[u8]) -> isize {
    unsafe { isize::from_ne_bytes(*(bytes.as_ptr() as *const [u8; size_of::<isize>()])) }
}

#[cfg(feature = "half")]
/// Unsafe, near zero cost transmutation of a byte array slice into a 16-bit floating point using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f16`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use half::f16;
/// use hyper_byte::read_f16_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f16_ne(&slice[0..2]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f16_ne(&slice[9..9]) };
///
///
/// pub fn read_f16_safe_ne(array: &[u8], index: &mut usize) -> f16 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f16_ne(&array[*index..(*index+2)]) };
///     *index += 2;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f16_ne(bytes: &[u8]) -> f16 {
    unsafe { f16::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 2])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a 32-bit floating point using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 4 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f32`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_f32_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f32_ne(&slice[0..4]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f32_ne(&slice[9..9]) };
///
///
/// pub fn read_f32_safe_ne(array: &[u8], index: &mut usize) -> f32 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f32_ne(&array[*index..(*index+4)]) };
///     *index += 4;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f32_ne(bytes: &[u8]) -> f32 {
    unsafe { f32::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 4])) }
}

/// Unsafe, near zero cost transmutation of a byte array slice into a 64-bit floating point using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 8 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: [`f64`]
///
/// # Examples
/// ```
/// # #[cfg(miri)] fn main() {}
/// # #[cfg(not(miri))]
/// # fn main() {
/// use hyper_byte::read_f64_ne;
///
/// let slice = [0u8; 16];
/// // PERFECTLY SAFE
/// let first_value = unsafe { read_f64_ne(&slice[0..8]) };
/// // NOT SAFE
/// let second_value = unsafe { read_f64_ne(&slice[9..9]) };
///
///
/// pub fn read_f64_safe_ne(array: &[u8], index: &mut usize) -> f64 {
///     // "SAFE" because even if the array is not of this length,
///     // it will still at least panic and not cause undefined behaviour
///     let third_value = unsafe { read_f64_ne(&array[*index..(*index+8)]) };
///     *index += 8;
///     third_value
/// }
/// # }
/// ```
#[inline(always)]
pub unsafe fn read_f64_ne(bytes: &[u8]) -> f64 {
    unsafe { f64::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::FastByteReader;
    use crate::readers::traits::*;
    use crate::writer::FastByteWriter;
    use crate::writers::traits::*;
    use half::f16;
    use std::hint;
    use std::time::Instant;

    #[test]
    fn test_signed_integers_be() {
        let int8 = 12i8;
        let int16 = 932i16;
        let int32 = 192i32;
        let int64 = 192932i64;
        let int128 = 19249462i128;
        let int_size = 1932387283747isize;

        let int8_array = int8.to_be_bytes();
        let int8_result = unsafe { read_i8_be(&int8_array) };
        assert_eq!(
            int8, int8_result,
            "Converting from i8 (big-endian) byte array results in the same i8 value"
        );

        let int16_array = int16.to_be_bytes();
        let int16_result = unsafe { read_i16_be(&int16_array) };
        assert_eq!(
            int16, int16_result,
            "Converting from i16 (big-endian) byte array results in the same i16 value"
        );

        let int32_array = int32.to_be_bytes();
        let int32_result = unsafe { read_i32_be(&int32_array) };
        assert_eq!(
            int32, int32_result,
            "Converting from i32 (big-endian) byte array results in the same i32 value"
        );

        let int64_array = int64.to_be_bytes();
        let int64_result = unsafe { read_i64_be(&int64_array) };
        assert_eq!(
            int64, int64_result,
            "Converting from i64 (big-endian) byte array results in the same i64 value"
        );

        let int128_array = int128.to_be_bytes();
        let int128_result = unsafe { read_i128_be(&int128_array) };
        assert_eq!(
            int128, int128_result,
            "Converting from i128 (big-endian) byte array results in the same i128 value"
        );

        let int_size_array = int_size.to_be_bytes();
        let int_size_result = unsafe { read_isize_be(&int_size_array) };
        assert_eq!(
            int_size, int_size_result,
            "Converting from isize (big-endian) byte array results in the same isize value"
        );
    }

    #[test]
    fn test_signed_integers_le() {
        let int8 = 12i8;
        let int16 = 932i16;
        let int32 = 192i32;
        let int64 = 192932i64;
        let int128 = 19249462i128;
        let int_size = 1932387283747isize;

        let int8_array = int8.to_le_bytes();
        let int8_result = unsafe { read_i8_le(&int8_array) };
        assert_eq!(
            int8, int8_result,
            "Converting from i8 (little-endian) byte array results in the same i8 value"
        );

        let int16_array = int16.to_le_bytes();
        let int16_result = unsafe { read_i16_le(&int16_array) };
        assert_eq!(
            int16, int16_result,
            "Converting from i16 (little-endian) byte array results in the same i16 value"
        );

        let int32_array = int32.to_le_bytes();
        let int32_result = unsafe { read_i32_le(&int32_array) };
        assert_eq!(
            int32, int32_result,
            "Converting from i32 (little-endian) byte array results in the same i32 value"
        );

        let int64_array = int64.to_le_bytes();
        let int64_result = unsafe { read_i64_le(&int64_array) };
        assert_eq!(
            int64, int64_result,
            "Converting from i64 (little-endian) byte array results in the same i64 value"
        );

        let int128_array = int128.to_le_bytes();
        let int128_result = unsafe { read_i128_le(&int128_array) };
        assert_eq!(
            int128, int128_result,
            "Converting from i128 (little-endian) byte array results in the same i128 value"
        );

        let int_size_array = int_size.to_le_bytes();
        let int_size_result = unsafe { read_isize_le(&int_size_array) };
        assert_eq!(
            int_size, int_size_result,
            "Converting from isize (little-endian) byte array results in the same isize value"
        );
    }

    #[test]
    fn test_signed_integers_ne() {
        let int8 = 12i8;
        let int16 = 932i16;
        let int32 = 192i32;
        let int64 = 192932i64;
        let int128 = 19249462i128;
        let int_size = 1932387283747isize;

        let int8_array = int8.to_ne_bytes();
        let int8_result = unsafe { read_i8_ne(&int8_array) };
        assert_eq!(
            int8, int8_result,
            "Converting from i8 (native-endian) byte array results in the same i8 value"
        );

        let int16_array = int16.to_ne_bytes();
        let int16_result = unsafe { read_i16_ne(&int16_array) };
        assert_eq!(
            int16, int16_result,
            "Converting from i16 (native-endian) byte array results in the same i16 value"
        );

        let int32_array = int32.to_ne_bytes();
        let int32_result = unsafe { read_i32_ne(&int32_array) };
        assert_eq!(
            int32, int32_result,
            "Converting from i32 (native-endian) byte array results in the same i32 value"
        );

        let int64_array = int64.to_ne_bytes();
        let int64_result = unsafe { read_i64_ne(&int64_array) };
        assert_eq!(
            int64, int64_result,
            "Converting from i64 (native-endian) byte array results in the same i64 value"
        );

        let int128_array = int128.to_ne_bytes();
        let int128_result = unsafe { read_i128_ne(&int128_array) };
        assert_eq!(
            int128, int128_result,
            "Converting from i128 (native-endian) byte array results in the same i128 value"
        );

        let int_size_array = int_size.to_ne_bytes();
        let int_size_result = unsafe { read_isize_ne(&int_size_array) };
        assert_eq!(
            int_size, int_size_result,
            "Converting from isize (native-endian) byte array results in the same isize value"
        );
    }

    #[test]
    fn test_unsigned_integers_be() {
        let int8 = 12u8;
        let int16 = 932u16;
        let int32 = 192u32;
        let int64 = 192932u64;
        let int128 = 19249462u128;
        let int_size = 1932387283747usize;

        let int8_array = int8.to_be_bytes();
        let int8_result = unsafe { read_u8_be(&int8_array) };
        assert_eq!(
            int8, int8_result,
            "Converting from u8 (big-endian) byte array results in the same u8 value"
        );

        let int16_array = int16.to_be_bytes();
        let int16_result = unsafe { read_u16_be(&int16_array) };
        assert_eq!(
            int16, int16_result,
            "Converting from u16 (big-endian) byte array results in the same u16 value"
        );

        let int32_array = int32.to_be_bytes();
        let int32_result = unsafe { read_u32_be(&int32_array) };
        assert_eq!(
            int32, int32_result,
            "Converting from u32 (big-endian) byte array results in the same u32 value"
        );

        let int64_array = int64.to_be_bytes();
        let int64_result = unsafe { read_u64_be(&int64_array) };
        assert_eq!(
            int64, int64_result,
            "Converting from u64 (big-endian) byte array results in the same u64 value"
        );

        let int128_array = int128.to_be_bytes();
        let int128_result = unsafe { read_u128_be(&int128_array) };
        assert_eq!(
            int128, int128_result,
            "Converting from u128 (big-endian) byte array results in the same u128 value"
        );

        let int_size_array = int_size.to_be_bytes();
        let int_size_result = unsafe { read_usize_be(&int_size_array) };
        assert_eq!(
            int_size, int_size_result,
            "Converting from usize (big-endian) byte array results in the same usize value"
        );
    }

    #[test]
    fn test_unsigned_integers_le() {
        let int8 = 12u8;
        let int16 = 932u16;
        let int32 = 192u32;
        let int64 = 192932u64;
        let int128 = 19249462u128;
        let int_size = 1932387283747usize;

        let int8_array = int8.to_le_bytes();
        let int8_result = unsafe { read_u8_le(&int8_array) };
        assert_eq!(
            int8, int8_result,
            "Converting from u8 (little-endian) byte array results in the same u8 value"
        );

        let int16_array = int16.to_le_bytes();
        let int16_result = unsafe { read_u16_le(&int16_array) };
        assert_eq!(
            int16, int16_result,
            "Converting from u16 (little-endian) byte array results in the same u16 value"
        );

        let int32_array = int32.to_le_bytes();
        let int32_result = unsafe { read_u32_le(&int32_array) };
        assert_eq!(
            int32, int32_result,
            "Converting from u32 (little-endian) byte array results in the same u32 value"
        );

        let int64_array = int64.to_le_bytes();
        let int64_result = unsafe { read_u64_le(&int64_array) };
        assert_eq!(
            int64, int64_result,
            "Converting from u64 (little-endian) byte array results in the same u64 value"
        );

        let int128_array = int128.to_le_bytes();
        let int128_result = unsafe { read_u128_le(&int128_array) };
        assert_eq!(
            int128, int128_result,
            "Converting from u128 (little-endian) byte array results in the same u128 value"
        );

        let int_size_array = int_size.to_ne_bytes();
        let int_size_result = unsafe { read_usize_le(&int_size_array) };
        assert_eq!(
            int_size, int_size_result,
            "Converting from usize (little-endian) byte array results in the same usize value"
        );
    }

    #[test]
    fn test_unsigned_integers_ne() {
        let int8 = 12u8;
        let int16 = 932u16;
        let int32 = 192u32;
        let int64 = 192932u64;
        let int128 = 19249462u128;
        let int_size = 1932387283747usize;

        let int8_array = int8.to_ne_bytes();
        let int8_result = unsafe { read_u8_ne(&int8_array) };
        assert_eq!(
            int8, int8_result,
            "Converting from u8 (native-endian) byte array results in the same u8 value"
        );

        let int16_array = int16.to_ne_bytes();
        let int16_result = unsafe { read_u16_ne(&int16_array) };
        assert_eq!(
            int16, int16_result,
            "Converting from u16 (native-endian) byte array results in the same u16 value"
        );

        let int32_array = int32.to_ne_bytes();
        let int32_result = unsafe { read_u32_ne(&int32_array) };
        assert_eq!(
            int32, int32_result,
            "Converting from u32 (native-endian) byte array results in the same u32 value"
        );

        let int64_array = int64.to_ne_bytes();
        let int64_result = unsafe { read_u64_ne(&int64_array) };
        assert_eq!(
            int64, int64_result,
            "Converting from u64 (native-endian) byte array results in the same u64 value"
        );

        let int128_array = int128.to_ne_bytes();
        let int128_result = unsafe { read_u128_ne(&int128_array) };
        assert_eq!(
            int128, int128_result,
            "Converting from u128 (native-endian) byte array results in the same u128 value"
        );

        let int_size_array = int_size.to_ne_bytes();
        let int_size_result = unsafe { read_usize_ne(&int_size_array) };
        assert_eq!(
            int_size, int_size_result,
            "Converting from usize (native-endian) byte array results in the same usize value"
        );
    }

    #[test]
    fn test_half_be() {
        let float16: f16 = f16::from_f32_const(10f32);
        let float16_array = float16.to_be_bytes();
        let float16_result = unsafe { read_f16_be(&float16_array) };
        assert_eq!(
            float16, float16_result,
            "Converting from f16 (big-endian) byte array results in the same f16 value"
        );
    }

    #[test]
    fn test_half_le() {
        let float16: f16 = f16::from_f32_const(10f32);
        let float16_array = float16.to_le_bytes();
        let float16_result = unsafe { read_f16_le(&float16_array) };
        assert_eq!(
            float16, float16_result,
            "Converting from f16 (little-endian) byte array results in the same f16 value"
        );
    }

    #[test]
    fn test_half_ne() {
        let float16: f16 = f16::from_f32_const(10f32);
        let float16_array = float16.to_ne_bytes();
        let float16_result = unsafe { read_f16_ne(&float16_array) };
        assert_eq!(
            float16, float16_result,
            "Converting from f16 (native-endian) byte array results in the same f16 value"
        );
    }

    #[test]
    fn test_floating_points_be() {
        let float32 = 192f32;
        let float64 = 192932f64;

        let float32_array = float32.to_be_bytes();
        let float32_result = unsafe { read_f32_be(&float32_array) };
        assert_eq!(
            float32, float32_result,
            "Converting from f32 (big-endian) byte array results in the same f32 value"
        );

        let float64_array = float64.to_be_bytes();
        let float64_result = unsafe { read_f64_be(&float64_array) };
        assert_eq!(
            float64, float64_result,
            "Converting from f64 (big-endian) byte array results in the same f64 value"
        );
    }

    #[test]
    fn test_floating_points_le() {
        let float32 = 192f32;
        let float64 = 192932f64;

        let float32_array = float32.to_le_bytes();
        let float32_result = unsafe { read_f32_le(&float32_array) };
        assert_eq!(
            float32, float32_result,
            "Converting from f32 (little-endian) byte array results in the same f32 value"
        );

        let float64_array = float64.to_le_bytes();
        let float64_result = unsafe { read_f64_le(&float64_array) };
        assert_eq!(
            float64, float64_result,
            "Converting from f64 (little-endian) byte array results in the same f64 value"
        );
    }

    #[test]
    fn test_floating_points_ne() {
        let float32 = 192f32;
        let float64 = 192932f64;

        let float32_array = float32.to_ne_bytes();
        let float32_result = unsafe { read_f32_ne(&float32_array) };
        assert_eq!(
            float32, float32_result,
            "Converting from f32 (native-endian) byte array results in the same f32 value"
        );

        let float64_array = float64.to_ne_bytes();
        let float64_result = unsafe { read_f64_ne(&float64_array) };
        assert_eq!(
            float64, float64_result,
            "Converting from f64 (native-endian) byte array results in the same f64 value"
        );
    }

    #[derive(PartialOrd, PartialEq, Debug)]
    struct MyTestStruct {
        unsigned8: u8,
        unsigned16: u16,
        unsigned32: u32,
        unsigned64: u64,
        unsigned128: u128,
        unsigned_size: usize,
        signed8: i8,
        signed16: i16,
        signed32: i32,
        signed64: i64,
        signed128: i128,
        signed_size: isize,
        float16: f16,
        float32: f32,
        float64: f64,
        raw_data: Vec<u8>,
    }

    impl Default for MyTestStruct {
        fn default() -> Self {
            Self {
                unsigned8: 220,
                unsigned16: 1034,
                unsigned32: 3,
                unsigned64: 293454,
                unsigned128: 13298439298,
                unsigned_size: 118283,
                signed8: 126,
                signed16: 193,
                signed32: 544,
                signed64: 353,
                signed128: 546755,
                signed_size: 43818,
                float16: f16::from_f32_const(93.21),
                float32: 38.358482,
                float64: 32848.23488,
                raw_data: vec![82u8, 38u8, 10u8, 2u8, 31u8, 165u8],
            }
        }
    }

    impl MyTestStruct {
        pub fn to_be_bytes(&self) -> Vec<u8> {
            let mut writer = FastByteWriter::new();
            writer.write_u8_be(self.unsigned8);
            writer.write_u16_be(self.unsigned16);
            writer.write_u32_be(self.unsigned32);
            writer.write_u64_be(self.unsigned64);
            writer.write_u128_be(self.unsigned128);
            writer.write_usize_be(self.unsigned_size);

            writer.write_i8_be(self.signed8);
            writer.write_i16_be(self.signed16);
            writer.write_i32_be(self.signed32);
            writer.write_i64_be(self.signed64);
            writer.write_i128_be(self.signed128);
            writer.write_isize_be(self.signed_size);

            writer.write_f16_be(self.float16);
            writer.write_f32_be(self.float32);
            writer.write_f64_be(self.float64);

            writer.write_bytes_be(&self.raw_data);

            writer.to_vec()
        }

        pub fn to_le_bytes(&self) -> Vec<u8> {
            let mut writer = FastByteWriter::new();
            writer.write_u8_le(self.unsigned8);
            writer.write_u16_le(self.unsigned16);
            writer.write_u32_le(self.unsigned32);
            writer.write_u64_le(self.unsigned64);
            writer.write_u128_le(self.unsigned128);
            writer.write_usize_le(self.unsigned_size);

            writer.write_i8_le(self.signed8);
            writer.write_i16_le(self.signed16);
            writer.write_i32_le(self.signed32);
            writer.write_i64_le(self.signed64);
            writer.write_i128_le(self.signed128);
            writer.write_isize_le(self.signed_size);

            writer.write_f16_le(self.float16);
            writer.write_f32_le(self.float32);
            writer.write_f64_le(self.float64);

            writer.write_bytes_le(&self.raw_data);

            writer.to_vec()
        }

        pub fn to_ne_bytes(&self) -> Vec<u8> {
            let mut writer = FastByteWriter::new();
            writer.write_u8_ne(self.unsigned8);
            writer.write_u16_ne(self.unsigned16);
            writer.write_u32_ne(self.unsigned32);
            writer.write_u64_ne(self.unsigned64);
            writer.write_u128_ne(self.unsigned128);
            writer.write_usize_ne(self.unsigned_size);

            writer.write_i8_ne(self.signed8);
            writer.write_i16_ne(self.signed16);
            writer.write_i32_ne(self.signed32);
            writer.write_i64_ne(self.signed64);
            writer.write_i128_ne(self.signed128);
            writer.write_isize_ne(self.signed_size);

            writer.write_f16_ne(self.float16);
            writer.write_f32_ne(self.float32);
            writer.write_f64_ne(self.float64);

            writer.write_bytes_ne(&self.raw_data);

            writer.to_vec()
        }
    }

    #[test]
    fn fast_reader_ne() {
        let my_struct = MyTestStruct::default();
        let vector_data = my_struct.to_ne_bytes();
        let mut fast_reader = FastByteReader::new(&vector_data);
        let parsed_struct = MyTestStruct {
            unsigned8: fast_reader.read_u8_ne(),
            unsigned16: fast_reader.read_u16_ne(),
            unsigned32: fast_reader.read_u32_ne(),
            unsigned64: fast_reader.read_u64_ne(),
            unsigned128: fast_reader.read_u128_ne(),
            unsigned_size: fast_reader.read_usize_ne(),
            signed8: fast_reader.read_i8_ne(),
            signed16: fast_reader.read_i16_ne(),
            signed32: fast_reader.read_i32_ne(),
            signed64: fast_reader.read_i64_ne(),
            signed128: fast_reader.read_i128_ne(),
            signed_size: fast_reader.read_isize_ne(),
            float16: fast_reader.read_f16_ne(),
            float32: fast_reader.read_f32_ne(),
            float64: fast_reader.read_f64_ne(),
            raw_data: fast_reader.read_n_ne(6),
        };

        assert_eq!(
            parsed_struct, my_struct,
            "Converting using Native Endian Reader"
        );
    }

    #[test]
    fn fast_reader_be() {
        let my_struct = MyTestStruct::default();
        let vector_data = my_struct.to_be_bytes();
        let mut fast_reader = FastByteReader::new(&vector_data);
        let parsed_struct = MyTestStruct {
            unsigned8: fast_reader.read_u8_be(),
            unsigned16: fast_reader.read_u16_be(),
            unsigned32: fast_reader.read_u32_be(),
            unsigned64: fast_reader.read_u64_be(),
            unsigned128: fast_reader.read_u128_be(),
            unsigned_size: fast_reader.read_usize_be(),
            signed8: fast_reader.read_i8_be(),
            signed16: fast_reader.read_i16_be(),
            signed32: fast_reader.read_i32_be(),
            signed64: fast_reader.read_i64_be(),
            signed128: fast_reader.read_i128_be(),
            signed_size: fast_reader.read_isize_be(),
            float16: fast_reader.read_f16_be(),
            float32: fast_reader.read_f32_be(),
            float64: fast_reader.read_f64_be(),
            raw_data: fast_reader.read_n_be(6),
        };

        assert_eq!(
            parsed_struct, my_struct,
            "Converting using Big Endian Reader"
        );
    }

    #[test]
    fn fast_reader_le() {
        let my_struct = MyTestStruct::default();
        let vector_data = my_struct.to_le_bytes();
        let mut fast_reader = FastByteReader::new(&vector_data);
        let parsed_struct = MyTestStruct {
            unsigned8: fast_reader.read_u8_le(),
            unsigned16: fast_reader.read_u16_le(),
            unsigned32: fast_reader.read_u32_le(),
            unsigned64: fast_reader.read_u64_le(),
            unsigned128: fast_reader.read_u128_le(),
            unsigned_size: fast_reader.read_usize_le(),
            signed8: fast_reader.read_i8_le(),
            signed16: fast_reader.read_i16_le(),
            signed32: fast_reader.read_i32_le(),
            signed64: fast_reader.read_i64_le(),
            signed128: fast_reader.read_i128_le(),
            signed_size: fast_reader.read_isize_le(),
            float16: fast_reader.read_f16_le(),
            float32: fast_reader.read_f32_le(),
            float64: fast_reader.read_f64_le(),
            raw_data: fast_reader.read_n_le(6),
        };

        assert_eq!(
            parsed_struct, my_struct,
            "Converting using Big Endian Reader"
        );
    }

    #[test]
    fn skips() {
        let mut bytes = Vec::new();
        102u8.to_ne_bytes().into_iter().for_each(|x| bytes.push(x));
        122u8.to_ne_bytes().into_iter().for_each(|x| bytes.push(x));

        1345u16
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        1567u16
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        1345445335u32
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        1561366457u32
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        134545847675787u64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        156721881824556u64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        1345458476757874858183847456634787169u128
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        1567218818245541501092162486263847136u128
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        134545847675787485usize
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        1567218818245541445usize
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        102i8.to_ne_bytes().into_iter().for_each(|x| bytes.push(x));
        122i8.to_ne_bytes().into_iter().for_each(|x| bytes.push(x));

        1345i16
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        1567i16
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        1345445335i32
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        1561366457i32
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        134545847675787i64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        156721881824556i64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        1345458476757874858183847456634787169i128
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        1567218818245541501092162486263847136i128
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        1345458476757874858isize
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        1567218818245541506isize
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        f16::from_f32(383.0)
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        f16::from_f32(323.0)
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        3942.543f32
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        3956.33f32
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        39545342436.5633f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        3954534243436.1834f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        39545342436.56331f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        39545342436.56332f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        39545342436.56333f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        3954534243436.1834f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        39545342436.56331f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        39545342436.56332f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        39545342436.56333f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));
        3954534243436.1834f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| bytes.push(x));

        let mut reader = FastByteReader::new(&bytes);
        reader.skip_u8();
        assert_eq!(122u8, reader.read_u8_ne());
        reader.skip_u16();
        assert_eq!(1567u16, reader.read_u16_ne());
        reader.skip_u32();
        assert_eq!(1561366457u32, reader.read_u32_ne());
        reader.skip_u64();
        assert_eq!(156721881824556u64, reader.read_u64_ne());
        reader.skip_u128();
        assert_eq!(
            1567218818245541501092162486263847136u128,
            reader.read_u128_ne()
        );
        reader.skip_usize();
        assert_eq!(1567218818245541445usize, reader.read_usize_ne());

        reader.skip_i8();
        assert_eq!(122i8, reader.read_i8_ne());
        reader.skip_i16();
        assert_eq!(1567i16, reader.read_i16_ne());
        reader.skip_i32();
        assert_eq!(1561366457i32, reader.read_i32_ne());
        reader.skip_i64();
        assert_eq!(156721881824556i64, reader.read_i64_ne());
        reader.skip_i128();
        assert_eq!(
            1567218818245541501092162486263847136i128,
            reader.read_i128_ne()
        );
        reader.skip_isize();
        assert_eq!(1567218818245541506isize, reader.read_isize_ne());

        reader.skip_f16();
        assert_eq!(f16::from_f32(323.0), reader.read_f16_ne());
        reader.skip_f32();
        assert_eq!(3956.33f32, reader.read_f32_ne());
        reader.skip_f64();
        assert_eq!(3954534243436.1834f64, reader.read_f64_ne());

        reader.skip_n(16);
        assert_eq!(39545342436.56333f64, reader.read_f64_ne());
        assert_eq!(3954534243436.1834f64, reader.read_f64_ne());

        let mut equate_bytes = Vec::new();
        39545342436.56331f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| equate_bytes.push(x));
        39545342436.56332f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| equate_bytes.push(x));
        39545342436.56333f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| equate_bytes.push(x));
        3954534243436.1834f64
            .to_ne_bytes()
            .into_iter()
            .for_each(|x| equate_bytes.push(x));

        assert_eq!(reader.read_n_ne(8 * 4), equate_bytes);
    }

    // Has bound checks for every indexing operation
    #[inline(always)]
    pub fn read_f64_ne_indexing(bytes: &[u8]) -> f64 {
        f64::from_ne_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    // Has more branching and operations involved, and ultimately
    #[inline(always)]
    pub fn read_f64_ne_ordinary(bytes: &[u8]) -> f64 {
        f64::from_ne_bytes(bytes.try_into().expect("Error!"))
    }

    #[cfg_attr(miri, ignore)]
    #[test]
    fn my_bench_mark() {
        let my_value = hint::black_box(1000f64);
        let mut bytes;
        let mut result = my_value;
        // Use of blackbox to attempt to prevent release mode from not running the code
        let start = Instant::now();
        for _ in 0..1_000_000_000 {
            bytes = result.to_ne_bytes();
            result = hint::black_box(unsafe { read_f64_ne(hint::black_box(&bytes)) });
        }
        let end = start.elapsed();
        println!("Fast {result} - {}s", end.as_secs_f64());

        let start = Instant::now();
        for _ in 0..1_000_000_000 {
            bytes = result.to_ne_bytes();
            result = hint::black_box(read_f64_ne_indexing(hint::black_box(&bytes)));
        }
        let end = start.elapsed();
        println!("Indexing: {result} - {}s", end.as_secs_f64());

        let start = Instant::now();
        for _ in 0..1_000_000_000 {
            bytes = result.to_ne_bytes();
            result = hint::black_box(read_f64_ne_ordinary(hint::black_box(&bytes)));
        }
        let end = start.elapsed();
        println!("Ordinary: {result} - {}s", end.as_secs_f64());
    }
}
