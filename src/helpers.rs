
pub fn identity<T>(x: T) -> T { x }

pub fn constant<T: Clone>(x: T) -> impl Fn() -> T { move || x.clone() }

pub fn compose2<T, V, R>(fn1: impl Fn(V) -> R, fn2: impl Fn(T) -> V) -> impl Fn(T) -> R {
    move |x| fn1(fn2(x))
}

