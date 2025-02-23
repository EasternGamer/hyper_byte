#[cfg(feature = "half")]
use half::f16;


/// Safe, very cheap and simple reader for a `u8`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u8`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u8_be(&slice, &mut index);
/// let y = reader::read_u8_be(&slice, &mut index);
/// ```
pub fn read_u8_be(array : &[u8], index: &mut usize) -> u8 {
    let current_index = *index;
    let new_index = current_index + size_of::<u8>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u8_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u16_be(&slice, &mut index);
/// let y = reader::read_u16_be(&slice, &mut index);
/// ```
pub fn read_u16_be(array : &[u8], index: &mut usize) -> u16 {
    let current_index = *index;
    let new_index = current_index + size_of::<u16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u16_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u32_be(&slice, &mut index);
/// let y = reader::read_u32_be(&slice, &mut index);
/// ```
pub fn read_u32_be(array : &[u8], index: &mut usize) -> u32 {
    let current_index = *index;
    let new_index = current_index + size_of::<u32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u32_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u64_be(&slice, &mut index);
/// let y = reader::read_u64_be(&slice, &mut index);
/// ```
pub fn read_u64_be(array : &[u8], index: &mut usize) -> u64 {
    let current_index = *index;
    let new_index = current_index + size_of::<u64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u64_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u128`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u128`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u128_be(&slice, &mut index);
/// let y = reader::read_u128_be(&slice, &mut index);
/// ```
pub fn read_u128_be(array : &[u8], index: &mut usize) -> u128 {
    let current_index = *index;
    let new_index = current_index + size_of::<u128>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u128_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `usize`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `usize`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u128_be(&slice, &mut index);
/// let y = reader::read_u128_be(&slice, &mut index);
/// ```
pub fn read_usize_be(array : &[u8], index: &mut usize) -> usize {
    let current_index = *index;
    let new_index = current_index + size_of::<usize>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_usize_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i8`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `i8`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i8_be(&slice, &mut index);
/// let y = reader::read_i8_be(&slice, &mut index);
/// ```
pub fn read_i8_be(array : &[u8], index: &mut usize) -> i8 {
    let current_index = *index;
    let new_index = current_index + size_of::<i8>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i8_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `i16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u16_be(&slice, &mut index);
/// let y = reader::read_u16_be(&slice, &mut index);
/// ```
pub fn read_i16_be(array : &[u8], index: &mut usize) -> i16 {
    let current_index = *index;
    let new_index = current_index + size_of::<i16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i16_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i32_be(&slice, &mut index);
/// let y = reader::read_i32_be(&slice, &mut index);
/// ```
pub fn read_i32_be(array : &[u8], index: &mut usize) -> i32 {
    let current_index = *index;
    let new_index = current_index + size_of::<i32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i32_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i64_be(&slice, &mut index);
/// let y = reader::read_i64_be(&slice, &mut index);
/// ```
pub fn read_i64_be(array : &[u8], index: &mut usize) -> i64 {
    let current_index = *index;
    let new_index = current_index + size_of::<i64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i64_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u128`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u128`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i128_be(&slice, &mut index);
/// let y = reader::read_i128_be(&slice, &mut index);
/// ```
pub fn read_i128_be(array : &[u8], index: &mut usize) -> i128 {
    let current_index = *index;
    let new_index = current_index + size_of::<i128>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i128_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `isize`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `isize`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_isize_be(&slice, &mut index);
/// let y = reader::read_isize_be(&slice, &mut index);
/// ```
pub fn read_isize_be(array : &[u8], index: &mut usize) -> isize {
    let current_index = *index;
    let new_index = current_index + size_of::<isize>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_isize_be(ranged_array)
    }
}

#[cfg(feature = "half")]
/// Safe, very cheap and simple reader for a `f16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f16_be(&slice, &mut index);
/// let y = reader::read_f16_be(&slice, &mut index);
/// ```
pub fn read_f16_be(array : &[u8], index: &mut usize) -> f16 {
    let current_index = *index;
    let new_index = current_index + size_of::<f16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f16_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `f32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f32_be(&slice, &mut index);
/// let y = reader::read_f32_be(&slice, &mut index);
/// ```
pub fn read_f32_be(array : &[u8], index: &mut usize) -> f32 {
    let current_index = *index;
    let new_index = current_index + size_of::<f32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f32_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `f64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f64_be(&slice, &mut index);
/// let y = reader::read_f64_be(&slice, &mut index);
/// ```
pub fn read_f64_be(array : &[u8], index: &mut usize) -> f64 {
    let current_index = *index;
    let new_index = current_index + size_of::<f64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f64_be(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u8`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u8`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u8_le(&slice, &mut index);
/// let y = reader::read_u8_le(&slice, &mut index);
/// ```
pub fn read_u8_le(array : &[u8], index: &mut usize) -> u8 {
    let current_index = *index;
    let new_index = current_index + size_of::<u8>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u8_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u16_le(&slice, &mut index);
/// let y = reader::read_u16_le(&slice, &mut index);
/// ```
pub fn read_u16_le(array : &[u8], index: &mut usize) -> u16 {
    let current_index = *index;
    let new_index = current_index + size_of::<u16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u16_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u32_le(&slice, &mut index);
/// let y = reader::read_u32_le(&slice, &mut index);
/// ```
pub fn read_u32_le(array : &[u8], index: &mut usize) -> u32 {
    let current_index = *index;
    let new_index = current_index + size_of::<u32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u32_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u64_le(&slice, &mut index);
/// let y = reader::read_u64_le(&slice, &mut index);
/// ```
pub fn read_u64_le(array : &[u8], index: &mut usize) -> u64 {
    let current_index = *index;
    let new_index = current_index + size_of::<u64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u64_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u128`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u128`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u128_le(&slice, &mut index);
/// let y = reader::read_u128_le(&slice, &mut index);
/// ```
pub fn read_u128_le(array : &[u8], index: &mut usize) -> u128 {
    let current_index = *index;
    let new_index = current_index + size_of::<u128>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u128_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `usize`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `usize`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u128_le(&slice, &mut index);
/// let y = reader::read_u128_le(&slice, &mut index);
/// ```
pub fn read_usize_le(array : &[u8], index: &mut usize) -> usize {
    let current_index = *index;
    let new_index = current_index + size_of::<usize>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_usize_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i8`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `i8`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i8_le(&slice, &mut index);
/// let y = reader::read_i8_le(&slice, &mut index);
/// ```
pub fn read_i8_le(array : &[u8], index: &mut usize) -> i8 {
    let current_index = *index;
    let new_index = current_index + size_of::<i8>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i8_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `i16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u16_le(&slice, &mut index);
/// let y = reader::read_u16_le(&slice, &mut index);
/// ```
pub fn read_i16_le(array : &[u8], index: &mut usize) -> i16 {
    let current_index = *index;
    let new_index = current_index + size_of::<i16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i16_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i32_le(&slice, &mut index);
/// let y = reader::read_i32_le(&slice, &mut index);
/// ```
pub fn read_i32_le(array : &[u8], index: &mut usize) -> i32 {
    let current_index = *index;
    let new_index = current_index + size_of::<i32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i32_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i64_le(&slice, &mut index);
/// let y = reader::read_i64_le(&slice, &mut index);
/// ```
pub fn read_i64_le(array : &[u8], index: &mut usize) -> i64 {
    let current_index = *index;
    let new_index = current_index + size_of::<i64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i64_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u128`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u128`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i128_le(&slice, &mut index);
/// let y = reader::read_i128_le(&slice, &mut index);
/// ```
pub fn read_i128_le(array : &[u8], index: &mut usize) -> i128 {
    let current_index = *index;
    let new_index = current_index + size_of::<i128>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i128_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `isize`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `isize`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_isize_le(&slice, &mut index);
/// let y = reader::read_isize_le(&slice, &mut index);
/// ```
pub fn read_isize_le(array : &[u8], index: &mut usize) -> isize {
    let current_index = *index;
    let new_index = current_index + size_of::<isize>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_isize_le(ranged_array)
    }
}

#[cfg(feature = "half")]
/// Safe, very cheap and simple reader for a `f16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f16_le(&slice, &mut index);
/// let y = reader::read_f16_le(&slice, &mut index);
/// ```
pub fn read_f16_le(array : &[u8], index: &mut usize) -> f16 {
    let current_index = *index;
    let new_index = current_index + size_of::<f16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f16_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `f32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f32_le(&slice, &mut index);
/// let y = reader::read_f32_le(&slice, &mut index);
/// ```
pub fn read_f32_le(array : &[u8], index: &mut usize) -> f32 {
    let current_index = *index;
    let new_index = current_index + size_of::<f32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f32_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `f64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f64_le(&slice, &mut index);
/// let y = reader::read_f64_le(&slice, &mut index);
/// ```
pub fn read_f64_le(array : &[u8], index: &mut usize) -> f64 {
    let current_index = *index;
    let new_index = current_index + size_of::<f64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f64_le(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u8`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
/// 
/// returns: `u8`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u8_ne(&slice, &mut index);
/// let y = reader::read_u8_ne(&slice, &mut index);
/// ```
pub fn read_u8_ne(array : &[u8], index: &mut usize) -> u8 {
    let current_index = *index;
    let new_index = current_index + size_of::<u8>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u8_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u16_ne(&slice, &mut index);
/// let y = reader::read_u16_ne(&slice, &mut index);
/// ```
pub fn read_u16_ne(array : &[u8], index: &mut usize) -> u16 {
    let current_index = *index;
    let new_index = current_index + size_of::<u16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u16_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u32_ne(&slice, &mut index);
/// let y = reader::read_u32_ne(&slice, &mut index);
/// ```
pub fn read_u32_ne(array : &[u8], index: &mut usize) -> u32 {
    let current_index = *index;
    let new_index = current_index + size_of::<u32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u32_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u64_ne(&slice, &mut index);
/// let y = reader::read_u64_ne(&slice, &mut index);
/// ```
pub fn read_u64_ne(array : &[u8], index: &mut usize) -> u64 {
    let current_index = *index;
    let new_index = current_index + size_of::<u64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u64_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u128`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u128`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u128_ne(&slice, &mut index);
/// let y = reader::read_u128_ne(&slice, &mut index);
/// ```
pub fn read_u128_ne(array : &[u8], index: &mut usize) -> u128 {
    let current_index = *index;
    let new_index = current_index + size_of::<u128>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_u128_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `usize`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `usize`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u128_ne(&slice, &mut index);
/// let y = reader::read_u128_ne(&slice, &mut index);
/// ```
pub fn read_usize_ne(array : &[u8], index: &mut usize) -> usize {
    let current_index = *index;
    let new_index = current_index + size_of::<usize>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_usize_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i8`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `i8`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i8_ne(&slice, &mut index);
/// let y = reader::read_i8_ne(&slice, &mut index);
/// ```
pub fn read_i8_ne(array : &[u8], index: &mut usize) -> i8 {
    let current_index = *index;
    let new_index = current_index + size_of::<i8>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i8_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `i16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_u16_ne(&slice, &mut index);
/// let y = reader::read_u16_ne(&slice, &mut index);
/// ```
pub fn read_i16_ne(array : &[u8], index: &mut usize) -> i16 {
    let current_index = *index;
    let new_index = current_index + size_of::<i16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i16_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i32_ne(&slice, &mut index);
/// let y = reader::read_i32_ne(&slice, &mut index);
/// ```
pub fn read_i32_ne(array : &[u8], index: &mut usize) -> i32 {
    let current_index = *index;
    let new_index = current_index + size_of::<i32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i32_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `i64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i64_ne(&slice, &mut index);
/// let y = reader::read_i64_ne(&slice, &mut index);
/// ```
pub fn read_i64_ne(array : &[u8], index: &mut usize) -> i64 {
    let current_index = *index;
    let new_index = current_index + size_of::<i64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i64_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `u128`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `u128`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_i128_ne(&slice, &mut index);
/// let y = reader::read_i128_ne(&slice, &mut index);
/// ```
pub fn read_i128_ne(array : &[u8], index: &mut usize) -> i128 {
    let current_index = *index;
    let new_index = current_index + size_of::<i128>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_i128_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `isize`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `isize`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_isize_ne(&slice, &mut index);
/// let y = reader::read_isize_ne(&slice, &mut index);
/// ```
pub fn read_isize_ne(array : &[u8], index: &mut usize) -> isize {
    let current_index = *index;
    let new_index = current_index + size_of::<isize>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_isize_ne(ranged_array)
    }
}

#[cfg(feature = "half")]
/// Safe, very cheap and simple reader for a `f16`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f16`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f16_ne(&slice, &mut index);
/// let y = reader::read_f16_ne(&slice, &mut index);
/// ```
pub fn read_f16_ne(array : &[u8], index: &mut usize) -> f16 {
    let current_index = *index;
    let new_index = current_index + size_of::<f16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f16_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `f32`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f32`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f32_ne(&slice, &mut index);
/// let y = reader::read_f32_ne(&slice, &mut index);
/// ```
pub fn read_f32_ne(array : &[u8], index: &mut usize) -> f32 {
    let current_index = *index;
    let new_index = current_index + size_of::<f32>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f32_ne(ranged_array)
    }
}

/// Safe, very cheap and simple reader for a `f64`
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: `f64`
///
/// # Examples
/// ```
/// use hyper_byte::reader;
///
/// let mut slice = [0u8; 32];
/// let mut index = 0;
///
/// let x = reader::read_f64_ne(&slice, &mut index);
/// let y = reader::read_f64_ne(&slice, &mut index);
/// ```
pub fn read_f64_ne(array : &[u8], index: &mut usize) -> f64 {
    let current_index = *index;
    let new_index = current_index + size_of::<f64>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        crate::read_f64_ne(ranged_array)
    }
}