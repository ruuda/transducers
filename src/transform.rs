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

use super::Transducer;

pub struct IdentityStep<'t, R, T> {
    step: Box<Fn(R, T) -> R + 't>
}

impl<'t, R, T> Fn<(R, T)> for IdentityStep<'t, R, T> {
    type Output = R;
    extern "rust-call" fn call(&self, args: (R, T)) -> R {
        let (r, t) = args;
        (*self.step)(r, t)
    }
}

pub struct Identity;

impl<'t, R: 't, T> Transducer<'t, R, T, T> for Identity {
    type Step = IdentityStep<'t, R, T>;

    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> IdentityStep<'t, R, T> {
        IdentityStep { step: Box::new(step) }
    }
}

/// The identity transducer.
pub fn identity() -> Identity {
    Identity
}

pub struct MappingStep<'t, R, T, F: 't> {
    step: Box<Fn(R, T) -> R + 't>,
    f: &'t F
}

impl<'t, R, T, U, F> Fn<(R, U)> for MappingStep<'t, R, T, F>
where F: Fn(U) -> T + 't {
    type Output = R;
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

/// The mapping transducer, applies `f` to every element.
pub fn mapping<'f, T, U, F: Fn(U) -> T + 'f>(f: &'f F) -> Mapping<'f, F> {
    Mapping { f: f }
}

pub struct FilteringStep<'t, R, T, P: 't> {
    step: Box<Fn(R, T) -> R + 't>,
    p: &'t P
}

impl<'t, R, T, P> Fn<(R, T)> for FilteringStep<'t, R, T, P>
where P: Fn(&T) -> bool + 't {
    type Output = R;
    extern "rust-call" fn call(&self, args: (R, T)) -> R {
        let (r, t) = args;
        if (self.p)(&t) {
            (*self.step)(r, t)
        } else {
            r
        }
    }
}

pub struct Filtering<'t, P: 't> {
    p: &'t P
}

impl <'t, R: 't, T, P> Transducer<'t, R, T, T> for Filtering<'t, P>
where P: Fn(&T) -> bool + 't {
    type Step = FilteringStep<'t, R, T, P>;

    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> FilteringStep<'t, R, T, P> {
        FilteringStep {
            step: Box::new(step),
            p: self.p
        }
    }
}

/// The filtering transducer passes through all elements for which the predicate `p` is true.
pub fn filtering<'p, T, P: Fn(&T) -> bool + 'p>(p: &'p P) -> Filtering<'p, P> {
    Filtering { p: p }
}
