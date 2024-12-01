#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate eduos_rs;
extern crate alloc;
#[cfg(target_arch = "x86_64")]
extern crate x86;


use alloc::string::String;
use eduos_rs::arch;
use eduos_rs::arch::load_application;
use eduos_rs::fs;
use eduos_rs::mm;
use mm::freelist::{FreeList, FreeListEntry};
use eduos_rs::scheduler;
use eduos_rs::scheduler::task::NORMAL_PRIORITY;
use eduos_rs::{LogLevel, LOGGER};

extern "C" fn create_user_foo() {
	let path = String::from("/bin/demo");

	info!("Hello from loader");

	// load application
	if load_application(&path).is_err() {
		error!("Unable to load elf64 binary {}", path)
	}
}

extern "C" fn foo() {
	let tid = scheduler::get_current_taskid();
    info!("Hello from task {}", tid);
}

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    info!("Operating System begins to start!");

	arch::init();
	mm::init();
	scheduler::init();
	fs::init();


	// Demonstrate memory allocation
    if let Some(ptr) = mm::simple_allocator::alloc(10) {
        info!("Allocated memory at: {:?}", ptr);

        // Deallocate the memory
        mm::simple_allocator::dealloc(ptr, 10);
        info!("Deallocated memory.");
    } else {
        info!("Failed to allocate memory.");
    }

	// Initialize a FreeList
    let mut freelist = FreeList::new();

    // Add an entry to the FreeList
    let entry = FreeListEntry::new(0x10000, 0x20000); // Example memory range
    freelist.list.push_back(entry);

    info!("FreeList initialized.");
    for node in freelist.list.iter() {
        info!("Start: 0x{:X}, End: 0x{:X}", node.start, node.end);
    }

    // Allocate memory from the FreeList
    if let Ok(addr) = freelist.allocate(0x1000, None) {
        info!("Allocated 0x1000 bytes at 0x{:X}", addr);
    } else {
        info!("Allocation failed");
    }

    info!("FreeList after allocation:");
    for node in freelist.list.iter() {
        info!("Start: 0x{:X}, End: 0x{:X}", node.start, node.end);
    }

    // Deallocate memory back to the FreeList
    freelist.deallocate(0x10000, 0x1000);

    info!("FreeList after deallocation:");
    for node in freelist.list.iter() {
        info!("Start: 0x{:X}, End: 0x{:X}", node.start, node.end);
    }


	for _i in 0..1 {
		scheduler::spawn(foo, NORMAL_PRIORITY).unwrap();
	}
	scheduler::spawn(create_user_foo, NORMAL_PRIORITY).unwrap();

	// enable interrupts => enable preemptive multitasking
	arch::irq::irq_enable();

	scheduler::reschedule();

    info!("Shutdown system!");
	// shutdown system
	arch::processor::shutdown();
}
