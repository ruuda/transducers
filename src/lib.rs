// Transducers -- Transducer library for Rust
// Copyright (C) 2014 Ruud van Asseldonk
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

#![feature(overloaded_calls)]
#![feature(closure_sugar)]

trait Transduce<R, T, U> {
    fn transduce<'t, Trans, LStep, RStep>(self, trans: Trans) -> R
        where Trans: Sized + Fn<(LStep,), RStep> + 't,
              LStep: Sized + Fn<(R, U), R> + 't,
              RStep: Sized + Fn<(R, T), R> + 't;
}

struct Step<'s, R, T> {
    step_fn: |R, T|: 's -> R
}

impl<'s, R, T> Fn<(R, T), R> for Step<'s, R, T> {
    pub fn call(&self, args: (R, T)) -> R {
        let (r, t) = args;
        (self.step_fn)(r, t)
    }
}

// TODO: this is not yet fully generic over R.
impl<T, U> Transduce<Vec<U>, T, U> for Vec<T> {
    fn transduce<'t, Trans, LStep, RStep>(self, trans: Trans) -> Vec<U>
        where Trans: Sized + Fn<(LStep,), RStep> + 't,
              LStep: Sized + Fn<(Vec<U>, U), Vec<U>> + 't,
              RStep: Sized + Fn<(Vec<U>, T), Vec<U>> + 't {
        let mut iter = self.into_iter();
        let step = Step { step_fn: |rr: Vec<U>, x| { rr.push(x); rr } };
        let transduced_step = trans(step);
        let mut r = Vec::new();
        loop {
            match iter.next() {
                Some(x) => r = transduced_step(r, x),
                None => break
            }
        }
        r
    }
}

struct Mapping<'f, T, U> {
    f: |T|: 'f -> U
}

impl<'t, R, T, U, LStep, RStep> Fn<(LStep,), RStep> for Mapping<'t, T, U>
    where LStep: Sized + Fn<(R, U), R> + 't,
          RStep: Sized + Fn<(R, T), R> + 't {
    pub fn call(&self, args: (LStep,)) -> RStep {
        let (step,) = args;
        |r, x| step(r, (self.f)(x))
    }
}

fn mapping<'f, T, U>(f: |T|: 'f -> U) -> Mapping<'f, T, U>{
    Mapping { f: f }
}

#[test]
fn it_works() {
}
