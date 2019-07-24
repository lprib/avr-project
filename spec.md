## Opcodes
### loading/storing
- `push <value>`
- `pushref <address>`
- `store <address>` pop top stack value into address
- `swap` swap top stack values
- `fetch <n>` swap the nth value from the top of the stack with the value at the top of the stack
- `pushoffset <address> <offset>` push the value \<offset\> bytes after \<address\>
- `storeoffset <address> <offset>` store the top stack value to the location \<offset\> bytes after \<address\>
- 

### Math ops
Self explanatory. Do operations on the top 1 or 2 stack values.

### Jump ops
General form `jumpxx <address>`. Jump to \<address\> if the top 1 or 2 stack values satisfy the condition xx.

## IO
- `syscall <number>` executes the specified syscall with args on the stack.