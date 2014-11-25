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

#![feature(unboxed_closures)]

type Step<'s, R, T> = Fn<(R, T), R> + 's;
type Transducer<'t, R, T, U> = Fn<(Step<'t, R, U>), Step<'t, R, T>> + 't;

fn do_mapping<'f, R, T, U>(f: &'f mut Fn<(T,), U>, step: Step<'f, R, U>) -> Step<'f, R, T> {
    |r, x| step(r, f(x))
}

fn mapping<'f, R, T, U>(f: &'f mut Fn<(T,), U>) -> Transducer<'f, R, T, U> {
    |&: step| do_mapping(f, step)
}

#[test]
fn it_works() {
}
