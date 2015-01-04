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
#![feature(associated_types)]
#![feature(old_orphan_check)]

trait Transducer<R, T, U, FromStep>
    where FromStep: Fn(R, U) -> R {
    type ToStep: Fn(R, T) -> R;
    // TODO: the full name of ToStep should not be necessary, right?
    // But the compiler complains without it ...
    fn call(&self, step: FromStep) -> <Self as Transducer<R, T, U, FromStep>>::ToStep;
}

struct MappingStep<Step, F> {
    step: Step,
    f: F
}

impl<R, T, U, Step, F> Fn(R, T) -> R for MappingStep<Step, F>
    where Step: Fn(R, U) -> R,
          F: Fn(T) -> U {
    extern "rust-call" fn call(&self, args: (R, T)) -> R {
        let (r, t) = args;
        (self.step)(r, (self.f)(t))
    }
}

struct Mapping<F> {
    f: F
}

impl<R, T, U, Step, F> Fn(Step) -> MappingStep<Step, F> for Mapping<F>
    where Step: Fn(R, U) -> R,
          F: Clone + Fn(T) -> U {
    extern "rust-call" fn call(&self, args: (Step,)) -> MappingStep<Step, F> {
        let (step,) = args;
        MappingStep { step: step, f: self.f.clone() }
    }
}

impl<R, T, U, FromStep, F> Transducer<R, T, U, FromStep> for Mapping<F>
    where FromStep: Fn(R, U) -> R,
          F: Clone + Fn(T) -> U {
    type ToStep = MappingStep<FromStep, F>;
    fn call(&self, step: FromStep) -> MappingStep<FromStep, F> {
        MappingStep { step: step, f: self.f.clone() } // TODO: struct field order consistency.
    }
}

fn mapping<T, U, F>(f: F) -> Mapping<F>
    where F: Fn(T) -> U {
    Mapping { f: f }
}

#[test]
fn it_works() {
    let m = mapping(|x: i32| x * 2);
}
