# LightOS-rs - A lightweight testing operating system written in Rust

<img width="498" alt="截圖 2024-12-01 下午3 28 31" src="https://github.com/user-attachments/assets/b8881062-d799-4f9b-a18c-16904d8a1701">

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
