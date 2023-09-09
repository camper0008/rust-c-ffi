use std::alloc::Layout;

#[repr(C)]
#[derive(Debug)]
struct Data {
    message: *const u8,
    length: usize,
}

#[link(name = "c-land", kind = "static")]
extern "C" {
    fn print_and_transform(data: *mut Data);
}

#[no_mangle]
pub extern "C" fn realloc_cringe_ptr(ptr: *mut u8, old_len: usize, new_len: usize) -> *mut u8 {
    println!("[Rust Land] Reallocating cringe ptr...");
    unsafe { std::alloc::realloc(ptr, Layout::from_size_align_unchecked(old_len, 1), new_len) }
}

fn main() {
    let message = b"Hello world from rust!";
    let length = message.len();
    let message = unsafe {
        let message_ptr = std::alloc::alloc(Layout::from_size_align_unchecked(length, 1));
        std::ptr::copy_nonoverlapping(message as *const _, message_ptr, length);
        message_ptr
    };

    let data = Box::new(Data { length, message });
    let data = Box::into_raw(data);
    unsafe {
        print_and_transform(data);
        print!("[Rust Land] Received transformed message: ");
        let data = Box::from_raw(data);
        (0..data.length)
            .map(|i| *data.message.offset(i as isize) as char)
            .for_each(|c| print!("{c}"));
        println!();
    }
}
