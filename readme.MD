# documentation


`00 00 00 00` nop: increase pc by 1

`00 00 00 01` hlt: stop program execution 

`00 00 00 02 00 00 00 AA XX XX XX XX` ldr: load value X into register A


`10 00 00 01 00 CC BB AA` add: reg A + reg B = store in reg C

`10 00 00 02 00 CC BB AA` sub: reg A - reg B = store in reg cC

`11 00 00 01 00 CC 00 AA XX XX XX XX` - add: adds constant and reg A and stores in reg C



`20 00 00 01 00 00 00 AA` - jmp: unconditional jmp to reg A

`20 00 00 02 00 00 00 AA` - jez: jmp if zero flag set to reg A

`20 00 00 03 00 00 00 AA` - jnz: jmp if zero flag not set to reg A

`30 00 00 01 00 00 00 AA` - pnt: print value in reg A as ascii value

`F0 00 00 01 00 00 BB AA` - cmp: A - B, set zero flag to 1 if zero, discard result



