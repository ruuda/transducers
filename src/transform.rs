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

use std::marker::PhantomData;
use super::Transducer;

// TODO: Would it be possible to take Fn(R, T) -> R as a parameter,
// and then have different kind of step functions for once, mut and borrow?
pub struct IdentityStep<'t, R, T> {
    step: Box<Fn(R, T) -> R + 't>
}

impl<'t, R, T> FnOnce<(R, T)> for IdentityStep<'t, R, T> {
    type Output = R;
    extern "rust-call" fn call_once(self, args: (R, T)) -> R {
        self.call(args)
    }
}

impl<'t, R, T> FnMut<(R, T)> for IdentityStep<'t, R, T> {
    extern "rust-call" fn call_mut(&mut self, args: (R, T)) -> R {
        self.call(args)
    }
}

impl<'t, R, T> Fn<(R, T)> for IdentityStep<'t, R, T> {
    extern "rust-call" fn call(&self, (r, t): (R, T)) -> R {
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

impl<'t, R, T, U, F> FnOnce<(R, U)> for MappingStep<'t, R, T, F>
where F: Fn(U) -> T + 't {
    type Output = R;
    extern "rust-call" fn call_once(self, args: (R, U)) -> R {
        self.call(args)
    }
}

impl<'t, R, T, U, F> FnMut<(R, U)> for MappingStep<'t, R, T, F>
where F: Fn(U) -> T + 't {
    extern "rust-call" fn call_mut(&mut self, args: (R, U)) -> R {
        self.call(args)
    }
}

impl<'t, R, T, U, F> Fn<(R, U)> for MappingStep<'t, R, T, F>
where F: Fn(U) -> T + 't {
    extern "rust-call" fn call(&self, (r, u): (R, U)) -> R {
        (*self.step)(r, (self.f)(u))
    }
}

/// The mapping transducer that applies a function to every element.
pub struct Mapping<'t, T: 't, U, F: Fn(U) -> T + 't> {
    f: &'t F,
    phantom_f: PhantomData<fn(U) -> T>
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
        Mapping { f: f, phantom_f: PhantomData }
    }
}

pub struct FilteringStep<'t, R, T, P: 't> {
    step: Box<Fn(R, T) -> R + 't>,
    p: &'t P
}

impl<'t, R, T, P> FnOnce<(R, T)> for FilteringStep<'t, R, T, P>
where P: Fn(&T) -> bool + 't {
    type Output = R;
    extern "rust-call" fn call_once(self, args: (R, T)) -> R {
        self.call(args)
    }
}

impl<'t, R, T, P> FnMut<(R, T)> for FilteringStep<'t, R, T, P>
where P: Fn(&T) -> bool + 't {
    extern "rust-call" fn call_mut(&mut self, args: (R, T)) -> R {
        self.call(args)
    }
}

impl<'t, R, T, P> Fn<(R, T)> for FilteringStep<'t, R, T, P>
where P: Fn(&T) -> bool + 't {
    extern "rust-call" fn call(&self, (r, t): (R, T)) -> R {
        if (self.p)(&t) {
            (*self.step)(r, t)
        } else {
            r
        }
    }
}

/// The filtering transducer that passes through elements that satisfy a predicate.
pub struct Filtering<'p, T: 'p, P: Fn(&T) -> bool + 'p> {
    p: &'p P,
    phantom_p: PhantomData<fn(&T) -> bool>
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
        Filtering { p: p, phantom_p: PhantomData }
    }
}
