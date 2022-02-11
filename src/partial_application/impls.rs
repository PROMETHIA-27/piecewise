use super::*;

impl<
    App: Tuple + Tuple<PrependToNonUnitTuple<(Tail::Head,)> = ShiftedApp> + NonUnitTuple<PrependNonUnitToTuple<Tail> = TotalApp>,
    Tail: NonUnitTuple + NonUnitTuple<Tail = ShiftedTail>, 
    R, 
    ShiftedApp: NonUnitTuple + NonUnitTuple<PrependNonUnitToTuple<ShiftedTail> = TotalApp>,
    ShiftedTail: Tuple,
    TotalApp: NonUnitTuple
> Applicable<App, Tail, R> for Applied<App, Tail, R> {
    fn apply(self, next_param: Tail::Head) -> Applied<ShiftedApp, ShiftedTail, R> {
        Applied { 
            app: self.app.prepend_to_non_unit_tuple(next_param.to_single()), 
            f: self.f
        }
    }
}

impl<T1, R> Applicable<(), (T1,), R> for fn(T1) -> R {
    fn apply(self, next_param: <(T1,) as NonUnitTuple>::Head) -> Applied<<() as Tuple>::PrependToNonUnitTuple<(<(T1,) as NonUnitTuple>::Head,)>, <(T1,) as NonUnitTuple>::Tail, R> {
        Applied {
            app: (next_param,),
            f: self,
        }
    }
}

impl<T1, T2, R> Applicable<(), (T1, T2), R> for fn(T1, T2) -> R {
    fn apply(self, next_param: <(T1, T2) as NonUnitTuple>::Head) -> Applied<<() as Tuple>::PrependToNonUnitTuple<(<(T1, T2) as NonUnitTuple>::Head,)>, <(T1, T2) as NonUnitTuple>::Tail, R> {
        Applied {
            app: (next_param,),
            f: self,
        }
    }
}

impl<T1, T2, T3, R> Applicable<(), (T1, T2, T3), R> for fn(T1, T2, T3) -> R {
    fn apply(self, next_param: <(T1, T2, T3) as NonUnitTuple>::Head) -> Applied<<() as Tuple>::PrependToNonUnitTuple<(<(T1, T2, T3) as NonUnitTuple>::Head,)>, <(T1, T2, T3) as NonUnitTuple>::Tail, R> {
        Applied { app: (next_param,), f: self }
    }
}