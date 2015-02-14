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

/// The identity transducer.
pub struct Identity;

impl<'t, R: 't, T: 't> Transducer<'t, R, T, T> for Identity {
    type Step = IdentityStep<'t, R, T>;

    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> IdentityStep<'t, R, T> {
        IdentityStep { step: Box::new(step) }
    }
}

impl Identity {
    /// Produces the identity transducer.
    pub fn new() -> Identity {
        Identity
    }
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

/// The mapping transducer that applies a function to every element.
pub struct Mapping<'t, T: 't, U, F: Fn(U) -> T + 't> {
    f: &'t F
}

impl<'t, R: 't, T: 't, U, F: Fn(U) -> T + 't> Transducer<'t, R, T, U>
for Mapping<'t, T, U, F> {

    type Step = MappingStep<'t, R, T, F>;

    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> MappingStep<'t, R, T, F> {
       MappingStep {
           step: Box::new(step),
           f: self.f
       }
    }
}

impl<'f, T: 'f, U, F: Fn(U) -> T + 'f> Mapping<'f, T, U, F> {
    /// The mapping transducer that applies `f` to every element.
    pub fn new(f: &'f F) -> Mapping<'f, T, U, F> {
        Mapping { f: f }
    }
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

/// The filtering transducer that passes through elements that satisfy a predicate.
pub struct Filtering<'p, T: 'p, P: Fn(&T) -> bool + 'p> {
    p: &'p P
}

impl<'p, R: 'p, T: 'p, P: Fn(&T) -> bool + 'p> Transducer<'p, R, T, T>
for Filtering<'p, T, P> {

    type Step = FilteringStep<'p, R, T, P>;

    fn apply<Step: Fn(R, T) -> R + 'p>(&self, step: Step) -> FilteringStep<'p, R, T, P> {
        FilteringStep {
            step: Box::new(step),
            p: self.p
        }
    }
}

impl<'p, T, P: Fn(&T) -> bool + 'p> Filtering<'p, T, P> {
    /// The filtering transducer passes through all elements for which the predicate `p` is true.
    pub fn new(p: &'p P) -> Filtering<'p, T, P> {
        Filtering { p: p }
    }
}
