use crate::*;

#[test]
fn triple_addition() {
    fn trip_add(x: i32, y: i32, z: i32) -> i32 {
        x + y + z
    }

    let partial = (trip_add as fn(i32, i32, i32) -> i32).apply(12);
    let partial = partial.apply(6);
    
    assert_eq!(partial(3), 21);
}