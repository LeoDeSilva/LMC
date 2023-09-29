# Compiler

Will include a 'standard library containing functions e.g. multiplication" linked into assembly and will use variables as parameters. **Must implement call and ret functions**

## Instruction Mapping

* Declaration: `let a; let a = 10; let a = 10 + 2`
```
a   dat 0

a   dat 0
    lda _10
    sta a

a dat _10

a   dat 0
    lda _10
    add _2
    sta a

_10 dat 10
_2  dat 2
```

Declaration code in the form: <declaration>, <expression - with result stored in acc>, <store - acc into variable>

* Assignments `a = 10; a = input(); a = b+2;`
```
    lda _10
    sta a

    inp
    sta a 

    lda b
    add _2
    sta a
```

* Conditionals 
```
if (a > b) {
    print(a);
} elif (a == b) {
    print(0);
} else {
    print(b);
}

    lda a
    sub b
    bgt l1

    lda a
    sub b
    brz l2

    bra l3
l1
    lda a
    out
    bra l4
l2
    lda _0
    out
    bra l4
l3
    lda b
    out
    bra l4
l4
    hlt
```

* Iteration
```
let a = 0;
while (a < b) {
    print(a);
    a = a + 1;
}

a   dat 0
l1  // condition 
    lda a
    sub b
    blt l2 // execute
    bra l3 // else exit loop
l2

    lda a
    out

    lda a
    add _1
    sta a

    bra l1
l3
    hlt

_1  dat 1

for (let i = 0; i < 10; i = i + 1) {
    print(i);
}

i  dat 0
l1 // condition
    lda i
    sub _10
    blt l2
    bra l3
l2 // expression
    lda i 
    out

    // increment
    lda i
    add 1
    sta i
    bra l1
l3 // end
    hlt
```


## Syntax
```
fn main() {
        let score = input();
        let grade = 0;

        if score > 75 {
                grade = 'A';
        } elif score > 50 {
                grade = 'B';
        } else {
                grade = 'C';
        }

        print(grade);
}

_main
score   dat 0 
        inp
        sta score

grade   dat 0
        lda ZERO
        sta score

        lda score
        sub SVTYF
        bgt then

        lda score
        sub FIFTY
        bgt elif

        bra else

then    lda A
        sta grade
        bra endif

elif    lda B 
        sta grade
        bra endif

else    lda C
        sta grade
        bra endif

endif   lda grade
        out

ZERO    dat 0
SVTYF   dat 75
FIFTY   dat 50

A       dat 65
B       dat 66
C       dat 67
```

## Basic Arithmatic
```
let a = 10;
let b = 5;
let c = a + b;
print(c);

a = 2;

a:      dat 0
        lda TEN
        sta a

b:      dat 0
        lda FIVE
        sta b

c:      dat 0
        lda a
        add b
        sta c

        lda c
        out

        lda TWO
        sta a

TEN     dat 10
FIVE    dat 5
TWO     dat 2
```

## Conditionals

```
let a = input();
let b = input();
let c;

if (a > b) {
        c = a;
} elif (a == b) {
        c = 0;
} else {
        c = b;
}
print(c);

a       dat 0
        inp
        sta a

b       dat 0
        inp
        sta b

c       dat 0

        lda a 
        sub b
        bgt if

        lda a
        sub b
        brz elif

        bra else

if      lda a
        sta c
        bra endif

elif    lda ZERO 
        sta c
        bra endif

else    lda b
        sta c
        bra endif

endif   lda c
        out

ZERO    dat 0
```