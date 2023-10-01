# README

* useage : `cargo build -r` : 'targets/release'
* lmc `emulate <file.bin>`
* `lmc assemble <file.lmasc> <outfile.bin>`
* `lmc run <file.lmasc> // assemble and run`

(alternatively run with `cargo run <args>`)

```
=> each instruction is 3 bytes (1 byte opcode, and 2 bytes operannd)

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
* [ ] 1100   DAT      <int>
* [ ] 1101   CALL      <int>
* [ ] 1110   RET      <int>

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
## Compiler (In Progress)

`lmc compile <infile.lmc> <outfile.lmasc>`

Compiles `.lmc` source code into `.lmasc` assembly.

```rust
fn mul(a, b) {
    let result;
    for (let i = 0; i < b; i = i + 1) {
        result = result + a;
    }
    return result;
}

fn main() {
    let a = input();
    let b = input();
    let result = mul(a, b);
    print(result);
}
```
