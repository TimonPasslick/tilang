# The Tilang type system

**Disclaimer**: This is **not implemented** yet.

In this document, the term value doesn't refer to the `val` specifier but to nonnullable state.

## Primitive types

There are a following primitive types in Tilang: `Bool`, `Char`, `Int`, `Float`, `Range`, `Tuple`, `Array`, `Set` and `Map`.

Primitive types are the types that are **always there, even if you don't use the standard library**.

A `Bool` is `true` or `false`.

A `Char` is a **character** or, to be more precise, a **Unicode slalar value**.

An `Int` is an **integer**.

A `Float` is a **number that you can represent in base 2**, 10 or 16.

A `Range` is a **left-inclusive** and **right-exclusive** range between two `Int`egers. It can have no upper bound.

A `Tuple` is an **ordered list of values with individual types with a fixed length**.

### Data structures

An `Array` is an **ordered list** of values of some common type. You can set **upper and lower boundaries for its length**.

A `Set` is an **unordered** `Array`. That means that there is no distinction between equal elements, so **all elements must be different**.

A `Map` is a `Set` of **key and value** pairs. Only the **keys are relevant for equality**.

## Algebraic data types

You can express that something is **either** of type `A` **or** of type `B` by writing `A | B`. A type like this is called **sum type**.

You can **remove an option** from a sum type by writing `(A | B) without B`. This also works with primitive types, `Int without 0` is possible, for example. A type like this is called **difference type**.

You can express that something is **both** of type `A` **and** of type `B` by writing `A & B`. A type like this is called **product type**.

## The unconstructible type

What's `A without A`? It's `!`, of course. ( ; `!` is the unconstructible type, which is fancy wording that says that **there is no value of this type**. The unconstructible type is **useful for expressing that a function never returns**.

By the way, that means that `A | !` is equal to `A` just as `A without !` and `A & !` is equal to `!`.

## Object types

Object types are **sets of named types or named values that can both be unspecified**. They can be **named themselves**.

Only values of object types must be fully specified, not the object types themselves.

Named object types are nominal types, so they are only equal if they have the same name.
Unnamed object types are structural types, so they are equal if the sets are equal.

## Newtypes

If you want a type `A` that **can be used just as another type** `B` but is **distinct**, you can write `<specifier/s> A = new B`.
You can **cast** a value `x` of type `A` **back** to `B` **explicitly** by writing `x as B`.

## Functions

A function **takes a `Tuple` as an argument** and returns `Anything` (see further down). The argument and return type are part of a function type but not the only possible parts as you will see in the next paragraph.

There are named and anonymous functions. Anonymous functions are all distinct because it's very hard to come up with an algorithm that proves that two functions have equivalent behavior.

## IO and static variables

There are imported and exported functions in wasm. Unfortunately, I don't know enough about wasm yet to design the types of these functions in Tilang. But I know that these functions are IO functions and that this is part of their type definitions. **Any function that calls an IO function is also an IO function.**

All variables can be static. A static variable is never deleted. **If a function reads or changes a static variable directly or indirectly, it's a state function.**

## Values as types

Values can be used as types that only have one possible state, which is the value.

Only types with multiple states must be annotated with the `type` specifier because you can't use them as values.

## Subtype relationships

If `A` is a subtype of `B`, we write `A: B`. You can read this as:

"Each `A` is a `B`."

"Each `A` can be used as a `B`."

"Each value of type `A` is implicitly convertible to type `B`."

The third interpretation is technically wrong in Tilang because each value of type `A` is both of type `A` and type `B`, so it doesn't need to be converted in the first place.

What exactly does it mean for a value `x` to be of type `A`? It simply means `x: A`.

### Fundamental laws

Here are the fundamental laws that are used to check for subtype relationships:

If `A: B` and `B: C`, then `A: C`.

`A: A`

`!: A`

`A: Anything` (`Anything` is in Tilang what `Object` is in Java.)

`A: A | B`

`A & B: A`

`Int: Float`

A `Tuple` is a subtype of another `Tuple` if it has the same length and each field type is a subtype of the corresponding field type.

An `Array` is a subtype of another array if its lower bound isn't lower than the other lower bound and its upper bound isn't higher than the other upper bound (The highest upper bound is no upper bound.) and if its element type is a subtype of the other element type.

The `Array` rule also applys for `Set`s and `Map`s.

A function is a subtype of another function if its argument and return types are subtypes of the other argument and return types and if it doesn't do IO or access static variables when the other function doesn't.

## Mutable run-time types
