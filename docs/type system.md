# The Tilang type system

**Disclaimer**: This is **not implemented** yet.

## Primitive types

There are a following primitive types in Tilang: `Bool`, `Char`, `Int`, `Float`, `Range`, `Array`, `Set` and `Map`.

Primitive types are the types that are **always there, even if you don't use the standard library**.

`Bool` is `true` or `false`.

`Char` is a **character** or, to be more precise, a **UTF8 code point**.
I might add different encodings like UTF16 in the future and `AsciiChar` is part of the standard library.

`Int` is an **integer**.

`Float` is a **number that you can represent in base 2**, 10 or 16.

`Range` is a **left-inclusive** and **right-exclusive** range between two `Int`egers.

### Data structures

`Array` is an **ordered list** of values of some type. You can set **upper and lower boundaries for its length**.

`Set` is an **unordered** `Array`. That means that there is no distinction between equal elements, so **all elements must be different**.

`Map` is a `Set` of **key and value** pairs. Only the **keys are relevant for equality**.

## Algebraic data types

You can express that something is **either** of type `A` **or** of type `B` by writing `A | B`.

You can express that something is **both** of type `A` **and** of type `B` by writing `A & B`.
