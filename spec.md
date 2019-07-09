bytecode file:
- number of functions **2 bytes**
- function table **2 bytes per function**
  - each entry is the byte index of that function in the bytecode
- for each function:
  - local allocation size declaration **2 bytes**
  - bytecode **n bytes**