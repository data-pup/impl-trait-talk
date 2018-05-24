# impl Trait and Why It Is Neat

## Introduction

This talk assumes some previous familiarity with Rust, we unfortunately will
not have time to cover some of the foundational concepts in play such as
trait objects, but I will try and make this easy to follow even if you are
not an expert on Rust. (I'm sure not!)

The tracking issue for the work on this feature is [here][1]. This contains
discussion between the language maintainers, and was first opened in June 2016.

The details of `impl Trait` are divided into a few different RFC's. If you
would like to browse Rust language RFC's you can find them [here][2]. It is a
nice look into the process of how substantial changes are made to a language,
tooling, etc., when the discussion required outgrows the typical pull request
workflow offered by GitHub.

The specific RFC related to this feature are:
*  RFC 1522 - The original RFC, only covered `impl Trait` in the return
              position for inherent (free-standing) functions.
*  RFC 1951 - Syntax design, added `impl Trait` to the argument position.
*  RFC 2071 - Use of `impl Trait` in `let`, `const`, and `static`
*  RFC 2250 - Finalizing syntax of `impl Trait`

Detailed motivation on the original RFC proposal (1522) can be found [here][3].

The Rust v1.26 announcement including information about `impl Trait` can be
found [here][4].

[1]: https://github.com/rust-lang/rust/issues/34511
[2]: https://github.com/rust-lang/rfcs
[3]: http://aturon.github.io/blog/2015/09/28/impl-trait/
[4]: https://blog.rust-lang.org/2018/05/10/Rust-1.26.html

## Brief Aside

Before we delve into `impl Trait`, I want to pass a helpful trick along that
I wish I knew earlier on while learning Rust.

`rustup` is the tool used to install and manage Rust toolchains, and it will
also help you navigate documentation for Rust! If you'd like to read the
Rust Programming Language book yourself, you can use this command:

```
rustup doc --book
```

Documentation for the standard library can also be found with this command:

```
rustup doc --std
```

## What is `impl Trait`?

The ‘impl Trait’ feature provides the idea of “existential types,” which sounds
complex but this makes more sense when it is demonstrated. Essentially, it means
that we might not know the specific type of an object, but we do know a trait
that it implements.

If you are familiar with interfaces in C#, Go, or TypeScript, this might look
familiar when you see an example. We will briefly cover some of the use cases
for this, and then move on to a quick demo!

## When is `impl Trait` useful?

Previously, we could work around the lack of this feature by using Boxes. This
is essentially Rust’s term for a pointer. The downside to this however, is that
it requires allocating space on the heap, which is much slower than using the
stack.

Aside from performance benefits, this is useful when we are working with code
that uses iterator adaptors such as ‘map’ or ‘filter’, especially when the
adaptors might change for different conditions.

‘impl Trait’ also makes it much easier to return closures!

### Demos

Demos at this point. See code examples.

