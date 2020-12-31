# Simple VM
![Build](https://github.com/renjithgr/simple-vm/workflows/Build/badge.svg)

A simple bytecode virtual machine based on the [LLJSVM](https://github.com/LowLevelJavaScript/16-Bit-Virtual-Machine)

## Test & Build

```sh
cargo test
cargo build
```

## VM Instructions

| Instruction   | Arguments | Description |
|---------------|-----------|-------------|
| `MOV_LIT_R1`  | `LIT`     | Move 16 bit literal to the register R1 |
| `MOV_LIT_R2`  | `LIT`     | Move 16 bit literal to the register R2 |
| `ADD_REG_REG` | `R1, R2`  | Add values of registers provided and place the result in the accumulator|
