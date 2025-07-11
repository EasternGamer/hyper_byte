# Hyper Byte
An unsafe byte slice transmuter and very fast iterator-like reader and writer for Rust's numeric types, for all three endianness'.<br/>
<br/>
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
* `&[u8]` (a binary slice for any generic size)

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
However, instructions don't lie, and I did manage to create a benchmark, present around line 2512 in [lib.rs](src/lib.rs#L2512-L2540).<br/>
In [Compiler Explorer](https://rust.godbolt.org/z/PfhWzGnnG), you can also see for yourself the instructions for each function.

Running it on my machine, in debug mode, it is around 150% to 200% faster than `try_into`. In release mode, it is closer to only 20% faster.
> [!NOTE]
> While I am very confident that this micro-optimization is faster than the existing solutions, I stand by this only until I'm otherwise corrected.

## Usage
There are several-prebuilt readers/writers available, driven by traits.<br/>For reader traits it is:
- [NativeEndianByteReader](src/readers/traits.rs#L140-L444)
- [LittleEndianByteReader](src/readers/traits.rs#L446-L750)
- [BigEndianByteReader](src/readers/traits.rs#L752-L1056)
- For reader implementations, there is [FastByteReader](src/reader.rs#L4-L51), [NetworkReader](src/reader.rs#L53-L98), [LittleReader](src/reader.rs#L100-L145), and [NativeReader](src/reader.rs#L147-L193).

For writer traits it is:
- [NativeEndianByteWriter](src/writers/traits.rs#L72-L475)
- [LittleEndianByteWriter](src/writers/traits.rs#L477-L880)
- [BigEndianByteWriter](src/writers/traits.rs#L882-L1285)
- For writer implementations, there is [FastByteWriter](src/writer.rs#L3-L71), [NetworkWriter](src/writer.rs#L73-L139), [LittleWriter](src/writer.rs#L141-L207), and [NativeWriter](src/writer.rs#L209-L275)

For a combined experience, reading and writing, look to [NetworkStream](src/hyper-stream.rs#L8-L45), [LittleEndianStream](src/hyper-stream.rs#L47-L84),  [NativeStream](src/hyper-stream.rs#L86-L123), and [HyperStream](src/hyper-stream.rs#L125-L166).

You might be wondering... why does FastByteReader and FastByteWriter exist? Well, it is to enable cursed functionality such as switching between different endianness.
At the heart of it, all these implementations are incredibly simple to re-implement in your own structs, you can even do a hybrid reader/writer.

### Fast Byte Reader
If you want the fastest possible reader without going into completely unsafe territory, then you should use the FastByteReader.<br/>
Simply, it is an iterator-like reader where reading a type will result in consuming up the reader.<br/>
There are no results/options here, it will simply panic if you attempt to read bytes that don't exist. Thus, using this reader means the expected input has a very predictable content.
```rust
use hyper_byte::reader::FastByteReader;
use hyper_byte::readers::traits::BigEndianByteReader;

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
    raw_data: vec![82u8, 38u8, 10u8, 2u8, 31u8, 165u8],
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
        raw_data: fast_reader.read_n_be(6)
    };
}
```
### Fast Byte Writer
If you want the fast writer with no particular endian-ness, without going into completely unsafe territory, then you should use the FastByteWriter.<br/>
```rust
use hyper_byte::writer::FastByteWriter;
use hyper_byte::writers::traits::*;

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
    raw_data: vec![82u8, 38u8, 10u8, 2u8, 31u8, 165u8],
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

## Disclaimer
Most of the core code used in this library comes directly from Rust's own library, such as `*(bytes.as_ptr() as *const [u8; 8])`. This is effectively what try_into is, except without the runtime checking and result capture. It is therefor super-unsafe if not used correctly. This code is far from original.
