// Transducers -- A transducer library for Rust
// Copyright (C) 2014-2015 Ruud van Asseldonk
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

// TODO: Rich Hickey says: “If you’re trying to produce the next process N, you
// _must_ supply the result of step N-1 as the input. If you’re trying to model
// this in your type system saying R -> R, that’s _wrong_. Right? Because I can
// call the step function five times, and then on the sixth time, take the
// return value from the first time, and pass it as the first thing. That’s
// wrong. So do you know how to make your type system make that wrong?”
//
// [Yes Mr Hickey, I do know how to make that wrong. Take R by value in the
// step function. If you then put its result into the step function again, then
// it has moved there, and you cannot return it any more. If R is not Copy, of
// course.]
//
// Then he goes on about a state machine being a valid state, but a sum type is
// wrong, because if X goes in, it is not X | Y | Z that comes out, it is
// _always_ Y.
//
// I think this might need something like associated types? It can become quite
// hairy to do it correctly, I think. For now, it is just R -> R. It is wrong.
// I know.

//! Transducers, a transducer library for Rust.
//!
//! TODO: Add some examples here.

#![warn(missing_docs)]
#![feature(unboxed_closures, core)]

pub use compose::{compose, compose_trans};
pub use transform::{Filtering, Identity, Mapping};

mod compose;
mod transform;

/// An abstract tranformation/reduction of data.
///
/// A transducer represents a transformation or reduction like `map`, `filter`
/// or `fold`. It specifies how to manipulate the data, independent of the way
/// in which that data might arrive.
///
/// While the trait (and especially its implementations) may look scary at
/// first, you rarely need to deal with this trait directly unless you intend
/// to implement your own transformation/reduction functions that cannot be
/// expressed as a composition of standard transducers.
pub trait Transducer<'t, R, T, U> {

    /// The type of step that application of the transducer produces.
    type Step: Fn(R, U) -> R + 't;

    /// Applies the transducer to the step function to obtain a new step function.
    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> Self::Step;
}

// NOTE: To create a Transduce trait, I think higher-ranked types would be required.
// TODO: Transduce into an interator, do not collect immediately.
// TODO: We use the size hint of the iterator, but even the min_sz could be an
//       overestimation due to a filtering transducer.
/// Transduces the iterator `iter` with the transducer `trans`.
///
/// This is an alternative to the `IteratorExt` methods in the standard library.
///
/// ```
/// # use transducers::{transduce, Mapping};
/// let v = vec!(2i32, 3, 5, 7, 11);
/// let f = |&x| x * 2;
/// let v_trans = transduce(&mut v.iter(), Mapping::new(&f));
/// let v_map: Vec<i32> = v.iter().map(f).collect();
/// assert_eq!(v_trans, v_map);
/// ```
pub fn transduce<'t, 'i, T, U, I: Iterator<Item = U>,
                 Step: Fn(Vec<T>, U) -> Vec<T> + 't,
                 Trans: Transducer<'t, Vec<T>, T, U, Step = Step> + 't>
                 (iter: &'i mut I, trans: Trans)
                 -> Vec<T> where Trans::Step: 't {
    // The step function for a vector is simply append.
    fn append<TT>(mut r: Vec<TT>, t: TT) -> Vec<TT> { r.push(t); r }

    // Then we transduce the step function into the desired form.
    let step = trans.apply(append);

    // The result is obtained by performing a left fold of the step function.
    let (min_sz, _) = iter.size_hint();
    let mut state = Vec::with_capacity(min_sz);
    for t in iter {
        state = step(state, t);
    }
    state
}

#[test]
fn identity_is_identity_on_iter() {
    let v = vec!(2i32, 3, 5, 7, 11);
    let w = transduce(&mut v.clone().into_iter(), Identity::new());
    assert_eq!(v, w);
}

#[test]
fn mapping_on_iter() {
    let u = vec!(2i32, 3, 5, 7, 11);
    let v = u.clone();
    let f = |x: &i32| *x * 2;
    let g = |x: i32| x * 2;
    let m = Mapping::new(&f);
    let n = Mapping::new(&g);
    let w = transduce(&mut u.iter(), m);
    let x = transduce(&mut v.into_iter(), n);
    assert_eq!(w, vec!(4i32, 6, 10, 14, 22));
    assert_eq!(w, x);
}

#[test]
fn filtering_on_iter() {
    let p = |x: &i32| *x % 2 == 0;
    let q = |x: &i32| *x % 3 != 0;
    let f = Filtering::new(&p);
    let h = Filtering::new(&q);
    let v = vec!(2i32, 3, 5, 6, 7, 11);
    let w = transduce(&mut v.iter().cloned(), f);
    let x = transduce(&mut v.iter().cloned(), h);
    assert_eq!(w, vec!(2i32, 6));
    assert_eq!(x, vec!(2i32, 5, 7, 11));
}

#[test]
fn compose_mapping_filtering() {
    let f = |x: i32| x * 2;
    let p = |x: &i32| *x % 4 != 0;
    let t = compose_trans(Mapping::new(&f), Filtering::new(&p));
    let v = vec!(2i32, 3, 4, 5, 6, 7, 11);
    let w = transduce(&mut v.into_iter(), t);
    assert_eq!(w, vec!(6i32, 10, 14, 22));
}
