use super::*;

macro reverse_tokens_and_call {
    (
        $macro:ident
        $($toks:tt)*
    ) => {
        reverse_tokens_and_call!{
            @inner
            $macro
            {}
            {$($toks)*}
        }
    },

    (
        @inner
        $macro:ident
        {$($buffer:tt)*}
        {$next:tt $($remaining:tt)*}
    ) => {
        reverse_tokens_and_call!{
            @inner
            $macro
            { $next $($buffer)* }
            {$($remaining)*}
        }
    },

    (
        @inner
        $macro:ident
        {$($buffer:tt)*}
        {}
    ) => {
        $macro ! { $($buffer)* }
    }
}

macro TupleImpl {
    (
        ($($($tys:tt),+)?)
        ($($tup_prepend:tt)*)
        ($($num:tt)*) // TODO: Replace with series generator
    ) => {
        impl $(<$($tys),+>)? Tuple for ( $($($tys,)+)? ) { // for ___
            type Cons<N> = (N, $($($tys),+)? ); // (N, ___)
            $($tup_prepend)*; // complicated
            type PrependToNonUnitTuple<T: NonUnitTuple> = Self::PrependToTuple<T>;
            type Fn<R> = fn($($($tys),+)?) -> R; // fn(___)
        
            tuple_cons_func!($($num)*);

            reverse_tokens_and_call!(tuple_prepend_to_tuple_func $($num)*);
        
            fn prepend_to_non_unit_tuple<T: NonUnitTuple>(self, tup: T) -> Self::PrependToNonUnitTuple<T> {
                self.prepend_to_tuple(tup)
            }
        }
    }
}

TupleImpl!{
    ()
    (type PrependToTuple<T: Tuple> = T)
    ()
}

TupleImpl!{
    (T1)
    (type PrependToTuple<T: Tuple> = T::Cons<T1>)
    (0)
}

TupleImpl!{ 
    (T1, T2) 
    (type PrependToTuple<T: Tuple> = <T::Cons<T2> as Tuple>::Cons<T1>)
    (0 1)
}

TupleImpl!{ 
    (T1, T2, T3) 
    (type PrependToTuple<T: Tuple> = <<T::Cons<T3> as Tuple>::Cons<T2> as Tuple>::Cons<T1>)
    (0 1 2)
}

macro tuple_cons_func {
    ($($num:tt)*) => {
        fn cons<N>(self, n: N) -> Self::Cons<N> {
            (n, $(self . $num),*) 
        }
    }
}

macro tuple_prepend_to_tuple_func {
    ($($num:tt)*) => {
        fn prepend_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependToTuple<T> {
            tup $(. cons ( self . $num ))* 
        }
    }
}

// impl Tuple for () { // for ___
//     type Cons<N> = (N,); // (N, ___)
//     type PrependToTuple<T: Tuple> = T; // complicated
//     type PrependToNonUnitTuple<T: NonUnitTuple> = Self::PrependToTuple<T>;
//     type Fn<R> = fn() -> R; // fn(___)

//     fn cons<N>(self, n: N) -> Self::Cons<N> {
//         (n,) // n, ___
//     }

//     fn prepend_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependToTuple<T> {
//         tup // .cons(_).cons(_)
//     }

//     fn prepend_to_non_unit_tuple<T: NonUnitTuple>(self, tup: T) -> Self::PrependToNonUnitTuple<T> {
//         self.prepend_to_tuple(tup)
//     }
// }
// impl<T1> Tuple for (T1,) {
//     type Cons<N> = (N, T1,);
//     type PrependToTuple<T: Tuple> = T::Cons<T1>;
//     type PrependToNonUnitTuple<T: NonUnitTuple> = Self::PrependToTuple<T>;
//     type Fn<R> = fn(T1,) -> R;

//     fn cons<N>(self, n: N) -> Self::Cons<N> {
//         (n, self.0)
//     }

//     fn prepend_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependToTuple<T> {
//         tup.cons(self.0)
//     }

//     fn prepend_to_non_unit_tuple<T: NonUnitTuple>(self, tup: T) -> Self::PrependToNonUnitTuple<T> {
//         self.prepend_to_tuple(tup)
//     }
// }
// impl<T1, T2> Tuple for (T1, T2) {
//     type Cons<N> = (N, T1, T2);
//     type PrependToTuple<T: Tuple> = <T::Cons<T2> as Tuple>::Cons<T1>;
//     type PrependToNonUnitTuple<T: NonUnitTuple> = Self::PrependToTuple<T>;
//     type Fn<R> = fn(T1, T2) -> R;

//     fn cons<N>(self, n: N) -> Self::Cons<N> {
//         (n, self.0, self.1)
//     }

//     fn prepend_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependToTuple<T> {
//         tup.cons(self.1).cons(self.0)
//     }

//     fn prepend_to_non_unit_tuple<T: NonUnitTuple>(self, tup: T) -> Self::PrependToNonUnitTuple<T> {
//         self.prepend_to_tuple(tup)
//     }
// }
// impl<T1, T2, T3> Tuple for (T1, T2, T3) {
//     // type Cons<N> = (N, T1, T2, T3);
//     type Cons<N> = (N, T2, T3);
//     type PrependToTuple<T: Tuple> = <<T::Cons<T3> as Tuple>::Cons<T2> as Tuple>::Cons<T1>;
//     type PrependToNonUnitTuple<T: NonUnitTuple> = Self::PrependToTuple<T>;
//     type Fn<R> = fn(T1, T2, T3) -> R;

//     fn cons<N>(self, n: N) -> Self::Cons<N> {
//         (n, self.1, self.2,)
//     }

//     fn prepend_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependToTuple<T> {
//         tup.cons(self.2).cons(self.1).cons(self.0)
//     }

//     fn prepend_to_non_unit_tuple<T: NonUnitTuple>(self, tup: T) -> Self::PrependToNonUnitTuple<T> {
//         self.prepend_to_tuple(tup)
//     }
// }

impl<T1> NonUnitTuple for (T1,) {
    type Head = T1;
    type Tail = ();
    type PrependNonUnitToTuple<T: Tuple> = Self::PrependToTuple<T>;

    fn split(self) -> (Self::Head, Self::Tail) {
        (self.0, ())
    }

    fn prepend_non_unit_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependNonUnitToTuple<T> {
        self.prepend_to_tuple(tup)
    }
}
impl<T1, T2> NonUnitTuple for (T1, T2) {
    type Head = T1;
    type Tail = (T2,);
    type PrependNonUnitToTuple<T: Tuple> = Self::PrependToTuple<T>;

    fn split(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1,))
    }

    fn prepend_non_unit_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependNonUnitToTuple<T> {
        self.prepend_to_tuple(tup)
    }
}
impl<T1, T2, T3> NonUnitTuple for (T1, T2, T3) {
    type Head = T1;
    type Tail = (T2, T3);
    type PrependNonUnitToTuple<T: Tuple> = Self::PrependToTuple<T>;

    fn split(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1, self.2))
    }

    fn prepend_non_unit_to_tuple<T: Tuple>(self, tup: T) -> Self::PrependNonUnitToTuple<T> {
        self.prepend_to_tuple(tup)
    }
}