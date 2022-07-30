# Crlox

CrLox is a rust implementation of Lox programming language described in Bob Nystrom's book: [Crafting Interpreters](http://craftinginterpreters.com). The bytecode virtual machine is implemented by rust features to make it fast, lightweight and modular. 

## Lox language
Lox is a programming language described in Bob Nystrom's book: [Crafting Interpreters](http://craftinginterpreters.com). The syntax is similiar to C, with dynamic types and garbage collector. Folowing is a example to calcute the sum of 10000000.

```
var sum = 0.0;
var i = 0;
while (i < 10000000) {
    sum = sum + i;
    i = i + 1;
}
print sum;
```

For more details on Lox's syntax, check out the [description](http://craftinginterpreters.com/the-lox-language.html)
in Bob's book.

