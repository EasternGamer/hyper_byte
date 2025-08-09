#![cfg(feature = "std")]
use std::io::Read;
use std::mem::MaybeUninit;


use crate::writers::traits::ByteWriter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;



// Taken from nightly, to avoid nightly requirements
#[inline(always)]
const unsafe fn assume_init_mut<T>(array: &mut [MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
    // mutable reference which is also guaranteed to be valid for writes.
    unsafe { &mut *(array as *mut [MaybeUninit<T>] as *mut [T]) }
}


pub trait HyperReadExactExt {
    fn read_into_exact<T: ByteWriter>(
        &mut self,
        buffer: &mut T,
        amount: usize,
    ) -> Result<(), std::io::Error>;
}

impl<R: Read> HyperReadExactExt for R {
    fn read_into_exact<T: ByteWriter>(
        &mut self,
        buffer: &mut T,
        amount: usize,
    ) -> Result<(), std::io::Error> {
        let inner = buffer.as_mut_vec();
        inner.reserve(amount);

        let old_length = inner.len();
        let reading_slice = unsafe { assume_init_mut(&mut inner.spare_capacity_mut()[..amount]) };

        self.read_exact(reading_slice)?;

        let new_length = amount + old_length;
        unsafe { inner.set_len(new_length) }
        Ok(())
    }
}

#[cfg(feature = "tokio")]
pub trait AsyncHyperReadExactExt {
    async fn read_into_exact<T: ByteWriter>(
        &mut self,
        buffer: &mut T,
        amount: usize,
    ) -> Result<usize, std::io::Error>;
}

#[cfg(feature = "tokio")]
impl<R: AsyncReadExt + Unpin> AsyncHyperReadExactExt for R {
    async fn read_into_exact<T: ByteWriter>(
        &mut self,
        buffer: &mut T,
        amount: usize,
    ) -> Result<usize, std::io::Error> {
        let inner = buffer.as_mut_vec();
        inner.reserve(amount);

        let length = inner.len();
        let reading_slice = unsafe { assume_init_mut(&mut inner.spare_capacity_mut()[..amount]) };

        let size_read = self.read_exact(reading_slice).await?;

        let new_length = length + size_read;
        unsafe { inner.set_len(new_length) };
        Ok(size_read)
    }
}
