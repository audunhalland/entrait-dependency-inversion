# Entrait dependency inversion POC

[Entrait](https://github.com/audunhalland/entrait) is not complete, because currently it's lacking in what kind of flexible architectures it can express.

It currently lacks proper dependency inversion on the `Impl<T>`-level.

The `Impl<T>`-level means the abstraction level where you 1. don't know whether the app is real or fake and 2. don't know the type of the app.
Dependencies fully at the `Impl<T>`-level are _internal dependencies_.
Dependencies that necessarily delegate out of `Impl<T>` (e.g. to `T`) are _leaf dependencies_.

_Dependency inversion_ means defining a trait upstream which is implemented downstream.
This is currently only possible by exiting the `Impl<T>`-layer.

This repository is a POC that uses safe type-magic to enable dependency inversion and keep the desired abstraction level at the same time.

For enabling dispatch of dependency inversion, we always have to implement the trait twice for two different types.
The implementation for `Impl<T>` is the "inbound" one, and needs to delegate to another implementation.
This means we need to temporarily use a different `Self` type, only while dispatching the call.

Implementing static dispatch of a dependency inversion requires the `T` to implement an associated type that points to the correct implementation.

Implementing dynamic dispatch will only require implementing a method that returns `dyn Trait`.
But we need to generate a _copy_ of the trait, with an extra `&Impl<T>`-receiver, to use as delegation target.
