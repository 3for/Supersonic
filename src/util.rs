use rug::Integer;

/// Pseudo-type-level programming.
/// This trait allows us to reflect "type-level" (i.e. static) information at runtime.
pub trait TypeRep: 'static {
  /// The associated type of the simulated type-level static information.
  type Rep: 'static;

  /// Returns the static data for the type.
  fn rep() -> &'static Self::Rep;
}

/// Convenience wrapper for creating `Rug` integers.
pub fn int<T>(val: T) -> Integer
where
  Integer: From<T>,
{
  Integer::from(val)
}