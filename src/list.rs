#![macro_use]
use std::cmp::Ordering;
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

    pub fn foldl<R>(self, func: impl Fn(R, T) -> R + Copy, init: R) -> R {
        match self {
            Nil => init,
            Cons(x, tail) => tail.foldl(func, func(init, x)),
        }
    }

    pub fn foldr<R>(self, func: impl Fn(T, R) -> R + Copy, init: R) -> R {
        match self {
            Nil => init,
            Cons(x, tail) => func(x, tail.foldr(func, init)),
        }
    }

    pub fn reverse(self) -> List<T> {
        fn aux<T>(xs: List<T>, ys: List<T>) -> List<T> {
            match ys {
                Nil => xs,
                Cons(y, tail) => aux(Cons(y, Box::new(xs)), *tail),
            }
        };
        aux(Nil, self)
    }

    pub fn map<R>(self, func: impl Fn(T) -> R + Copy) -> List<R> {
        match self {
            Nil => Nil,
            Cons(x, tail) => Cons(func(x), Box::new(tail.map(func))),
        }
    }

    pub fn filter(self, func: impl Fn(&T) -> bool + Copy) -> Self {
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

    fn chop(self, ign: u32) -> Self {
        if ign == 0 {
            self
        } else {
            match self {
                Cons(_, tx) => tx.chop(ign - 1),
                _ => panic!("TODO: meaningful error message"),
            }
        }
    }
}

impl<T: Ord> List<T>
where
    T: Clone,
{
    pub fn qsort(self) -> Self {
        self.qsort_by(|x, y| x.cmp(y))
    }

    pub fn qsort_by(self, cmpfn: impl Fn(&T, &T) -> Ordering + Copy) -> Self {
        match self {
            Nil => self,
            Cons(head, tail) => {
                let smaller = tail
                    .clone()
                    .filter(|x| cmpfn(x, &head) == Ordering::Less)
                    .qsort_by(cmpfn);
                let bigger = tail
                    .filter(|x| cmpfn(x, &head) != Ordering::Less)
                    .qsort_by(cmpfn);

                smaller.append(head).concat(bigger)
            },
        }
    }
}


impl<T: Ord> List<T>
where
    T: Copy,
{
    pub fn sort(&self) -> Self {
        self.sort_by(|x, y| x.cmp(y))
    }

    pub fn sort_by(&self, cmpfn: impl Fn(&T, &T) -> Ordering + Copy) -> Self {
        let ln = self.len();
        if ln <= 1 {
            return self.clone();
        }
        let l1 = self.clone().chop(ln / 2);
        let l2 = self.clone().reverse().chop(ln - (ln / 2));
        l1.sort_by(cmpfn).merge(cmpfn, &l2.sort_by(cmpfn))
    }

    pub fn partition(self, p: fn(T) -> bool) -> (Self, Self) {
        fn aux<T: Clone + Copy + Eq + Ord>(
            yes: List<T>,
            no: List<T>,
            p: fn(T) -> bool,
            xs: List<T>,
        ) -> (List<T>, List<T>) {
            match xs {
                Nil => (yes, no),
                Cons(x, tx) => {
                    if p(x) {
                        // By doing aux(Cons(x, Box::new(yes)), no, p, *tx) [efficent]
                        // We get the yes list in the reverse order, but if
                        // we want of preserve order we need to append [not efficient]
                        aux(yes.append(x), no, p, *tx)
                    } else {
                        // aux(yes, Cons(x, Box::new(no)), p, *tx)
                        aux(yes, no.append(x), p, *tx)
                    }
                },
            }
        };
        aux(Nil, Nil, p, self)
    }

    pub fn merge(&self, cmpfn: impl Fn(&T, &T) -> Ordering + Copy, ys: &List<T>) -> Self {
        match (self, ys) {
            (Nil, l) => l.clone(),
            (l, Nil) => l.clone(),
            (Cons(x, tx), Cons(y, ty)) => {
                let cmp = cmpfn(x, y);
                if cmp == Ordering::Equal || cmp == Ordering::Less {
                    Cons(*x, Box::new(ys.merge(cmpfn, &**tx)))
                } else {
                    Cons(*y, Box::new(self.merge(cmpfn, &**ty)))
                }
            },
        }
    }
}
