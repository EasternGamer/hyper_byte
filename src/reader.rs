#[cfg(feature = "half")]
use half::f16;
use std::ptr::slice_from_raw_parts;


/// Safe, cheap and simple reader for a [`u8`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u8`]
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

/// Safe, cheap and simple reader for a [`u16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u16`]
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

/// Safe, cheap and simple reader for a [`u32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u32`]
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

/// Safe, cheap and simple reader for a [`u64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u64`]
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

/// Safe, cheap and simple reader for a [`u128`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u128`]
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

/// Safe, cheap and simple reader for a [`usize`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`usize`]
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

/// Safe, cheap and simple reader for a [`i8`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`i8`]
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

/// Safe, cheap and simple reader for a [`i16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`i16`]
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

/// Safe, cheap and simple reader for a [`i32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u32`]
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

/// Safe, cheap and simple reader for a [`i64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u64`]
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

/// Safe, cheap and simple reader for a [`u128`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u128`]
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

/// Safe, cheap and simple reader for a [`isize`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`isize`]
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
/// Safe, cheap and simple reader for a [`f16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f16`]
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

/// Safe, cheap and simple reader for a [`f32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f32`]
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

/// Safe, cheap and simple reader for a [`f64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f64`]
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

/// Safe, cheap and simple reader for a [`u8`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u8`]
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

/// Safe, cheap and simple reader for a [`u16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u16`]
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

/// Safe, cheap and simple reader for a [`u32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u32`]
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

/// Safe, cheap and simple reader for a [`u64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u64`]
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

/// Safe, cheap and simple reader for a [`u128`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u128`]
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

/// Safe, cheap and simple reader for a [`usize`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`usize`]
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

/// Safe, cheap and simple reader for a [`i8`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`i8`]
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

/// Safe, cheap and simple reader for a [`i16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`i16`]
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

/// Safe, cheap and simple reader for a [`i32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u32`]
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

/// Safe, cheap and simple reader for a [`i64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u64`]
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

/// Safe, cheap and simple reader for a [`u128`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u128`]
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

/// Safe, cheap and simple reader for a [`isize`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`isize`]
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
/// Safe, cheap and simple reader for a [`f16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f16`]
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

/// Safe, cheap and simple reader for a [`f32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f32`]
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

/// Safe, cheap and simple reader for a [`f64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f64`]
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

/// Safe, cheap and simple reader for a [`u8`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
/// 
/// returns: [`u8`]
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

/// Safe, cheap and simple reader for a [`u16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u16`]
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

/// Safe, cheap and simple reader for a [`u32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u32`]
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

/// Safe, cheap and simple reader for a [`u64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u64`]
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

/// Safe, cheap and simple reader for a [`u128`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u128`]
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

/// Safe, cheap and simple reader for a [`usize`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`usize`]
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

/// Safe, cheap and simple reader for a [`i8`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`i8`]
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

/// Safe, cheap and simple reader for a [`i16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`i16`]
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

/// Safe, cheap and simple reader for a [`i32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u32`]
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

/// Safe, cheap and simple reader for a [`i64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u64`]
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

/// Safe, cheap and simple reader for a [`u128`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`u128`]
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

/// Safe, cheap and simple reader for a [`isize`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`isize`]
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
/// Safe, cheap and simple reader for a [`f16`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f16`]
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

/// Safe, cheap and simple reader for a [`f32`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f32`]
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

/// Safe, cheap and simple reader for a [`f64`]
/// If you want to use an even faster reader, use [`FastByteReader`]
/// # Arguments
/// * `bytes`: the byte array reference
/// * `index`: the mutable index to use as reference 
///
/// returns: [`f64`]
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

/// Cheap byte reader, which does not hold your hand. If you mess up, it will panic.
///
/// # Examples
/// ```
/// use hyper_byte::reader::FastByteReader;
///
/// let slice = [0u8; 32];
/// let mut reader = FastByteReader::new(&slice);
/// let x = reader.read_f64_ne();
/// let y = reader.read_f64_ne();
/// ```
pub struct FastByteReader<'reader> {
    byte_array: &'reader [u8]
}

impl<'reader> FastByteReader<'reader> {
    /// Cheap byte reader, which does not hold your hand. If you mess up, it will panic.
    ///
    /// # Examples
    /// ```
    /// use hyper_byte::reader::FastByteReader;
    ///
    /// let slice = [0u8; 32];
    /// let mut reader = FastByteReader::new(&slice);
    /// let x = reader.read_f64_ne();
    /// let y = reader.read_f64_ne();
    /// ```
    pub const fn new(byte_array: &'reader [u8]) -> Self {
        Self {
            byte_array
        }
    }

