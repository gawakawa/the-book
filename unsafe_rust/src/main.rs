use std::slice;

#[allow(dead_code)]
/// # Safety
unsafe trait Foo {
    // method go here
}

unsafe impl Foo for i32 {
    // method implementation go here
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", std::ptr::addr_of!(COUNTER).read());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

#[allow(dead_code)]
fn ffi() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[allow(dead_code)]
fn call_safe_func_with_unsafe_block() {
    let mut v = [1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

#[allow(dead_code)]
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            #[allow(clippy::ptr_offset_with_cast)]
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

#[allow(dead_code)]
fn unsafe_func() {
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
}

#[allow(dead_code)]
fn basic_raw_poiner() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r is: {}", *r);
    }
}
