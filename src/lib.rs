#[cfg(feature = "half")]
use half::f16;

/// Unsafe, near zero cost transmutation of a byte array slice into an unsigned 8-bit integer using big-endianness.
/// # Safety
/// To make it safe and does not cause memory errors, you must ensure the input has at least 1 byte prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: `u8`
///
/// # Examples
/// ```
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
/// returns: `u16`
///
/// # Examples
/// ```
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
/// returns: `u32`
///
/// # Examples
/// ```
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
/// returns: `u64`
///
/// # Examples
/// ```
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
/// returns: `u128`
///
/// # Examples
/// ```
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
/// returns: `usize`
///
/// # Examples
/// ```
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
/// returns: `i8`
///
/// # Examples
/// ```
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
/// returns: `i16`
///
/// # Examples
/// ```
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
/// returns: `i32`
///
/// # Examples
/// ```
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
/// returns: `i64`
///
/// # Examples
/// ```
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
/// returns: `i128`
///
/// # Examples
/// ```
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
/// ```
#[inline(always)]
pub unsafe fn read_i128_be(bytes: &[u8]) -> i128 {
    unsafe { i128::from_be_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

#[cfg(feature = "half")]
/// Unsafe, near zero cost transmutation of a byte array slice into a 16-bit floating point using big-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: `f16`
///
/// # Examples
/// ```
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
/// returns: `f32`
///
/// # Examples
/// ```
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
/// returns: `f64`
///
/// # Examples
/// ```
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
/// returns: `u8`
///
/// # Examples
/// ```
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
/// returns: `u16`
///
/// # Examples
/// ```
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
/// returns: `u32`
///
/// # Examples
/// ```
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
/// returns: `u64`
///
/// # Examples
/// ```
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
/// returns: `u128`
///
/// # Examples
/// ```
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
/// returns: `usize`
///
/// # Examples
/// ```
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
/// returns: `i8`
///
/// # Examples
/// ```
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
/// returns: `i16`
///
/// # Examples
/// ```
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
/// returns: `i32`
///
/// # Examples
/// ```
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
/// returns: `i64`
///
/// # Examples
/// ```
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
/// returns: `i128`
///
/// # Examples
/// ```
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
/// ```
#[inline(always)]
pub unsafe fn read_i128_le(bytes: &[u8]) -> i128 {
    unsafe { i128::from_le_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

#[cfg(feature = "half")]
/// Unsafe, near zero cost transmutation of a byte array slice into a 16-bit floating point using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: `f16`
///
/// # Examples
/// ```
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
/// returns: `f32`
///
/// # Examples
/// ```
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
/// returns: `f64`
///
/// # Examples
/// ```
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
/// returns: `u8`
///
/// # Examples
/// ```
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
/// returns: `u16`
///
/// # Examples
/// ```
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
/// returns: `u32`
///
/// # Examples
/// ```
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
/// returns: `u64`
///
/// # Examples
/// ```
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
/// returns: `u128`
///
/// # Examples
/// ```
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
/// returns: `usize`
///
/// # Examples
/// ```
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
/// returns: `i8`
///
/// # Examples
/// ```
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
/// returns: `i16`
///
/// # Examples
/// ```
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
/// returns: `i32`
///
/// # Examples
/// ```
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
/// returns: `i64`
///
/// # Examples
/// ```
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
/// returns: `i128`
///
/// # Examples
/// ```
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
/// ```
#[inline(always)]
pub unsafe fn read_i128_ne(bytes: &[u8]) -> i128 {
    unsafe { i128::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 16])) }
}

#[cfg(feature = "half")]
/// Unsafe, near zero cost transmutation of a byte array slice into a 16-bit floating point using little-endianness.<br/>
/// # Safety
/// To make it "safe" and does not cause memory errors, you must ensure the input has at least 2 bytes prior to calling this.
/// # Arguments
/// * `bytes`: the byte array reference
///
/// returns: `f16`
///
/// # Examples
/// ```
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
/// returns: `f32`
///
/// # Examples
/// ```
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
/// returns: `f64`
///
/// # Examples
/// ```
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
/// ```
#[inline(always)]
pub unsafe fn read_f64_ne(bytes: &[u8]) -> f64 {
    unsafe { f64::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "half")]
    use half::f16;
    
    use pretty_assertions::{assert_eq};
    use super::*;

    #[test]
    fn test_signed_integers_be() {
        let int8 = 12i8;
        let int16 = 932i16;
        let int32 = 192i32;
        let int64 = 192932i64;
        let int128 = 19249462i128;

        let int8_array = int8.to_be_bytes();
        let int8_result = unsafe { read_i8_be(&int8_array) };
        assert_eq!(int8, int8_result, "Converting from i8 (big-endian) byte array results in the same i8 value");

        let int16_array = int16.to_be_bytes();
        let int16_result = unsafe { read_i16_be(&int16_array) };
        assert_eq!(int16, int16_result, "Converting from i16 (big-endian) byte array results in the same i16 value");

        let int32_array = int32.to_be_bytes();
        let int32_result = unsafe { read_i32_be(&int32_array) };
        assert_eq!(int32, int32_result, "Converting from i32 (big-endian) byte array results in the same i32 value");

        let int64_array = int64.to_be_bytes();
        let int64_result = unsafe { read_i64_be(&int64_array) };
        assert_eq!(int64, int64_result, "Converting from i64 (big-endian) byte array results in the same i64 value");

        let int128_array = int128.to_be_bytes();
        let int128_result = unsafe { read_i128_be(&int128_array) };
        assert_eq!(int128, int128_result, "Converting from i128 (big-endian) byte array results in the same i128 value");
    }

    #[test]
    fn test_signed_integers_le() {
        let int8 = 12i8;
        let int16 = 932i16;
        let int32 = 192i32;
        let int64 = 192932i64;
        let int128 = 19249462i128;

        let int8_array = int8.to_le_bytes();
        let int8_result = unsafe { read_i8_le(&int8_array) };
        assert_eq!(int8, int8_result, "Converting from i8 (little-endian) byte array results in the same i8 value");

        let int16_array = int16.to_le_bytes();
        let int16_result = unsafe { read_i16_le(&int16_array) };
        assert_eq!(int16, int16_result, "Converting from i16 (little-endian) byte array results in the same i16 value");

        let int32_array = int32.to_le_bytes();
        let int32_result = unsafe { read_i32_le(&int32_array) };
        assert_eq!(int32, int32_result, "Converting from i32 (little-endian) byte array results in the same i32 value");

        let int64_array = int64.to_le_bytes();
        let int64_result = unsafe { read_i64_le(&int64_array) };
        assert_eq!(int64, int64_result, "Converting from i64 (little-endian) byte array results in the same i64 value");

        let int128_array = int128.to_le_bytes();
        let int128_result = unsafe { read_i128_le(&int128_array) };
        assert_eq!(int128, int128_result, "Converting from i128 (little-endian) byte array results in the same i128 value");
    }

    #[test]
    fn test_signed_integers_ne() {
        let int8 = 12i8;
        let int16 = 932i16;
        let int32 = 192i32;
        let int64 = 192932i64;
        let int128 = 19249462i128;

        let int8_array = int8.to_ne_bytes();
        let int8_result = unsafe { read_i8_ne(&int8_array) };
        assert_eq!(int8, int8_result, "Converting from i8 (little-endian) byte array results in the same i8 value");

        let int16_array = int16.to_ne_bytes();
        let int16_result = unsafe { read_i16_ne(&int16_array) };
        assert_eq!(int16, int16_result, "Converting from i16 (little-endian) byte array results in the same i16 value");

        let int32_array = int32.to_ne_bytes();
        let int32_result = unsafe { read_i32_ne(&int32_array) };
        assert_eq!(int32, int32_result, "Converting from i32 (little-endian) byte array results in the same i32 value");

        let int64_array = int64.to_ne_bytes();
        let int64_result = unsafe { read_i64_ne(&int64_array) };
        assert_eq!(int64, int64_result, "Converting from i64 (little-endian) byte array results in the same i64 value");

        let int128_array = int128.to_ne_bytes();
        let int128_result = unsafe { read_i128_ne(&int128_array) };
        assert_eq!(int128, int128_result, "Converting from i128 (little-endian) byte array results in the same i128 value");
    }

    #[test]
    fn test_unsigned_integers_be() {
        let int8 = 12u8;
        let int16 = 932u16;
        let int32 = 192u32;
        let int64 = 192932u64;
        let int128 = 19249462u128;

        let int8_array = int8.to_be_bytes();
        let int8_result = unsafe { read_u8_be(&int8_array) };
        assert_eq!(int8, int8_result, "Converting from u8 (big-endian) byte array results in the same u8 value");

        let int16_array = int16.to_be_bytes();
        let int16_result = unsafe { read_u16_be(&int16_array) };
        assert_eq!(int16, int16_result, "Converting from u16 (big-endian) byte array results in the same u16 value");

        let int32_array = int32.to_be_bytes();
        let int32_result = unsafe { read_u32_be(&int32_array) };
        assert_eq!(int32, int32_result, "Converting from u32 (big-endian) byte array results in the same u32 value");

        let int64_array = int64.to_be_bytes();
        let int64_result = unsafe { read_u64_be(&int64_array) };
        assert_eq!(int64, int64_result, "Converting from u64 (big-endian) byte array results in the same u64 value");

        let int128_array = int128.to_be_bytes();
        let int128_result = unsafe { read_u128_be(&int128_array) };
        assert_eq!(int128, int128_result, "Converting from u128 (big-endian) byte array results in the same u128 value");
    }

    #[test]
    fn test_unsigned_integers_le() {
        let int8 = 12u8;
        let int16 = 932u16;
        let int32 = 192u32;
        let int64 = 192932u64;
        let int128 = 19249462u128;

        let int8_array = int8.to_le_bytes();
        let int8_result = unsafe { read_u8_le(&int8_array) };
        assert_eq!(int8, int8_result, "Converting from u8 (little-endian) byte array results in the same u8 value");

        let int16_array = int16.to_le_bytes();
        let int16_result = unsafe { read_u16_le(&int16_array) };
        assert_eq!(int16, int16_result, "Converting from u16 (little-endian) byte array results in the same u16 value");

        let int32_array = int32.to_le_bytes();
        let int32_result = unsafe { read_u32_le(&int32_array) };
        assert_eq!(int32, int32_result, "Converting from u32 (little-endian) byte array results in the same u32 value");

        let int64_array = int64.to_le_bytes();
        let int64_result = unsafe { read_u64_le(&int64_array) };
        assert_eq!(int64, int64_result, "Converting from u64 (little-endian) byte array results in the same u64 value");

        let int128_array = int128.to_le_bytes();
        let int128_result = unsafe { read_u128_le(&int128_array) };
        assert_eq!(int128, int128_result, "Converting from u128 (little-endian) byte array results in the same u128 value");
    }

    #[test]
    fn test_unsigned_integers_ne() {
        let int8 = 12u8;
        let int16 = 932u16;
        let int32 = 192u32;
        let int64 = 192932u64;
        let int128 = 19249462u128;

        let int8_array = int8.to_ne_bytes();
        let int8_result = unsafe { read_u8_ne(&int8_array) };
        assert_eq!(int8, int8_result, "Converting from u8 (native-endian) byte array results in the same u8 value");

        let int16_array = int16.to_ne_bytes();
        let int16_result = unsafe { read_u16_ne(&int16_array) };
        assert_eq!(int16, int16_result, "Converting from u16 (native-endian) byte array results in the same u16 value");

        let int32_array = int32.to_ne_bytes();
        let int32_result = unsafe { read_u32_ne(&int32_array) };
        assert_eq!(int32, int32_result, "Converting from u32 (native-endian) byte array results in the same u32 value");

        let int64_array = int64.to_ne_bytes();
        let int64_result = unsafe { read_u64_ne(&int64_array) };
        assert_eq!(int64, int64_result, "Converting from u64 (native-endian) byte array results in the same u64 value");

        let int128_array = int128.to_ne_bytes();
        let int128_result = unsafe { read_u128_ne(&int128_array) };
        assert_eq!(int128, int128_result, "Converting from u128 (native-endian) byte array results in the same u128 value");
    }

    #[cfg(feature = "half")]
    #[test]
    fn test_half_be() {
        let float16 : f16 = f16::from_f32_const(10f32);
        let float16_array = float16.to_be_bytes();
        let float16_result = unsafe { read_f16_be(&float16_array) };
        assert_eq!(float16, float16_result, "Converting from f16 (big-endian) byte array results in the same f16 value");
    }

    #[cfg(feature = "half")]
    #[test]
    fn test_half_le() {
        let float16 : f16 = f16::from_f32_const(10f32);
        let float16_array = float16.to_le_bytes();
        let float16_result = unsafe { read_f16_le(&float16_array) };
        assert_eq!(float16, float16_result, "Converting from f16 (little-endian) byte array results in the same f16 value");
    }

    #[cfg(feature = "half")]
    #[test]
    fn test_half_ne() {
        let float16 : f16 = f16::from_f32_const(10f32);
        let float16_array = float16.to_ne_bytes();
        let float16_result = unsafe { read_f16_ne(&float16_array) };
        assert_eq!(float16, float16_result, "Converting from f16 (native-endian) byte array results in the same f16 value");
    }
    
    #[test]
    fn test_floating_points_be() {
        let float32 = 192f32;
        let float64 = 192932f64;

        let float32_array = float32.to_be_bytes();
        let float32_result = unsafe { read_f32_be(&float32_array) };
        assert_eq!(float32, float32_result, "Converting from f32 (big-endian) byte array results in the same f32 value");

        let float64_array = float64.to_be_bytes();
        let float64_result = unsafe { read_f64_be(&float64_array) };
        assert_eq!(float64, float64_result, "Converting from f64 (big-endian) byte array results in the same f64 value");
    }

    #[test]
    fn test_floating_points_le() {
        let float32 = 192f32;
        let float64 = 192932f64;

        let float32_array = float32.to_le_bytes();
        let float32_result = unsafe { read_f32_le(&float32_array) };
        assert_eq!(float32, float32_result, "Converting from f32 (little-endian) byte array results in the same f32 value");

        let float64_array = float64.to_le_bytes();
        let float64_result = unsafe { read_f64_le(&float64_array) };
        assert_eq!(float64, float64_result, "Converting from f64 (little-endian) byte array results in the same f64 value");
    }

    #[test]
    fn test_floating_points_ne() {
        let float32 = 192f32;
        let float64 = 192932f64;

        let float32_array = float32.to_ne_bytes();
        let float32_result = unsafe { read_f32_ne(&float32_array) };
        assert_eq!(float32, float32_result, "Converting from f32 (native-endian) byte array results in the same f32 value");

        let float64_array = float64.to_ne_bytes();
        let float64_result = unsafe { read_f64_ne(&float64_array) };
        assert_eq!(float64, float64_result, "Converting from f64 (native-endian) byte array results in the same f64 value");
    }
    
}
