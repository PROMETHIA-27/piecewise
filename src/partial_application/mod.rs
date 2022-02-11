mod impls;

pub use impls::*;

use crate::tuple_traits::*;

pub struct Applied<App: NonUnitTuple, Tail: Tuple, R> {
    app: App,
    f: <App::PrependNonUnitToTuple<Tail> as Tuple>::Fn<R>,
}

impl<App: NonUnitTuple, Tail: Tuple, R> FnOnce<Tail> for Applied<App, Tail, R> {
    type Output = R;

    extern "rust-call" fn call_once(self, args: Tail) -> Self::Output {
        self.f.call_once(self.app.prepend_non_unit_to_tuple(args))
    }
}

impl<App: NonUnitTuple, Tail: Tuple, R> Applied<App, Tail, R> {
    pub fn call(self, rest: Tail) -> R {
        self.f.call(self.app.prepend_non_unit_to_tuple(rest))
    }
}

pub trait Applicable<App: Tuple, Tail: NonUnitTuple, R> {
    fn apply(self, next_param: Tail::Head) -> Applied<App::PrependToNonUnitTuple<(Tail::Head,)>, Tail::Tail, R>;
}