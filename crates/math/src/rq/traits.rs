#![warn(missing_docs, unused_imports)]

//! Traits associated with polynomials.

use super::{Context, Poly, Representation};
use std::rc::Rc;

/// Conversions to create polynomials.
///
/// We unfortunaly cannot use the `TryFrom` trait from std::convert because we
/// need to specify additional parameters, and if we try to redefine a `TryFrom` trait here,
/// we need to fully specify the trait when we use it because of the blanket implementation
/// <https://github.com/rust-lang/rust/issues/50133#issuecomment-488512355>.
pub trait TryConvertFrom<T>
where
	Self: Sized,
{
	/// The type of errors.
	type Error;

	/// Attempt to convert the `value` into a polynomial with a specific context and
	/// under a specific representation. The representation may optional and be
	/// specified as `None`; this is useful for example when converting from a
	/// representation that encodes the representation.
	fn try_convert_from<R>(
		value: T,
		ctx: &Rc<Context>,
		representation: R,
	) -> Result<Self, Self::Error>
	where
		R: Into<Option<Representation>>;
}

/// Switch a polynomial from one context to another.
pub trait ContextSwitcher {
	/// The type of errors.
	type Error;

	/// Switch the context of the polynomial.
	fn switch_context(&self, polynomial: &Poly) -> Result<Poly, Self::Error>;
}