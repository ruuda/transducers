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

type Step<'s, R, T> = proc(R, T): 's -> R;
type Transducer<'t, R, T, U> = proc(Step<'t, R, U>): 't -> Step<'t, R, T>;

trait Transduce<R, T, U> {
    fn transduce<'s>(self, trans: Transducer<'s, R, T, U>) -> R;
}

// TODO: this is not yet fully generic over R.
impl<T, U> Transduce<Vec<U>, T, U> for Vec<T> {
    fn transduce<'s>(self, trans: Transducer<'s, Vec<U>, T, U>) -> Vec<U> {
        let mut iter = self.move_iter();
        let step = trans(proc(rr, x) { rr.push(x); rr });
        let mut r = Vec::new();
        loop {
            match iter.next() {
                Some(x) => r = step(r, x),
                None => break
            }
        }
        r
    }
}

fn mapping<'f, R, T, U>(f: |T|: 'f -> U) -> Transducer<'f, R, T, U> {
    proc(step) proc(r, x) step(r, f(x))
}

fn filtering<'p, R, T>(pred: |&T|: 'p -> bool) -> Transducer<'p, R, T, T> {
    proc(step) proc(r, x) if pred(&x) { step(r, x) } else { r }
}

#[test]
fn it_works() {
}
