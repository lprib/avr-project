## Opcodes
### loading/storing
- 0x00 `pushaddress address` pushes 16 bits at address to stack
- 0x01 `pushconst constant` pushes constant (immediate value) to stack
- 0x02 `popinto address` pops the top stack value and stores it at address
- 0x03 `store stack_index address` stores the value `stack_index` slots from the stack pointer into address
### jumps
- 0x
