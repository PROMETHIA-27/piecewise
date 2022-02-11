mod impls;

pub use impls::*;

pub trait Tuple where Self: Sized {
    type Cons<N>: NonUnitTuple;
    type PrependToTuple<T: Tuple>: Tuple;
    type PrependToNonUnitTuple<T: NonUnitTuple>: NonUnitTuple;
    type Fn<R>: Fn<Self, Output = R>;

    fn cons<N>(self, n: N) -> Self::Cons<N>;

    fn prepend_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependToTuple<T>;

    fn prepend_to_non_unit_tuple<T: NonUnitTuple>(self, tup: T) -> Self::PrependToNonUnitTuple<T>;
}

pub trait NonUnitTuple : Tuple {
    type Head;
    type Tail: Tuple;
    type PrependNonUnitToTuple<T: Tuple>: NonUnitTuple;

    fn split(self) -> (Self::Head, Self::Tail);

    fn prepend_non_unit_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependNonUnitToTuple<T>;
}

pub trait ToSingle where Self: Sized {
    fn to_single(self) -> (Self,) {
        (self,)
    }
}
impl<T> ToSingle for T {}