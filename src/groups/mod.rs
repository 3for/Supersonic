//! Implementations for different mathematical groups, each of which satisfies our
//! `UnknownOrderGroup` trait. 
use crate::util::{int, TypeRep};
use rug::Integer;
use std::hash::Hash;
use std::fmt::Debug;

mod classgroupsti;

/// [Follow the idea from `https://github.com/cambrian/accumulator/src/group/mod.rs`]
/// A mathematical group.
///
/// This trait allows the implementation of standard group routines:
/// - Identity
/// - Op (the fundamental group operation)
/// - Exponentiation
/// - Inverse (particularly where this is efficient to compute)
///
/// The `TypeRep` trait lets us emulate type-level static fields, e.g. the modulus in an RSA group
/// or the discriminant in a class group.
///
/// Clients of this trait need to implement functions of the form `*_`, which take in `TypeRep`
/// data as a parameter. Consumers use functions without the underscore: `id`, `op`, `exp`, and
/// `inv`.

// The other traits are only required here because Rust can't figure out how to do stuff with an
// `Accumulator<G>` even though it's just a wrapped `G::Elem`. If possible we'd remove them.
pub trait Group: Clone + Debug + Eq + Hash + TypeRep + Send + Sync {
  // In theory the association `Group::Elem` is bijective, such that it makes sense to write
  // something like `Elem::Group::get()`. This would let us define `op`, `exp`, `inv`, etc. on the
  //`Elem` type and avoid using prefix notation for all of our group operations. Bijective
  // associated types are not currently supported by Rust.

  /// The associated group element type for this group.
  type Elem: Clone + Debug + Eq + Hash + Sized + Send + Sync;

  /// A group-specific wrapper for `id`.
  fn id_(rep: &Self::Rep) -> Self::Elem;

  /// A group-specific wrapper for `op`.
  fn op_(rep: &Self::Rep, a: &Self::Elem, b: &Self::Elem) -> Self::Elem;

  /// A group-specific wrapper for `exp`, although it comes with a default implementation via
  /// repeated squaring.
  ///
  /// Specific implementations may provide more performant specializations as needed (e.g.
  /// Montgomery multiplication for RSA groups).
  fn exp_(_rep: &Self::Rep, a: &Self::Elem, n: &Integer) -> Option<Self::Elem> {
    let (mut val, mut a, mut n) = {
      if *n < int(0) {
        (Self::idnew(), Self::invnew(a), int(-n))
      } else {
        (Self::idnew(), a.clone(), n.clone())
      }
    };
    while n > int(0) {
      if n.is_odd() {
        val = Self::opnew(&val, &a);
      }
      a = Self::opnew(&a, &a);
      n >>= 1;
    }
    Some(val)
  }

  /// A group-specific wrapper for `inv`.
  fn inv_(rep: &Self::Rep, a: &Self::Elem) -> Self::Elem;

  // -------------------
  // END OF REQUIRED FNS
  // -------------------

  /// Returns the identity element of the group.
  fn idnew() -> Self::Elem {
    Self::id_(Self::rep())
  }

  /// Applies the group operation to elements `a` and `b` and returns the result.
  fn opnew(a: &Self::Elem, b: &Self::Elem) -> Self::Elem {
    Self::op_(Self::rep(), a, b)
  }

  /// Applies the group operation to `a` and itself `n` times and returns the result.
  fn exp(a: &Self::Elem, n: &Integer) -> Option<Self::Elem> {
    Self::exp_(Self::rep(), a, n)
  }

  /// Returns the group inverse of `a`.
  fn invnew(a: &Self::Elem) -> Self::Elem {
    Self::inv_(Self::rep(), a)
  }
}

/// A group containing elements of unknown order.
///
/// **Note**: This trait does not imply that the group itself has unknown order (e.g. RSA groups).
#[allow(clippy::module_name_repetitions)]
pub trait UnknownOrderGroup: Group {
  /// Returns an element of unknown order in the group.
  fn unknown_order_elemnew() -> Self::Elem {
    Self::unknown_order_elem_(Self::rep())
  }

  /// A group-specific wrapper for `unknown_order_elem`.
  fn unknown_order_elem_(rep: &Self::Rep) -> Self::Elem;
}

/// Like `From<T>`, but implemented on the `Group` instead of the element type.
pub trait ElemFrom<T>: Group {
  /// Returns a group element from an initial value.
  fn elem_(val: T) -> Self::Elem;
}

/// Computes the product of `alpha_i ^ (p(x) / x_i)`, where `i` is an index into the `alphas` and
/// `x` arrays, and `p(x)` is the product of all `x_i`. See BBF (page 11).
pub fn multi_exp<G: Group>(alphas: &[G::Elem], x: &[Integer]) -> G::Elem {
  if alphas.len() == 1 {
    return alphas[0].clone();
  }

  let n_half = alphas.len() / 2;
  let alpha_l = &alphas[..n_half];
  let alpha_r = &alphas[n_half..];
  let x_l = &x[..n_half];
  let x_r = &x[n_half..];
  let x_star_l = x_l.iter().product();
  let x_star_r = x_r.iter().product();
  let l = multi_exp::<G>(alpha_l, x_l);
  let r = multi_exp::<G>(alpha_r, x_r);
  G::opnew(&G::exp(&l, &x_star_r).unwrap(), &G::exp(&r, &x_star_l).unwrap())
}
