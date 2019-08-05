# The Tilang type system

**Disclaimer**: This is **not implemented** yet.

In this document, the term value doesn't refer to the `val` specifier but to nonnullable state.

## Primitive types

There are a following primitive types in Tilang: `Bool`, `Char`, `Int`, `Float`, `Range`, `Tuple`, `Array`, `Set` and `Map`.

Primitive types are the types that are **always there, even if you don't use the standard library**.

A `Bool` is `true` or `false`.

A `Char` is a **character** or, to be more precise, a **UTF8 code point**.
I might add different encodings like UTF16 in the future and `AsciiChar` is part of the standard library.

An `Int` is an **integer**.

A `Float` is a **number that you can represent in base 2**, 10 or 16.

A `Range` is a **left-inclusive** and **right-exclusive** range between two `Int`egers.

A `Tuple` is an **ordered list of values with individual types with a fixed length**.

### Data structures

An `Array` is an **ordered list** of values of some common type. You can set **upper and lower boundaries for its length**.

A `Set` is an **unordered** `Array`. That means that there is no distinction between equal elements, so **all elements must be different**.

A `Map` is a `Set` of **key and value** pairs. Only the **keys are relevant for equality**.

## Algebraic data types

You can express that something is **either** of type `A` **or** of type `B` by writing `A | B`.

You can express that something is **both** of type `A` **and** of type `B` by writing `A & B`.

## Object types

Object types are **sets of named types or named values that can both be unspecified** and they are **named themselves**.

Only values of object types must be fully specified, not the object types themselves.

## Newtypes

If you want a type `A` that **can be used just as another type** `B` but is **distinct**, you can write `<specifier/s> A = new B`.
You can **cast** a value `x` of type `A` **back** to `B` **explicitly** by writing `x as B`.

## Functions

## IO and global variables

## Values as types

Values can be used as types that only have one possible state, which is the value.

Only types with multiple states must be annotated with the `type` specifier because you can't use them as values.

## Subtype relationships

### `Anything`, the supertype of all types

## Mutable run-time types
