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

/// The mapping transducer, applies `f` to every element.
pub fn mapping<'f, T, U, F: Fn(U) -> T + 'f>(f: &'f F) -> Mapping<'f, F> {
    Mapping { f: f }
}
