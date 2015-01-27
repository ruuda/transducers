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

/// The function composition `F` after `G`.
pub struct Composed<X, Y, Z, F, G> {
    f: F,
    g: G
}

impl<X, Y, Z, F, G> Fn(X) -> Z for Composed<X, Y, Z, F, G>
where F: Fn(Y) -> Z,
      G: Fn(X) -> Y {
    extern "rust-call" fn call(&self, arg: (X,)) -> Z {
        let (x,) = arg;
        let y = (self.g)(x);
        let z = (self.f)(y);
        z
    }
}

/// Composes the functions `f` and `g` to the function `f` after `g`.
pub fn compose<X, Y, Z, F, G>(f: F, g: G) -> Composed<X, Y, Z, F, G>
where F: Fn(Y) -> Z,
      G: Fn(X) -> Y {
    Composed { f: f, g: g }
}

#[test]
fn compose_is_associative() {
    let f = |&: x: i32| x * 2;
    let g = |&: x: i32| x + 2;
    let h = |&: x: i32| x * x;
    assert_eq!(compose(compose(f, g), h)(42), 3532);
    let f = |&: x: i32| x * 2;
    let g = |&: x: i32| x + 2;
    let h = |&: x: i32| x * x;
    assert_eq!(compose(f, compose(g, h))(42), 3532);
}

#[test]
fn compose_typechecks() {
    use std::num;
    let f = |&: x: Option<i16>| if let Some(n) = x { n } else { 0 };
    let g = |&: x: u16| num::cast::<u16, i16>(x);
    let h = compose(f, g);
    assert_eq!(h(42), 42);
    assert_eq!(h(65535), 0);
}

#[test]
fn compose_with_id_is_id() {
    let id = |&: x: i32| x;
    let ff = |&: x: i32| x * 2;
    assert_eq!(ff(42), compose(id, ff)(42));
    let id = |&: x: i32| x;
    let ff = |&: x: i32| x * 2;
    assert_eq!(ff(42), compose(ff, id)(42));
}

/// The transducer composition `F` after `G`.
pub struct ComposedTransducer<'t, R, X, Y, Z, F, G> {
    f: F,
    g: G
}

// NOTE: The lifetime error looks similar to https://github.com/rust-lang/rust/issues/21520
// It could be a compiler bug. Otherwise the help message is misleading. Posted
// to the topic at discuss.rust-lang.org.
impl<'t, R, X, Y, Z, F, G> Transducer<'t, R, X, Z>
for ComposedTransducer<'t, R, X, Y, Z, F, G>
where F: Transducer<'t, R, Y, Z>,
      G: Transducer<'t, R, X, Y>,
      <F as Transducer<'t, R, Y, Z>>::Step: 't {
    type Step = <F as Transducer<'t, R, Y, Z>>::Step;
    fn apply<Step: Fn(R, X) -> R + 't>(&self, step: Step) -> <F as Transducer<'t, R, Y, Z>>::Step {
        self.f.apply(self.g.apply(step))
    }
}
