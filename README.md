# Hyper Byte
An unsafe byte slice transmuter and very fast iterator-like reader for Rust's numeric types, for all three endianness'.<br/>
Additionally included is a simple-to-use byte reader.<br/>
**Supported types:**
* `u8`
* `u16`
* `u32`
* `u64`
* `u128`
* `i8`
* `i16`
* `i32`
* `i64`
* `i128`
* `f16` (If you have the `half` crate)
* `f32`
* `f64`
* `usize`
* `isize`

## Why?
What a great question. There are plenty of ways to do what this crate does, and there are plenty of crates which already do something similar.
### For example
```rust
// Has bound checks for every indexing operation
#[no_mangle]
#[inline(always)]
pub fn read_f64_ne(bytes: &[u8]) -> f64 {
    f64::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

// Has more branching and operations involved, and ultimately should be slower
#[no_mangle]
#[inline(always)]
pub fn read_f64_ne(bytes: &[u8]) -> f64 {
    f64::from_ne_bytes(bytes.try_into().expect("Error!"))
}

// My version of this, taking what try_into does and removing the branching,
// thus making it unsafe, but can easily be made safe
#[no_mangle]
#[inline(always)]
pub unsafe fn read_f64_ne(bytes: &[u8]) -> f64 {
    unsafe { f64::from_ne_bytes(*(bytes.as_ptr() as *const [u8; 8])) }
}
```
Benchmarking it is rather difficult since the compiler will do anything to optimize the call out completely (not run the code, I don't mean make it faster).
However, instructions don't lie, and I did manage to create a benchmark, present around line 2256 in [lib.rs](src/lib.rs#L2286-L2314).<br/>
In [Compiler Explorer](https://rust.godbolt.org/z/PfhWzGnnG), you can also see for yourself the instructions for each function.

Running it on my machine, in debug mode, it is around 150% to 200% faster than `try_into`. In release mode, it is closer to only 20% faster.
> [!NOTE]
> While I am very confident that this micro-optimization is faster than the existing solutions, I stand by this only until I'm otherwise corrected.

## Usage
### Fast Byte Reader
If you want the fastest possible reader without going into completely unsafe territory, then you should use the FastByteReader.<br/>
Simply, it is an iterator-like reader where reading a type will result in consuming up the reader.<br/>
There are no results/options here, it will simply panic if you attempt to read bytes that don't exist. Thus, using this reader means the expected input has a very predictable content.
```rust
use hyper_byte::reader::FastReader;

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
    float16: f16, // if `half` is enabled
    float32: f32,
    float64: f64,
}

fn main() {
    let some_byte_stream : Vec<u8> = Vec::new();
    
    let fast_reader = FastReader::new(&some_byte_stream);
    
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
    };
}
```
### Basic Reader
If you want a more "hands on" reader implementation with a bit more protection and nicer panic messages, this is the reader for you. It is slower than the fast reader (it has 13 instructions vs 9 instructions for the fast reader) because it has an index and additional bound check.
```rust
use hyper_byte::reader;

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
    float16: f16, // if `half` is enabled
    float32: f32,
    float64: f64,
}

fn main() {
    let some_byte_stream : Vec<u8> = Vec::new();
    let mut index = 0;

    let parsed_struct = MyTestStruct {
        unsigned8: reader::read_u8_le(&vector_data, &mut index),
        unsigned16: reader::read_u16_le(&vector_data, &mut index),
        unsigned32: reader::read_u32_le(&vector_data, &mut index),
        unsigned64: reader::read_u64_le(&vector_data, &mut index),
        unsigned128: reader::read_u128_le(&vector_data, &mut index),
        unsigned_size: reader::read_usize_le(&vector_data, &mut index),
        signed8: reader::read_i8_le(&vector_data, &mut index),
        signed16: reader::read_i16_le(&vector_data, &mut index), // if `half` is enabled
        signed32: reader::read_i32_le(&vector_data, &mut index),
        signed64: reader::read_i64_le(&vector_data, &mut index),
        signed128: reader::read_i128_le(&vector_data, &mut index),
        signed_size: reader::read_isize_le(&vector_data, &mut index),
        float16: reader_read_f16_le(&vector_data, &mut index),
        float32: reader::read_f32_le(&vector_data, &mut index),
        float64: reader::read_f64_le(&vector_data, &mut index),
    };
}
```
### Unsafe Functions
This is for if you have an even faster way of doing these operations, or want to use these functions standalone in someway. It is quite literally 1-2 instructions. It is 1 instruction using native-endian.
```rust
pub fn read_u16_be(array : &[u8], index: &mut usize) -> u16 {
    let current_index = *index;
    let new_index = current_index + size_of::<u16>();
    let ranged_array = &array[current_index..new_index];
    *index = new_index;
    // SAFETY: Ranged array will not allow this function to proceed to unsafe code if there aren't enough bytes to read
    unsafe {
        hyper_byte::read_u16_be(ranged_array)
    }
}
```