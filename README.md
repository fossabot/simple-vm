# Simple VM

![Build](https://github.com/renjithgr/simple-vm/workflows/Build/badge.svg)

A simple bytecode virtual machine based on the [LLJSVM](https://github.com/LowLevelJavaScript/16-Bit-Virtual-Machine) by [Francis Stokes](https://github.com/francisrstokes).

## Why?

- To improve my understanding of how computers work.
- To learn and practice [Rust](https://www.rust-lang.org/)
- And to have fun!

## Project Structure

```
|
|- vm
|- assembler
|- screen-device
|- examples
```

## Test & Build

```sh
cargo test
cargo build
```

## Run Examples

```sh
cargo run example screen-device-example
```

## VM Instructions

| Instruction    | Arguments      | Description |
|----------------|----------------|-------------|
| `MOV_LIT_REG`  | `LIT, REG`     | Moves a 16 bit literal to a register. e.g., `MOV_LIT_REG 0x12 0x34 0x02`|
| `MOV_REG_REG`  | `REG_FROM, REG_TO`     | Move the value of `REG_FROM` to `REG_TO`. e.g., `MOV_REG_REG 0x02 0x03` |
| `ADD_REG_REG`  | `R1, R2`       | Add values of registers provided and place the result in the accumulator|

## Registers

| Register   | Offset      |Description|
|------------|-------------|-----------|
|`IP`        |`0x00`       | Instruction pointer |
|`Acc`       |`0x01`       | Accumulator |
|`R1`        |`0x02`       | General Purpose Register 1 |
|`R2`        |`0x03`       | General Purpose Register 2 |
|`R3`        |`0x04`       | General Purpose Register 3 |
|`R4`        |`0x05`       | General Purpose Register 4 |
|`R5`        |`0x06`       | General Purpose Register 5 |
|`R6`        |`0x07`       | General Purpose Register 6 |
|`R7`        |`0x08`       | General Purpose Register 7 |
|`R8`        |`0x09`       | General Purpose Register 8 |
