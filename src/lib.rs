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

#![feature(unboxed_closures)]

pub trait Transducer<'t, R, T, U> {
    type Step: Fn(R, U) -> R + 't;
    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> Self::Step;
}

// To create a Transduce trait, I think higher-ranked types would be required.
pub fn transduce<'t, T, U, I: Iterator<Item = U>,
                 Trans: Transducer<'t, Vec<T>, T, U> + 't>
                 (mut iter: I, trans: Trans)
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


pub struct MappingStep<'t, R, T, F: 't> {
    step: Box<Fn(R, T) -> R + 't>,
    f: &'t F
}

impl<'t, R, T, U, F> Fn(R, U) -> R for MappingStep<'t, R, T, F>
    where F: Fn(U) -> T + 't {
    extern "rust-call" fn call(&self, args: (R, U)) -> R {
        let (r, u) = args;
        (*self.step)(r, (self.f)(u))
    }
}

pub struct Mapping<'t, F: 't> {
    f: &'t F
}

impl<'t, R: 't, T, U, F> Transducer<'t, R, T, U> for Mapping<'t, F>
where F: Fn(U) -> T + 't {
    type Step = MappingStep<'t, R, T, F>;

    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> MappingStep<'t, R, T, F> {
       MappingStep {
           step: Box::new(step),
           f: self.f
       }
    }
}

pub fn mapping<'f, R, S, F: Fn(S) -> R + 'f>(f: &'f F) -> Mapping<'f, F> {
    Mapping { f: f }
}

#[test]
fn mapping_on_iter() {
    let f = |&: x: &i32| *x * 2;
    let g = |&: x: i32| x * 2;
    let m = mapping(&f);
    let n = mapping(&g);
    let v = vec!(2i32, 3, 5, 7, 11);
    let w = transduce(v.iter(), m);
    let x = transduce(v.into_iter(), n);
    assert_eq!(w, vec!(4i32, 6, 10, 14, 22));
    assert_eq!(w, x);
}
