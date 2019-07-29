# The Tilang type system

Disclaimer: This isn't implemented yet.

## Primitive types

There are three primitives in Tilang: `Bool`, `Char` and `Num`.

`Bool` represents `true` or `false`.

`Char` represents a character or, to be more precise, a UTF8 code point.
I might add different encodings like UTF16 in the future and `AsciiChar` is part of the standard library.

`Num` represents a real number.
This means that it doesn't represent a float or double. A `Num` is infinitely precise (theoretically, RAM is limited).
Using `Num`, other numeric types are constructed in the standard library. These range from `Nat` or `Complex` to faster types that correspond to wasm primitives like `i32` and `f64`.
