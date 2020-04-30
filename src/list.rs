#![macro_use]

#[derive(Clone, Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::{Cons, Nil};

#[macro_export]
macro_rules! ls[
  ()                       => (List::Nil);
  ($x:expr)                => (List::Cons($x, Box::new(List::Nil)));
  ($x:expr, $($xs:expr),+) => (List::Cons($x, Box::new(ls!($($xs),+))));
];

impl<T: Clone> List<T> {
    pub fn new<A>() -> List<A> { Nil }

    pub fn prepend(self, x: T) -> Self { return Cons(x, Box::new(self)); }

    pub fn get(&self, index: u32) -> Option<&T> {
        match self {
            Nil => None,
            Cons(head, _) if index == 0 => Some(head),
            Cons(_, ref tail) => tail.get(index - 1),
        }
    }

    pub fn map<R>(self, func: fn(x: T) -> R) -> List<R> {
        match self {
            Nil => Nil,
            Cons(x, tail) => Cons(func(x), Box::new(tail.clone().map(func))),
        }
    }

    pub fn filter(self, func: fn(x: &T) -> bool) -> Self {
        match self {
            Nil => self,
            Cons(x, tail) => if func(&x) {
                return Cons(x, Box::new(tail.clone().filter(func)));
            } else {
                return tail.clone().filter(func);
            }
        }
    }

    pub fn len(&self) -> u32 {
        match *self {
            Nil => 0,
            Cons(_, ref tail) => 1 + tail.len(),
        }
    }
}
