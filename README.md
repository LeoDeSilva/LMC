# README

```
=> each instruction is 3 bytes (1 byte opcode, and 2 bytes operannd)
        => only 4 bits of operand in use

opcode   operand
xxxxxxxx yyyyyyyyyyyyyyyy
8 bit    16 bit

      OPCODE MNEMONIC OPERAND
* [ ] 0000   HLT      <none>
* [ ] 0001   ADD      <addr>
* [ ] 0010   SUB      <addr>
* [ ] 0011   LDA      <addr>
* [ ] 0100   STA      <none>
* [ ] 0101   BRA      <addr>
* [ ] 0110   BRZ      <addr>
* [ ] 1011   BLT      <addr>
* [ ] 0111   BGT      <addr>
* [ ] 1000   IO       <op>
* [ ] 1001   OTC      <none>
* [ ]        DAT      <int>

lda A
sta CHAR
_start  lda CHAR
        otc
        add ONE
        sta CHAR
        sub Z
        blt _start
        
A     dat 65
Z     dat 91
ONE   dat 1
CHAR  dat
```