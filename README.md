# Simple VM

![Build](https://github.com/renjithgr/simple-vm/workflows/Build/badge.svg)

A simple bytecode virtual machine based on the [LLJSVM](https://github.com/LowLevelJavaScript/16-Bit-Virtual-Machine) by [Francis Stokes](https://github.com/francisrstokes).

## Why?

- To improve my understanding of how computers work.
- To learn and practice [Rust](https://www.rust-lang.org/)
- And to have fun!

## Test & Build

```sh
cargo test
cargo build
```

## VM Instructions

| Instruction   | Arguments      | Description |
|---------------|----------------|-------------|
| `MOV_LIT_REG` | `LIT, REG`     | Moves a 16 bit literal to a register. e.g., `MOV_LIT_REG 0x12 0x34 0x02`|
| `MOV_LIT_R2`  | `LIT`          | Move 16 bit literal to the register R2 |
| `ADD_REG_REG` | `R1, R2`       | Add values of registers provided and place the result in the accumulator|

## Registers

| Register   | Offset      |Description|
|------------|-------------|-----------|
|`IP`        |`0x00`       | Instruction pointer |
|`Acc`       |`0x01`       | Accumulator |
|`R1`        |`0x02`       | Register 1 |
|`R2`        |`0x03`       | Register 2 |
|`R3`        |`0x04`       | Register 2 |
