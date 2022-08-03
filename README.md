# Entrait dependency inversion POC

[Entrait](https://github.com/audunhalland/entrait) is not complete, because currently it's lacking in what kind of flexible architectures it can express.

It currently lacks proper dependency inversion on the `Impl<T>`-level.

The `Impl<T>`-level means the abstraction level where you 1. don't know whether the app is real or fake and 2. don't know the type of the app.
Dependencies fully at the `Impl<T>`-level are _internal dependencies_.
Dependencies that necessarily delegate out of `Impl<T>` (e.g. to `T`) are _leaf dependencies_.

_Dependency inversion_ means defining a trait upstream which is implemented downstream.
This is currently only possible by exiting the `Impl<T>`-layer.

This repository is a POC that uses safe type-magic to enable dependency inversion and keep the desired abstraction level at the same time.

For enabling dispatch of inverted dependencies, we have to implement the trait twice, for two different types.
The implementation for `Impl<T>` is the "inbound" one, and needs to delegate to another implementation.
For "normal" entraited functions, it just delegates by calling those functions, that's easy.
For delegating to another trait implementation (of the same trait), we temporarily have to use
  a different `Self`-type while delegating to the inner one.
This type needs to be _unique_ for the selected implementation.
It has to be a smart-pointer-like type that borrows the `Impl<T>`, so that inner type can be extracted again at the other side of the call, and injected further into the business logic.

Implementing static dispatch of a dependency inversion requires the `T` to implement an associated type that points to the correct implementation.

Implementing dynamic dispatch will only require implementing a method that returns `dyn Trait`.
But we need to generate a _copy_ of the trait, with an extra `&Impl<T>`-receiver, to use as delegation target.

## 🧅 Architecture
This project models a so-called "onion" architecture by using a crate graph.
This ensures that coherence (a.k.a. Orphan) rules are not broken.

The inner layer is inside `domain/`, and defines some traits which get delegated to an unknown receiver.
The `intermediate/` layer is supposed to provide those delegated-to implementations.
The main crate in `src/` defines the application and links things together.
