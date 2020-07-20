// Reference Implementation: https://github.com/lucasaiu/ocaml/blob/master/stdlib/set.ml
#![macro_use]
use std::cmp::max;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Set<T> {
    Node(Box<Set<T>>, T, Box<Set<T>>, i64),
    Empty,
}

#[macro_export]
macro_rules! set[
    [$($x:expr),*] => {{
        let mut s = Set::new();
        $( s = s.add($x); )*
        s
    }};
];

impl<T: Eq> Set<T> {
    pub fn new() -> Self {
        Set::Empty
    }

    pub fn is_empty(&self) -> bool {
        self.eq(&Set::Empty)
    }

    pub fn len(&self) -> u32 {
        fn aux<T>(xs: &Set<T>, size: u32) -> u32 {
            match *xs {
                Set::Empty => 0,
                Set::Node(ref left, _, ref right, _) => aux(left, 1 + aux(right, size)),
            }
        };

        aux(self, 0)
    }
}

impl<T: Clone> Set<T>
where
    T: Ord,
{
    #[allow(clippy::should_implement_trait, clippy::many_single_char_names)]
    pub fn add(self, x: T) -> Self {
        match self.clone() {
            Set::Empty => Set::create_node(x),
            Set::Node(left, val, right, h) => match x.cmp(&val) {
                Ordering::Equal => self,
                Ordering::Less => {
                    let root = Set::Node(Box::new(left.add(x)), val, right, h);
                    match root {
                        Set::Node(l, v, r, _) => Set::balance(
                            Set::Node(
                                l.clone(),
                                v,
                                r.clone(),
                                1 + max(Set::height(&l), Set::height(&r)),
                            ),
                            *l,
                        ),
                        Set::Empty => Set::Empty,
                    }
                }
                Ordering::Greater => {
                    let root = Set::Node(left, val, Box::new(right.add(x)), h);
                    match root {
                        Set::Node(l, v, r, _) => Set::balance(
                            Set::Node(
                                l.clone(),
                                v,
                                r.clone(),
                                1 + max(Set::height(&l), Set::height(&r)),
                            ),
                            *r,
                        ),
                        Set::Empty => Set::Empty,
                    }
                }
            },
        }
    }

    fn height(s: &Set<T>) -> i64 {
        match *s {
            Set::Node(_, _, _, h) => h,
            Set::Empty => -1,
        }
    }

    fn right_rotate(parent: Set<T>, node: Set<T>) -> Self {
        match (parent, node) {
            (
                Set::Node(_, parent_val, parent_right, _),
                Set::Node(node_left, node_val, node_right, _),
            ) => {
                let h1 = 1 + max(Set::height(&parent_right), Set::height(&node_right));
                let h2 = 1 + max(h1, Set::height(&node_left));
                Set::Node(
                    node_left,
                    node_val,
                    Box::new(Set::Node(node_right, parent_val, parent_right, h1)),
                    h2,
                )
            }
            (_, _) => Set::Empty,
        }
    }

    fn left_rotate(parent: Set<T>, node: Set<T>) -> Self {
        match (parent, node) {
            (
                Set::Node(parent_left, parent_val, _, _),
                Set::Node(node_left, node_val, node_right, _),
            ) => {
                let h1 = 1 + max(Set::height(&parent_left), Set::height(&node_left));
                let h2 = 1 + max(h1, Set::height(&node_right));
                Set::Node(
                    Box::new(Set::Node(parent_left, parent_val, node_left, h1)),
                    node_val,
                    node_right,
                    h2,
                )
            }
            (_, _) => Set::Empty,
        }
    }

    fn balance(parent: Set<T>, node: Set<T>) -> Self {
        match parent.clone() {
            Set::Empty => parent,
            Set::Node(parent_left, _, parent_right, _) => match node {
                Set::Empty => parent,
                Set::Node(ref node_left, _, ref node_right, _) => {
                    let diff = Set::height(&parent_left) - Set::height(&parent_right);
                    if diff.abs() > 1 {
                        if diff > 0 {
                            let node_diff = Set::height(node_left) - Set::height(node_right);
                            if node_diff > 0 {
                                return Set::right_rotate(parent, node);
                            } else {
                                return Set::right_rotate(
                                    parent,
                                    Set::left_rotate(node.clone(), *node_right.clone()),
                                );
                            }
                        } else {
                            let node_diff = Set::height(node_left) - Set::height(node_right);
                            if node_diff < 0 {
                                return Set::left_rotate(parent, node);
                            } else {
                                return Set::left_rotate(
                                    parent,
                                    Set::right_rotate(node.clone(), *node_left.clone()),
                                );
                            }
                        }
                    }
                    parent
                }
            },
        }
    }

    fn create_node(x: T) -> Self {
        Set::Node(Box::new(Set::Empty), x, Box::new(Set::Empty), 0)
    }

    // TODO: fix later
    #[allow(clippy::wrong_self_convention)]
    pub fn to_vec(self) -> Vec<T> {
        let mut v = vec![];
        fn aux<I>(n: Set<I>, vs: &mut Vec<I>) -> &mut Vec<I> {
            match n {
                Set::Empty => vs,
                Set::Node(l, v, r, _) => {
                    aux(*l, vs);
                    vs.push(v);
                    aux(*r, vs);
                    vs
                }
            }
        }

        aux(self, &mut v);

        v
    }

    pub fn mem(self, x: T) -> bool {
        match self {
            Set::Empty => false,
            Set::Node(l, v, r, _) => match x.cmp(&v) {
                Ordering::Equal => true,
                Ordering::Less => l.mem(x),
                Ordering::Greater => r.mem(x),
            },
        }
    }

    pub fn remove(self, x: T) -> Self {
        match self {
            Set::Empty => Set::Empty,
            Set::Node(left, val, right, h) => match x.cmp(&val) {
                Ordering::Equal => match (*left.clone(), *right.clone()) {
                    (Set::Empty, Set::Empty) => Set::Empty,
                    (Set::Node(_, _, _, _), Set::Empty) => *left,
                    (Set::Empty, Set::Node(_, _, _, _)) => *right,
                    (Set::Node(_, _, _, _), Set::Node(_, _, _, _)) => {
                        let max = left.max_elt();
                        let root = Set::Node(Box::new(left.remove(max.clone())), max, right, h);
                        match root.clone() {
                            Set::Empty => Set::Empty,
                            Set::Node(l, _, _, _) => Set::balance(root, *l),
                        }
                    }
                },
                Ordering::Less => {
                    let root = Set::Node(Box::new(left.clone().remove(x)), val, right, h);
                    match root.clone() {
                        Set::Node(l, _, _, _) => Set::balance(root, *l),
                        Set::Empty => Set::Empty,
                    }
                }
                Ordering::Greater => {
                    let root = Set::Node(left, val, Box::new(right.clone().remove(x)), h);
                    match root.clone() {
                        Set::Node(_, _, r, _) => Set::balance(root, *r),
                        Set::Empty => Set::Empty,
                    }
                }
            },
        }
    }

    pub fn max_elt(&self) -> T {
        match self {
            Set::Empty => panic!("Not Found"),
            Set::Node(_, v, right, _) => match (**right).clone() {
                Set::Empty => v.clone(),
                Set::Node(_, _, _, _) => right.max_elt(),
            },
        }
    }
}

impl<T> From<Vec<T>> for Set<T>
where
    T: Eq + Clone + Ord,
{
    fn from(xs: Vec<T>) -> Self {
        xs.iter().fold(Set::new(), |s, x| s.add(x.clone()))
    }
}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Set::Empty
    }
}
