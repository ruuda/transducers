Transducers
===========

A transducer library for Rust.

[![Build Status][ci-img]][ci]
[![Crates.io version][crate-img]][crate]

[Transducers][transducers] are a way to decouple tranformation and reduction
operations from the procedure in which the data is provided. They allow the
implementation of common functions like `map` and `filter` to be reused for
any type that represents a succession of data, and they allow your reduction
functions to be decoupled from the way in which the data is provided, whether
that is a collection, an iterator, a channel, or an observable.

The library is licensed under the [GNU General Public License][gplv3] during
the alpha stage.

[ci-img]:      https://travis-ci.org/ruud-v-a/transducers.svg
[ci]:          https://travis-ci.org/ruud-v-a/transducers
[crate-img]:   http://img.shields.io/crates/v/transducers.svg
[crate]:       https://crates.io/crates/transducers
[transducers]: https://www.youtube.com/watch?v=6mTbuzafcII
[gplv3]:       https://www.gnu.org/licenses/gpl.html
