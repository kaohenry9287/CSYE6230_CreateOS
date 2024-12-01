use core::ptr::NonNull;

const MEMORY_POOL_SIZE: usize = 1024; // Simple pool size for demonstration
static mut MEMORY_POOL: [u8; MEMORY_POOL_SIZE] = [0; MEMORY_POOL_SIZE];
static mut ALLOCATED: [bool; MEMORY_POOL_SIZE] = [false; MEMORY_POOL_SIZE];

/// Initialize the memory pool (optional in this simple example)
pub fn init() {
    unsafe {
        for i in 0..MEMORY_POOL_SIZE {
            MEMORY_POOL[i] = 0;
            ALLOCATED[i] = false;
        }
    }
}

/// Allocate a block of memory of the given size
pub fn alloc(size: usize) -> Option<NonNull<u8>> {
    if size == 0 || size > MEMORY_POOL_SIZE {
        return None;
    }

    unsafe {
        for i in 0..MEMORY_POOL_SIZE - size {
            if ALLOCATED[i..i + size].iter().all(|&x| !x) {
                for j in i..i + size {
                    ALLOCATED[j] = true;
                }
                return Some(NonNull::new_unchecked(&mut MEMORY_POOL[i]));
            }
        }
    }
    None
}

/// Deallocate a block of memory starting at a given pointer
pub fn dealloc(ptr: NonNull<u8>, size: usize) {
    let start = ptr.as_ptr() as usize;
    unsafe {
        for i in start..start + size {
            if i < MEMORY_POOL_SIZE {
                ALLOCATED[i] = false;
            }
        }
    }
}
