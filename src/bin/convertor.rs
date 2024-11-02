//! From https://www.reddit.com/r/learnrust/comments/13eyfcx/fastest_and_safest_way_to_convert_a_generic_type/
//! Also see bytemuck crate: https://crates.io/crates/bytemuck
use std::mem::{size_of, MaybeUninit};

unsafe fn from_bytes<T: Copy>(bytes: &[u8]) -> T {
    assert_eq!(bytes.len(), std::mem::size_of::<T>());

    let mut to: MaybeUninit<T> = MaybeUninit::uninit();

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), to.as_mut_ptr().cast::<u8>(), size_of::<T>());

        to.assume_init()
    }
}

fn to_bytes<T: Copy>(value: T) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(size_of::<T>());

    unsafe {
        std::ptr::copy_nonoverlapping(
            std::ptr::addr_of!(value).cast::<u8>(),
            bytes.as_mut_ptr(),
            size_of::<T>(),
        );
        bytes.set_len(size_of::<T>());
    }

    bytes
}

fn main() {
    let bytes = to_bytes(100);
    println!("{}", unsafe { from_bytes::<i32>(bytes.as_slice()) });
}
