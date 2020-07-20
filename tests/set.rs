use fp_collections::{set, set::Set};

#[test]
fn set_create() {
    let s = set![10, 9, 8, 9, 8, 7, 8, 7, 6, 5, 4, 3, 2, 1, 1, 10];

    assert_eq!(format!("{:?}", s), 
    "Node(Node(Node(Node(Empty, 1, Empty, 0), 2, Empty, 1), 3, Node(Node(Empty, 4, Empty, 0), 5, Node(Empty, 6, Empty, 0), 1), 2), 7, Node(Node(Empty, 8, Empty, 0), 9, Node(Empty, 10, Empty, 0), 1), 3)");

    assert_eq!(s.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
}

pub mod set_from_vec {
    use fp_collections::{set, set::Set};

    #[test]
    fn it_returns_vec_from_set() {
        let s = Set::from(vec![1, 2, 3]);
        assert_eq!(s, set![1, 2, 3])
    }
}

pub mod set_add {
    use fp_collections::{set, set::Set};

    #[test]
    fn it_adds_values_in_set() {
        let mut s: Set<i64> = set![];
        s = s.add(3);
        s = s.add(3);
        s = s.add(3);
        s = s.add(3);
        s = s.add(3);
        s = s.add(2);
        s = s.add(2);
        s = s.add(2);
        s = s.add(1);
        s = s.add(1);
        s = s.add(1);

        assert_eq!(s, set![1, 2, 3])
    }
}

pub mod set_mem {
    use fp_collections::{set, set::Set};

    #[test]
    fn it_checks_membership_in_set() {
        let s = set![1, 3, 5, 7, 9];

        assert_eq!(s.clone().mem(0), false);
        assert_eq!(s.clone().mem(1), true);
        assert_eq!(s.clone().mem(2), false);
        assert_eq!(s.clone().mem(3), true);
        assert_eq!(s.clone().mem(4), false);
        assert_eq!(s.clone().mem(5), true);
        assert_eq!(s.clone().mem(6), false);
        assert_eq!(s.clone().mem(7), true);
        assert_eq!(s.clone().mem(8), false);
        assert_eq!(s.clone().mem(9), true);
        assert_eq!(s.clone().mem(10), false);
    }
}

pub mod set_remove {
    use fp_collections::{set, set::Set};

    #[test]
    fn it_removes_value_from_set() {
        let mut s = set![1, 3, 5, 7, 9];
        s = s.remove(0);
        println!("{:?}", s);

        s = s.remove(5);
        s = s.remove(1);
        s = s.remove(2);
        s = s.remove(3);
        s = s.remove(4);

        assert_eq!(s, set![7, 9]);
    }
}

pub mod set_max_elt {
    use fp_collections::{set, set::Set};

    #[test]
    fn it_returns_max_element_in_set() {
        let s = set![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(s.max_elt(), 10);
    }
}
