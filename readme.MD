# documentation

## System Overview
* 12 general registers, 1-12, and 3 special purpose registers LK (link), SP (Stack Pointer), and PC (Program Counter)

* stack is empty ascending

* 1 instruction processed per cycle

* Jumps are aligned to 4 byte words, not bytes, same with PC

* RAM is 1,000,000 words (0 - 999,999). Stack starts at 0

## 0x00 op codes

`nop: 00 00 00 00` - increase pc by 1

`hlt: 00 00 00 01` - stop program execution 

`ldr: 00 00 00 02 00 00 00 AA XX XX XX XX` - load value X into register A

`mov: 00 00 00 03 00 00 BB AA` - load value in reg A into reg B

`psh: 00 00 00 03 00 00 XX XX` - push registers onto stack, where XX XX is bitflag of registers (2^0 = r1... 2^11 = r12, 2^12 = LK, 2^13 = SP, 2^14 = PC)

`pop: 00 00 00 03 00 00 XX XX` - pop registers from stack, where XX XX is bitflag of registers (2^0 = r1... 2^11 = r12, 2^12 = LK, 2^13 = SP, 2^14 = PC)

## 0x10 op codes

`add: 10 00 00 01 00 CC BB AA` - reg A + reg B = store in reg C

`sub: 10 00 00 02 00 CC BB AA` - reg A - reg B = store in reg C

`mul: 10 00 00 03 00 CC BB AA` - reg A * reg B = store in reg C

`div: 10 00 00 04 DD CC BB AA` - reg A // reg B = store in reg C, remainder in reg D

## 0x11 op codes


`???: 11 00 00 01 00 CC 00 AA XX XX XX XX` - adds constant and reg A and stores in reg C NOT YET IMPLEMENTED


## 0x20 op codes

jumps offset to the number of words

`jmp: 20 00 00 01 00 00 00 AA` - unconditional jmp to reg A

`jez: 20 00 00 02 00 00 00 AA` - jmp if zero flag set to reg A

`jnz: 20 00 00 03 00 00 00 AA` - jmp if zero flag not set to reg A

`blr: 20 00 10 01 00 00 00 AA` - unconditional jmp and link to reg A

`bez: 20 00 10 02 00 00 00 AA` - jmp and link if zero flag set to reg A

`bnz: 20 00 10 03 00 00 00 AA` - jmp and link if zero flag not set to reg A

`jpc: 20 10 00 01 AA AA AA AA` - unconditional jmp to A

`jec: 20 10 00 02 AA AA AA AA` - jmp if zero flag set to A

`jnc: 20 10 00 03 AA AA AA AA` - jmp if zero flag not set to A

`blc: 20 10 10 01 AA AA AA AA` - unconditional jmp and link to A

`bec: 20 10 10 02 AA AA AA AA` - jmp and link if zero flag set to A

`bnc: 20 10 10 03 AA AA AA AA` - jmp and link if zero flag not set to A

`lrt: 20 F0 00 01 AA AA AA AA` - return to adr in link register

## 0x30 op codes


`ptc: 30 00 00 01 00 00 00 AA` - print value in reg A as ascii value

`pnt: 30 00 00 02 AA AA AA AA` - print value A as dec number

`pth: 30 00 00 03 AA AA AA AA` - print value A as hex number

## 0xF0 op codes

`cmp: F0 00 00 01 00 00 BB AA` - A - B, set zero flag to 1 if zero, discard result

# Example

```
ldr A 65
ldr B 04
ldr C 0
ldr D 1
label hi
pnt A 
sub B D 
cmp B C 
jnz hi 
hlt 
```

would be translated to the bytecode 

```
00 00 00 02 00 00 00 01 00 00 00 41 

00 00 00 02 00 00 00 02 00 00 00 04 

00 00 00 02 00 00 00 03 00 00 00 00 

00 00 00 02 00 00 00 04 00 00 00 01 

30 00 00 01 00 00 00 01 

10 00 00 02 00 02 04 02 

F0 00 00 01 00 00 03 02 

20 00 00 03 00 00 00 0C 

00 00 00 01
```

