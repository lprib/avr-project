#ifndef OPCODES_H
#define OPCODES_H

#define OP_PUSH 0x00
#define OP_PUSHREF 0x01
#define OP_STORE 0x02
#define OP_SWAP 0x03
#define OP_FETCH 0x04
#define OP_PUSHOFFSET 0x05
#define OP_STOREOFFSET 0x06
#define OP_DEREF 0x07
#define OP_FETCHLOCAL 0x08
#define OP_FETCHARG 0x09
#define OP_ADD 0x20
#define OP_SUB 0x21
#define OP_MUL 0x22
#define OP_DIV 0x23
#define OP_MOD 0x24
#define OP_INC 0x25
#define OP_DEC 0x26
#define OP_JUMP 0x40
#define OP_JUMPZ 0x41
#define OP_JUMPNZ 0x42
#define OP_JUMPLT 0x43
#define OP_JUMPGT 0x44
#define OP_JUMPLE 0x45
#define OP_JUMPGE 0x46
#define OP_CALL 0x47
#define OP_RETURN 0x48
#define OP_SYSCALL 0x50

#endif