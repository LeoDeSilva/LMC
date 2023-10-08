# README
[Documentation](https://leo-de-silva.gitbook.io/lmc/)\
[LMC Instruction Set](https://peterhigginson.co.uk/lmc/help.html)
[LMC Wikipedia](https://en.wikipedia.org/wiki/Little_man_computer)

* useage : `cargo build -r` : lmc binary located in `targets/release`
* `lmc emulate <infile.bin>`
* `lmc assemble <infile.lmasc> <outfile.bin>`
* `lmc run <infile.lmasc> // assemble and run`
* `lmc compile <infile.lmc> <outfile.lmasc>`

(alternatively run with `cargo run <args>`)

```
=> each instruction is 3 bytes (1 byte opcode, and 2 bytes operand)

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
## Compiler 

`lmc compile <infile.lmc> <outfile.lmasc>`

Compiles `.lmc` source code into `.lmasc` assembly.

```rust
use std;

fn get_grade(score) {
    let grade;
    
    if score >= 75 {
        grade = 65;
    } elif score >= 50 {
        grade = 66;
    } else {
        grade = 67;
    }

    return grade;
}

fn _main() {
    let score = input(); 
    printc(get_grade(score));
    return 0;
}
```

will compile to

```x86
call _main
hlt

print       lda _p0
            out
            ret

printc      lda _p0
            otc
            ret

input       inp
            sta _ret
            ret
get_grade
      score dat 0
      lda _p0
      sta score

      grade dat 0
      lda _0
      sta grade

      lda score
      SUB _75
      bgt _l1
      brz _l1

      lda score
      SUB _50
      bgt _l2
      brz _l2

      lda _67
      sta grade
      bra _l0

      _l1
            lda _65
            sta grade
            bra _l0
      _l2
            lda _66
            sta grade
            bra _l0
      _l0

      _ret dat 0
      lda grade
      sta _ret

      ret

_main
      score dat 0
      call input
      lda _ret
      sta score

      _p0 dat 0
      lda score
      sta _p0

      call get_grade
      lda _ret
      sta _p0

      call printc
      lda _ret

      _ret dat 0
      lda _0
      sta _ret
      ret

_66 dat 66
_67 dat 67
_50 dat 50
_0 dat 0
_75 dat 75
_65 dat 65
```

```rust
use std;

fn multiply(a, b) {
    let result;
    while b > 0 {
        result = result + a;
        b = b - 1;
    }

    return result;
}

fn _main() {
    let a = input();
    let b = input();
    print(multiply(a, b));
}
```
