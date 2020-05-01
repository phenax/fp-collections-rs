
pub fn identity<T>(x: T) -> T { x }

// pub fn constant<T>(x: T) -> impl Fn() -> T { || x }

// macro_rules! compose {
    // ( $last:expr ) => { $last };
    // ( $head:expr, $($tail:expr), +) => {
        // compose_two($head, compose!($($tail),+))
    // };
// }

// fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
// where
    // F: Fn(A) -> B,
    // G: Fn(B) -> C,
// {
    // move |x| g(f(x))
// }

