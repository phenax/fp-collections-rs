#![macro_use]
use std::slice::Iter;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub use List::{Cons, Nil};

#[macro_export]
macro_rules! ls[
  []                       => (List::Nil);
  [$x:expr]                => (List::Cons($x, Box::new(List::Nil)));
  [$x:expr, $($xs:expr),+] => (List::Cons($x, Box::new(ls![$($xs),+])));
];

impl<T: Clone> From<Iter<'_, T>> for List<T> {
    fn from(iter: Iter<'_, T>) -> Self {
        let mut list = ls![];
        for x in iter {
            list = list.append(x.clone());
        }

        list
    }
}

impl<T: Clone> From<&[T]> for List<T> {
    fn from(array: &[T]) -> Self {
        List::from(array.iter()).map(|x| x)
    }
}

impl<T: Eq> List<T> {
    pub fn new<A>() -> Self {
        Nil
    }

    pub fn is_empty(&self) -> bool {
        self.eq(&Nil)
    }
    pub fn null(&self) -> bool {
        self.is_empty()
    }

    pub fn len(&self) -> u32 {
        fn aux<T>(xs: &List<T>, size: u32) -> u32 {
            match *xs {
                Nil => size,
                Cons(_, ref tail) => aux(tail, size + 1),
            }
        };

        aux(self, 0)
    }
}

impl<T: Clone> List<T> {
    pub fn prepend(self, x: T) -> Self {
        Cons(x, Box::new(self))
    }

    pub fn append(self, item: T) -> Self {
        match self {
            Nil => ls![item],
            Cons(head, tail) => tail.append(item).prepend(head),
        }
    }

    pub fn concat(self, other: Self) -> Self {
        match self {
            Nil => other,
            Cons(head, tail) => tail.concat(other).prepend(head),
        }
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        match self {
            Nil => None,
            Cons(head, _) if index == 0 => Some(head),
            Cons(_, ref tail) => tail.get(index - 1),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn tail(&self) -> Self {
        match self {
            Nil => (*self).clone(),
            Cons(_, ref tail) => unsafe {
                let t = (*tail).clone();
                (*Box::into_raw(t)).clone()
            },
        }
    }

    pub fn foldl<R>(self, func: fn(R, T) -> R, init: R) -> R {
        match self {
            Nil => init,
            Cons(x, tail) => tail.foldl(func, func(init, x)),
        }
    }
 
    pub fn foldr<R>(self, func: fn(T, R) -> R, init: R) -> R {
        match self {
            Nil => init,
            Cons(x, tail) => func(x, tail.foldr(func, init)),
        }
    }

    pub fn reverse(self) -> List<T> {
        fn aux<T>(xs: List<T>, ys: List<T>) -> List<T> {
            match ys {
                Nil => xs,
                Cons(y, tail) => aux(Cons(y, Box::new(xs)), *tail)
            }
        };
        aux(Nil, self)
    }

    pub fn map<R>(self, func: fn(x: T) -> R) -> List<R> {
        match self {
            Nil => Nil,
            Cons(x, tail) => Cons(func(x), Box::new(tail.map(func))),
        }
    }

    pub fn filter(self, func: fn(&T) -> bool) -> Self {
        match self {
            Nil => self,
            Cons(x, tail) => {
                if func(&x) {
                    Cons(x, Box::new(tail.filter(func)))
                } else {
                    tail.filter(func)
                }
            },
        }
    }
}
