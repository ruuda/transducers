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

impl<X, Y, Z, F, G> Fn<(X,)> for Composed<X, Y, Z, F, G>
where F: Fn(Y) -> Z,
      G: Fn(X) -> Y {
    type Output = Z;
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
pub struct ComposedTransducer<R, T, U, V, F, G> {
    f: F,
    g: G
}

impl<'t, R, T, U, V, FStep: Fn(R, V) -> R, GStep: Fn(R, U) -> R, F, G>
    Transducer<'t, R, T, V> for ComposedTransducer<R, T, U, V, F, G>
where F: Transducer<'t, R, U, V, Step = FStep> + 't,
      G: Transducer<'t, R, T, U, Step = GStep> + 't,
      FStep: 't,
      GStep: 't {
    type Step = FStep;
    fn apply<Step: Fn(R, T) -> R + 't>(&self, step: Step) -> FStep {
        self.f.apply(self.g.apply(step))
    }
}

pub fn compose_trans<'t, R, T, U, V, F, G>(f: F, g: G) -> ComposedTransducer<R, T, U, V, F, G>
where F: Transducer<'t, R, U, V>,
      G: Transducer<'t, R, T, U> {
    ComposedTransducer { f: f, g: g }
}

#[test]
fn compose_trans_is_associative() {
    use super::Mapping;
    let f = |&: x: i32| x * 2;
    let g = |&: x: i32| x + 2;
    let h = |&: x: i32| x * x;
    let step = |&: r: i32, t: i32| r + t;
    let comp_left = compose_trans(compose_trans(Mapping::new(&h), Mapping::new(&g)), Mapping::new(&f));
    assert_eq!(comp_left.apply(step)(0, 42), 3532);
    let step = |&: r: i32, t: i32| r + t;
    let comp_right = compose_trans(Mapping::new(&h), compose_trans(Mapping::new(&g), Mapping::new(&f)));
    assert_eq!(comp_right.apply(step)(0, 42), 3532);
}

#[test]
fn compose_trans_typechecks() {
    use std::num;
    use super::Mapping;
    let f = |&: x: Option<i16>| if let Some(n) = x { n } else { 0 };
    let g = |&: x: u16| num::cast::<u16, i16>(x);

    // Note the 'reversed' composition order, to map `f after g`, we compose
    // as `mapping(g) after mapping(f)`.
    let comp = compose_trans(Mapping::new(&g), Mapping::new(&f));
    let step = |&: r: i16, t: i16| r + t;
    assert_eq!(comp.apply(step)(0, 42), 42);
    let step = |&: r: i16, t: i16| r + t;
    assert_eq!(comp.apply(step)(0, 65535), 0);
}

#[test]
fn compose_trans_with_id_is_id() {
    use super::{Identity, Mapping};
    let f = |&: x: i32| x * 2;
    let step = |&: r: i32, t: i32| r + t;
    let comp_left = compose_trans(Identity::new(), Mapping::new(&f));
    assert_eq!(f(42), comp_left.apply(step)(0, 42));
    let step = |&: r: i32, t: i32| r + t;
    let comp_right = compose_trans(Mapping::new(&f), Identity::new());
    assert_eq!(f(42), comp_right.apply(step)(0, 42));
}
