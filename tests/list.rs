use fp_collections::{list::{List}, ls};

#[test]
fn it_creates_list() {
    let list = ls!(1, 2);
    let debug_str = format!("{:?}", list);
    assert_eq!(debug_str, "Cons(1, Cons(2, Nil))");
}

pub mod ls_clone {
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_clone_list_correctly() {
        let list = ls!(0, 1, 2, 3);
        let newlist = list.clone();
        assert_eq!(list, newlist);
    }
}

pub mod ls_eq {
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_returns_false_for_different_lengths() {
        let a = ls!(5, 4, 3, 2, 1);
        let b = ls!(1, 2);
        assert_eq!(a.eq(&b), false);
    }

    #[test]
    fn it_returns_true_if_same() {
        let a = ls!(5, 4, 3, 2, 1);
        let b = ls!(5, 4, 3, 2, 1);
        assert_eq!(a.eq(&b), true);
    }

    #[test]
    fn it_returns_false_if_different() {
        let a = ls!(5, 4, 3, 2, 1);
        let b = ls!(5, 8, 3, 1, 1);
        assert_eq!(a.eq(&b), false);
    }
}


pub mod ls_get {
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_returns_some_item_at_index_if_it_exists() {
        let list = ls!(5, 4, 3, 2, 1);

        match list.get(0) {
            Some(n) => assert_eq!(*n, 5),
            None => panic!("Expected Some"),
        }

        match list.get(3) {
            Some(n) => assert_eq!(*n, 2),
            None => panic!("Expected Some"),
        }

        match list.get(4) {
            Some(n) => assert_eq!(*n, 1),
            None => panic!("Expected Some"),
        }
    }

    #[test]
    fn it_returns_none_if_it_doesnt_exists() {
        let list = ls!(5, 4, 3, 2, 1);
        match list.get(5) {
            Some(_) => panic!("Expected None"),
            None => (),
        }
    }
}


pub mod ls_head {
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_returns_some_head_if_it_exists() {
        let list = ls!(5, 4, 3, 2, 1);
        match list.head() {
            Some(n) => assert_eq!(*n, 5),
            None => panic!("Expected Some"),
        }
    }

    #[test]
    fn it_returns_none_if_it_doesnt_exists() {
        let list: List<i32> = ls!();
        match list.head() {
            Some(_) => panic!("Expected None"),
            None => (),
        }
    }
}


pub mod ls_prepend {
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_prepends_an_item_to_an_empty_list() {
        let list: List<i32> = ls!();
        let newlist = list.clone().prepend(9);
        assert_eq!(newlist, ls!(9));
        assert_eq!(list, ls!());
    }

    #[test]
    fn it_prepends_an_item_to_the_list() {
        let list = ls!(2, 1);
        let newlist = list.clone().prepend(3);
        assert_eq!(newlist, ls!(3, 2, 1));
        assert_eq!(list, ls!(2, 1));
    }
}


pub mod ls_map {
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_maps_an_empty_list() {
        let list: List<i32> = ls!();
        let newlist = list.clone().map(|x| x * 2);
        assert_eq!(list, newlist);
    }

    #[test]
    fn it_maps_a_list() {
        let list = ls!(8, 11);
        let newlist = list.clone().map(|x| x * 2);
        assert_ne!(list, newlist);
        assert_eq!(newlist, ls!(16, 22));
    }
}


pub mod ls_filter {
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_filter_an_empty_list() {
        let list: List<i32> = ls!();
        let newlist = list.clone().filter(|x| x % 2 == 0);
        assert_eq!(list, newlist);
    }

    #[test]
    fn it_filter_out_a_list_of_even_numbers() {
        let list = ls!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
        let newlist = list.clone().filter(|x| x % 2 == 0);
        assert_ne!(list, newlist);
        assert_eq!(newlist, ls!(2, 4, 6, 8, 10));
    }
}
