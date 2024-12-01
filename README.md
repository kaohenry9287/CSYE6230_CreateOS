# LightOS-rs - A lightweight testing operating system written in Rust

## Introduction

LightOS-rs is a Unix-like operating system based on a monolithic architecture for educational purposes.
It is developed for the course "CSYE6230: Operating Systems" at Northeastern University and includes a modified hypervisor that simplifies the boot process to increase the intelligibility of the OS.

Here are the 4 main function of this OS:

1. Allocated memory
2. Deallocated memory
3. File Management
4. Task Management (Preemptive multitasking, synchronization primitives)

## Building

After cloning the repository, you can run the kernel with following command:

```sh
$ cargo build
```

```sh
$ cargo run
```
