// Copyright (c) 2018 Stefan Lankes, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod freelist;
pub mod simple_allocator;

use crate::arch;
use crate::arch::mm::get_memory_size;
use crate::logging::*;
#[cfg(not(test))]
use alloc::alloc::Layout;

pub fn init() {
    info!("Initializing memory...");
    let mem_size = get_memory_size();
    info!("Detected memory size: {} MBytes", mem_size >> 20);

    arch::mm::init(); // Ensure this does not loop or hang

    // Additional diagnostic checks
    debug!("Memory manager initialized successfully.");
}

#[cfg(not(test))]
#[alloc_error_handler]
pub fn rust_oom(layout: Layout) -> ! {
    println!(
        "[!!!OOM!!!] Memory allocation of {} bytes failed at align {}",
        layout.size(),
        layout.align()
    );

    arch::irq::irq_disable(); // Disable interrupts for safety
    loop {
        arch::processor::halt(); // Halt instead of looping indefinitely
    }
}
