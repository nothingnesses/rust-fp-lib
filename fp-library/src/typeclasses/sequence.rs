use crate::hkt::{Apply, Kind};

pub trait Sequence {
	/// forall a b. Sequence f => f (a -> b) -> f a -> f b
	fn sequence<F, A, B>(ff: &Apply<Self, F>) -> impl Fn(&Apply<Self, A>) -> Apply<Self, B>
	where
		Self: Kind<F> + Kind<A> + Kind<B>,
		F: Fn(&A) -> B,
		A: Clone;
}

/// forall a b. Sequence f => f (a -> b) -> f a -> f b
pub fn sequence<Brand, F, A, B>(
	ff: &Apply<Brand, F>
) -> impl Fn(&Apply<Brand, A>) -> Apply<Brand, B>
where
	Brand: Kind<F> + Kind<A> + Kind<B> + Sequence,
	F: Fn(&A) -> B,
	A: Clone,
{
	|fa| Brand::sequence::<F, _, _>(ff)(fa)
}