    /// Reads a big-endian [`u8`] from the byte array, advancing the reader forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u8_be(&mut self) -> u8 {
        let byte_size_needed = size_of::<u8>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u8_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u8 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`u16`] from the byte array, advancing the reader forward by 2 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u16_be(&mut self) -> u16 {
        let byte_size_needed = size_of::<u16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u16_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u16 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`u32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u32_be(&mut self) -> u32 {
        let byte_size_needed = size_of::<u32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u32_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u32 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`u64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u64_be(&mut self) -> u64 {
        let byte_size_needed = size_of::<u64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u64_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u64 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`u128`] from the byte array, advancing the reader forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u128_be(&mut self) -> u128 {
        let byte_size_needed = size_of::<u128>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u128_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u128 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian unsigned big-endian usize from the byte array, advancing the reader forward by [`size_of::<usize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_usize_be(&mut self) -> usize {
        let byte_size_needed = size_of::<usize>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_usize_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read usize of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i8`] from the byte array, advancing the reader forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i8_be(&mut self) -> i8 {
        let byte_size_needed = size_of::<i8>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i8_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i8 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i16`] from the byte array, advancing the reader forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i16_be(&mut self) -> i16 {
        let byte_size_needed = size_of::<i16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i16_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i16 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i32_be(&mut self) -> i32 {
        let byte_size_needed = size_of::<i32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i32_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i32 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i64_be(&mut self) -> i64 {
        let byte_size_needed = size_of::<i64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i64_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i64 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`i128`] from the byte array, advancing the reader forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i128_be(&mut self) -> i128 {
        let byte_size_needed = size_of::<i128>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i128_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i128 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`isize`] from the byte array, advancing the reader forward by [`size_of::<isize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_isize_be(&mut self) -> isize {
        let byte_size_needed = size_of::<isize>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_isize_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read isize of big-endian of an array without enough space within the array.");
        }
    }

    #[cfg(feature = "half")]
    /// Reads a big-endian [`f16`] from the byte array, advancing the reader forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f16_be(&mut self) -> f16 {
        let byte_size_needed = size_of::<f16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f16_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f16 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`f32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f32_be(&mut self) -> f32 {
        let byte_size_needed = size_of::<f32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f32_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f32 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a big-endian [`f64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f64_be(&mut self) -> f64 {
        let byte_size_needed = size_of::<f64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f64_be(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f64 of big-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u8`] from the byte array, advancing the reader forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u8_le(&mut self) -> u8 {
        let byte_size_needed = size_of::<u8>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u8_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u8 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u16`] from the byte array, advancing the reader forward by 2 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u16_le(&mut self) -> u16 {
        let byte_size_needed = size_of::<u16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u16_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u16 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u32_le(&mut self) -> u32 {
        let byte_size_needed = size_of::<u32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u32_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u32 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u64_le(&mut self) -> u64 {
        let byte_size_needed = size_of::<u64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u64_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u64 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`u128`] from the byte array, advancing the reader forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u128_le(&mut self) -> u128 {
        let byte_size_needed = size_of::<u128>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u128_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u128 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian unsigned little-endian usize from the byte array, advancing the reader forward by [`size_of::<usize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_usize_le(&mut self) -> usize {
        let byte_size_needed = size_of::<usize>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_usize_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read usize of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i8`] from the byte array, advancing the reader forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i8_le(&mut self) -> i8 {
        let byte_size_needed = size_of::<i8>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i8_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i8 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i16`] from the byte array, advancing the reader forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i16_le(&mut self) -> i16 {
        let byte_size_needed = size_of::<i16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i16_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i16 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i32_le(&mut self) -> i32 {
        let byte_size_needed = size_of::<i32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i32_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i32 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i64_le(&mut self) -> i64 {
        let byte_size_needed = size_of::<i64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i64_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i64 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`i128`] from the byte array, advancing the reader forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i128_le(&mut self) -> i128 {
        let byte_size_needed = size_of::<i128>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i128_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i128 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`isize`] from the byte array, advancing the reader forward by [`size_of::<isize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_isize_le(&mut self) -> isize {
        let byte_size_needed = size_of::<isize>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_isize_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read isize of little-endian of an array without enough space within the array.");
        }
    }

    #[cfg(feature = "half")]
    /// Reads a little-endian [`f16`] from the byte array, advancing the reader forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f16_le(&mut self) -> f16 {
        let byte_size_needed = size_of::<f16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f16_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f16 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`f32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f32_le(&mut self) -> f32 {
        let byte_size_needed = size_of::<f32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f32_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f32 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a little-endian [`f64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f64_le(&mut self) -> f64 {
        let byte_size_needed = size_of::<f64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f64_le(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f64 of little-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u8`] from the byte array, advancing the reader forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u8_ne(&mut self) -> u8 {
        let byte_size_needed = size_of::<u8>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u8_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u8 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u16`] from the byte array, advancing the reader forward by 2 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u16_ne(&mut self) -> u16 {
        let byte_size_needed = size_of::<u16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u16_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u16 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u32_ne(&mut self) -> u32 {
        let byte_size_needed = size_of::<u32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u32_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u32 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u64_ne(&mut self) -> u64 {
        let byte_size_needed = size_of::<u64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u64_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u64 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`u128`] from the byte array, advancing the reader forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_u128_ne(&mut self) -> u128 {
        let byte_size_needed = size_of::<u128>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_u128_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read u128 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian unsigned native-endian usize from the byte array, advancing the reader forward by [`size_of::<usize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_usize_ne(&mut self) -> usize {
        let byte_size_needed = size_of::<usize>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_usize_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read usize of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i8`] from the byte array, advancing the reader forward by 1 byte</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i8_ne(&mut self) -> i8 {
        let byte_size_needed = size_of::<i8>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i8_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i8 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i16`] from the byte array, advancing the reader forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i16_ne(&mut self) -> i16 {
        let byte_size_needed = size_of::<i16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i16_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i16 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i32_ne(&mut self) -> i32 {
        let byte_size_needed = size_of::<i32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i32_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i32 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i64_ne(&mut self) -> i64 {
        let byte_size_needed = size_of::<i64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i64_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i64 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`i128`] from the byte array, advancing the reader forward by 16 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_i128_ne(&mut self) -> i128 {
        let byte_size_needed = size_of::<i128>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_i128_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read i128 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`isize`] from the byte array, advancing the reader forward by [`size_of::<isize>()`] bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_isize_ne(&mut self) -> isize {
        let byte_size_needed = size_of::<isize>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_isize_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read isize of native-endian of an array without enough space within the array.");
        }
    }

    #[cfg(feature = "half")]
    /// Reads a native-endian [`f16`] from the byte array, advancing the reader forward by 2 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f16_ne(&mut self) -> f16 {
        let byte_size_needed = size_of::<f16>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f16_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f16 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`f32`] from the byte array, advancing the reader forward by 4 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f32_ne(&mut self) -> f32 {
        let byte_size_needed = size_of::<f32>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f32_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f32 of native-endian of an array without enough space within the array.");
        }
    }

    /// Reads a native-endian [`f64`] from the byte array, advancing the reader forward by 8 bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_f64_ne(&mut self) -> f64 {
        let byte_size_needed = size_of::<f64>();
        let new_length = (self.byte_array.len() as isize) - byte_size_needed as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let data = crate::read_f64_ne(self.byte_array);
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size_needed), new_length as usize);
                data
            }
        } else {
            panic!("Attempted to read f64 of native-endian of an array without enough space within the array.");
        }
    }

    /// Skips a `[u8]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_u8(&mut self) {
        self.skip_n(size_of::<u8>());
    }

    /// Skips an `[i16]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_u16(&mut self) {
        self.skip_n(size_of::<u16>());
    }

    /// Skips an `[i32]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_u32(&mut self) {
        self.skip_n(size_of::<u32>());
    }

    /// Skips an `[u64]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_u64(&mut self) {
        self.skip_n(size_of::<u64>());
    }

    /// Skips an `[u128]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_u128(&mut self) {
        self.skip_n(size_of::<u128>());
    }

    /// Skips an `[usize]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_usize(&mut self) {
        self.skip_n(size_of::<usize>());
    }

    /// Skips an `[i8]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_i8(&mut self) {
        self.skip_n(size_of::<i8>());
    }

    /// Skips an `[i16]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_i16(&mut self) {
        self.skip_n(size_of::<i16>());
    }

    /// Skips an `[i32]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_i32(&mut self) {
        self.skip_n(size_of::<i32>());
    }

    /// Skips an `[i64]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_i64(&mut self) {
        self.skip_n(size_of::<i64>());
    }

    /// Skips an `[i128]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_i128(&mut self) {
        self.skip_n(size_of::<i128>());
    }

    /// Skips an `[isize]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_isize(&mut self) {
        self.skip_n(size_of::<isize>());
    }

    #[cfg(feature = "half")]
    /// Skips a `[f16]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_f16(&mut self) {
        self.skip_n(size_of::<f16>());
    }

    /// Skips a `[f32]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_f32(&mut self) {
        self.skip_n(size_of::<f32>());
    }

    /// Skips a `[f64]` number of bytes from the byte array, advancing the reader forward</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn skip_f64(&mut self) {
        self.skip_n(size_of::<f64>());
    }

    /// Skips a custom number of bytes from the byte array, advancing the reader forward by the specified [`byte_size`] of bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub const fn skip_n(&mut self, byte_size : usize) {
        let new_length = (self.byte_array.len() as isize) - byte_size as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size), new_length as usize);
            }
        } else {
            panic!("Attempted to skip bytes of an array without space in the array.");
        }
    }

    /// Reads a custom number of bytes from the byte array, advancing the reader forward by the specified [`byte_size`] of bytes</br>
    /// Panics if there is not enough space
    #[inline(always)]
    pub fn read_n(&mut self, byte_size : usize) -> Vec<u8> {
        let new_length = (self.byte_array.len() as isize) - byte_size as isize;
        if new_length >= 0 {
            // SAFETY: This will only execute if it is within bounds of the array
            unsafe {
                let bytes = self.byte_array[..byte_size].to_vec();
                self.byte_array = &*slice_from_raw_parts(self.byte_array.as_ptr().add(byte_size), new_length as usize);
                bytes
            }
        } else {
            panic!("Attempted to read custom number bytes of an array without space in the array.");
        }
    }
}