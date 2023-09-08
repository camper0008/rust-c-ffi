use std::alloc::Layout;

#[repr(C)]
#[derive(Debug)]
struct Data {
    message: *const u8,
    length: usize,
}

extern "C" { fn print_and_transform(data: *mut Data); }

#[no_mangle]
pub extern "C" fn free_cringe_ptr(ptr: *mut u8, len: usize) {
    println!("[Rust Land] Freeing cringe ptr...");
    unsafe {
        std::alloc::dealloc(ptr, Layout::from_size_align_unchecked(len, 1))
    }
}

fn main() {
    let original_message = b"Hello world from rust!";
    let message = unsafe {
        std::alloc::alloc(Layout::from_size_align_unchecked(original_message.len(), 1))
    };

    (0..original_message.len()).for_each(|i| {
        unsafe {
            *message.offset(i as isize) = original_message[i];
        }
    });
    
    let data = Box::new(Data {length: original_message.len(), message});
    let data = Box::into_raw(data);
    unsafe {
        print_and_transform(data);
        print!("[Rust Land] Received transformed message: ");
        let data = Box::from_raw(data);
        (0..data.length).for_each(|i| {
            print!("{}", *data.message.offset(i as isize) as char);
        });
        println!();
    }
}
