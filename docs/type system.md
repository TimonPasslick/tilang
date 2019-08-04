# The Tilang type system

**Disclaimer**: This is **not implemented** yet.

## Primitive types

There are a following primitive types in Tilang: `Bool`, `Char`, `Int`, `Float` and `Range`.

Primitive types are the types that are **always there, even if you don't use the standard library**.

`Bool` represents `true` or `false`.

`Char` represents a **character** or, to be more precise, a **UTF8 code point**.
I might add different encodings like UTF16 in the future and `AsciiChar` is part of the standard library.

`Int` represents an **integer**.

`Float` represents a **number** that you can write with a **finite amount** of decimal, hexadecimal or binary **digits**, for example 3.14 or 42.

`Range` represents a **left-inclusive** and **right-exclusive** range between two `Int`egers.

## Algebraic data types

### Sum types

You can express that something is **either** of type `A` **or** of type `B` by writing `A | B`.

### Product types

You can express that something is **both** of type `A` **and** of type `B` by writing `A & B`.
