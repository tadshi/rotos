# ROTOS
This is a toy operating system in Rust.

This repo is a personal work and a pastime for the author. However, as the author
no loner study Operating System but other fields now, the repo will may not be updated
and soon get archived.

The project is a cargo workspace. It may not be a good practice to
take the whole workspace a repo but anyway this is a toy project.

Basically this project tend to be a micro-kernel OS with high flexibility, fast and robust ITC
and new technologies. But after all, this is a toy project.

## Build
`cargo brq` to build the OS for a riscv64 qemu target.

The project ought to always compile on the newest nightly version of Rust.

## Run
The OS is typically test on a 4-hart qemu virtual machine. It runs under S-mode and use SBI.

A port towards other platform should be easy, even though it is not supported yet.
