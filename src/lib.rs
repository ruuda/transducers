// Transducers -- Transducer library for Rust
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
// Then he goes on about a state machine being a valid state, but a sum type is
// wrong, because if X goes in, it is not X | Y | Z that comes out, it is
// _always_ Y.
//
// I think this might need something like associated types? It can become quite
// hairy to do it correctly, I think. For now, it is just R -> R. It is wrong.
// I know.

#![feature(unboxed_closures)]

trait Transducer<'t, R, T, U> {
    type Step: Fn(R, U) -> R + 't;

    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> Self::Step;
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
fn it_works() {
    let m = mapping(|x: i32| x * 2);
}
