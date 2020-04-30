#![macro_use]

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

impl<T: Clone> List<T> {
    pub fn new<A>() -> Self { Nil }

    pub fn prepend(self, x: T) -> Self { Cons(x, Box::new(self)) }

    pub fn get(&self, index: u32) -> Option<&T> {
        match self {
            Nil => None,
            Cons(head, _) if index == 0 => Some(head),
            Cons(_, ref tail) => tail.get(index - 1),
        }
    }

    pub fn head(&self) -> Option<&T> { self.get(0) }

    pub fn tail(&self) -> Self {
        match self {
            Nil => (*self).clone(),
            Cons(_, ref tail) => unsafe {
                let t = (*tail).clone();
                (*Box::into_raw(t)).clone()
            },
        }
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
            Cons(x, tail) => if func(&x) {
                Cons(x, Box::new(tail.filter(func)))
            } else {
                tail.filter(func)
            }
        }
    }

    pub fn len(&self) -> u32 {
        match *self {
            Nil => 0,
            Cons(_, ref tail) => 1 + tail.len(),
        }
    }

    // pub fn is_empty(&self) -> bool { return *self == Nil; }


    //TODO: Implement this more efficiently O(n)
    //append - add a value to the end of a list
    pub fn append(self, y: T) -> Self {
        match self {
            Nil => Cons(y, Box::new(Nil)),
            Cons(x, tail) => Cons(x, Box::new(tail.append(y)))
        }
    }

    //TODO: Implement this more efficiently O(n^2)
    //concat - combine two lists (1, (2, Nil)) + (3, Nil) -> (1, (2, (3, Nil)))
    pub fn concat(self, ys: List<T>) -> Self {
        match ys {
            Nil => self,
            Cons(y, tail) => self.append(y).concat(*tail)
        }
    }
}
