# documentation

## 0x00 op codes

`00 00 00 00` nop: increase pc by 1

`00 00 00 01` hlt: stop program execution 

`00 00 00 02 00 00 00 AA XX XX XX XX` ldr: load value X into register A

`00 00 00 03 00 00 BB AA` mov: load value in reg A into reg B

## 0x10 op codes

`10 00 00 01 00 CC BB AA` add: reg A + reg B = store in reg C

`10 00 00 02 00 CC BB AA` sub: reg A - reg B = store in reg cC


## 0x11 op codes


`11 00 00 01 00 CC 00 AA XX XX XX XX` - add: adds constant and reg A and stores in reg C NOT YET IMPLEMENTED


## 0x20 op codes


`20 00 00 01 00 00 00 AA` - jmp: unconditional jmp to reg A

`20 00 00 02 00 00 00 AA` - jez: jmp if zero flag set to reg A

`20 00 00 03 00 00 00 AA` - jnz: jmp if zero flag not set to reg A


## 0x30 op codes


`30 00 00 01 00 00 00 AA` - pnt: print value in reg A as ascii value

## 0xF0 op codes

`F0 00 00 01 00 00 BB AA` - cmp: A - B, set zero flag to 1 if zero, discard result

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

