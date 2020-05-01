use fp_collections::{list::List, ls};

#[test]
fn it_creates_list() {
    let list = ls![1, 2];
    let debug_str = format!("{:?}", list);
    assert_eq!(debug_str, "Cons(1, Cons(2, Nil))");
}

pub mod list_from {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_returns_list_from_array() {
        let arr: &[i32] = &[0, 1, 2, 3];
        assert_eq!(List::from(arr), ls![0, 1, 2, 3]);

        let arr: &[i32] = &[];
        assert_eq!(List::from(arr), ls![]);
    }

    #[test]
    fn it_returns_list_from_iterator() {
        let arr: &[i32] = &[0, 1, 2, 3];
        assert_eq!(List::from(arr.iter()), ls![0, 1, 2, 3]);

        let arr: &[i32] = &[];
        assert_eq!(List::from(arr.iter()), ls![]);
    }
}

pub mod ls_clone {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_clone_list_correctly() {
        let list = ls![0, 1, 2, 3];
        let newlist = list.clone();
        assert_eq!(list, newlist);
    }
}

pub mod ls_eq {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_returns_false_for_different_lengths() {
        let a = ls![5, 4, 3, 2, 1];
        let b = ls![1, 2];
        assert_eq!(a.eq(&b), false);
    }

    #[test]
    fn it_returns_true_if_same() {
        let a = ls![5, 4, 3, 2, 1];
        let b = ls![5, 4, 3, 2, 1];
        assert_eq!(a.eq(&b), true);
    }

    #[test]
    fn it_returns_false_if_different() {
        let a = ls![5, 4, 3, 2, 1];
        let b = ls![5, 8, 3, 1, 1];
        assert_eq!(a.eq(&b), false);
    }
}

pub mod ls_is_empty {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_returns_true_for_empty_list() {
        let empty: List<u8> = ls![];
        assert_eq!(empty.is_empty(), true);
        assert_eq!(empty.null(), true);
    }

    #[test]
    fn it_returns_false_for_nonempty_list() {
        assert_eq!(ls![1, 2].is_empty(), false);
        assert_eq!(ls![1, 2].null(), false);
    }
}

pub mod ls_get {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_returns_some_item_at_index_if_it_exists() {
        let list = ls![5, 4, 3, 2, 1];

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
        let list = ls![5, 4, 3, 2, 1];
        if list.get(5).is_some() {
            panic!("Expected None")
        }
    }
}

pub mod ls_head {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_returns_some_head_if_it_exists() {
        let list = ls![5, 4, 3, 2, 1];
        match list.head() {
            Some(n) => assert_eq!(*n, 5),
            None => panic!("Expected Some"),
        }
    }

    #[test]
    fn it_returns_none_if_it_doesnt_exists() {
        let list: List<i32> = ls![];
        if list.head().is_some() {
            panic!("Expected None")
        }
    }
}

pub mod ls_tail {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_returns_some_head_if_it_exists() {
        let list = ls![5, 4, 3, 2, 1];
        assert_eq!(list.tail(), ls![4, 3, 2, 1]);
    }

    #[test]
    fn it_returns_none_if_it_doesnt_exists() {
        let list: List<i32> = ls![];
        assert_eq!(list.tail(), ls![]);
    }
}

pub mod ls_prepend {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_prepends_an_item_to_an_empty_list() {
        let list: List<i32> = ls![];
        let newlist = list.clone().prepend(9);
        assert_eq!(newlist, ls![9]);
        assert_eq!(list, ls![]);
    }

    #[test]
    fn it_prepends_an_item_to_the_list() {
        let list = ls![2, 1];
        let newlist = list.clone().prepend(3);
        assert_eq!(newlist, ls![3, 2, 1]);
        assert_eq!(list, ls![2, 1]);
    }
}

pub mod ls_append {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_prepends_an_item_to_an_empty_list() {
        let list: List<i32> = ls![];
        let newlist = list.clone().append(9);
        assert_eq!(newlist, ls![9]);
        assert_eq!(list, ls![]);
    }

    #[test]
    fn it_prepends_an_item_to_the_list() {
        let list = ls![2, 1];
        let newlist = list.clone().append(3);
        assert_eq!(newlist, ls![2, 1, 3]);
        assert_eq!(list, ls![2, 1]);
    }
}

pub mod ls_concat {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_concat_two_empty_lists() {
        let xs: List<i32> = ls![];
        let ys: List<i32> = ls![];
        let newlist = xs.concat(ys);
        assert_eq!(ls![], newlist);
    }

    #[test]
    fn it_concat_empty_list_list() {
        let xs: List<i32> = ls![];
        let ys: List<i32> = ls![1, 2, 3];
        let newlist = xs.concat(ys);
        assert_eq!(ls![1, 2, 3], newlist);
    }

    #[test]
    fn it_concat_list_and_empty_list() {
        let xs: List<i32> = ls![1, 2, 3];
        let ys: List<i32> = ls![];
        let newlist = xs.concat(ys);
        assert_eq!(ls![1, 2, 3], newlist);
    }
    #[test]
    fn it_concat_two_lists() {
        let xs: List<i32> = ls![1, 2, 3];
        let ys: List<i32> = ls![4, 5, 6];
        let newlist = xs.concat(ys);
        assert_eq!(ls![1, 2, 3, 4, 5, 6], newlist);
    }
}

pub mod ls_map {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_maps_an_empty_list() {
        let list: List<i32> = ls![];
        let newlist = list.clone().map(|x| x * 2);
        assert_eq!(list, newlist);
    }

    #[test]
    fn it_maps_a_list() {
        let list = ls![8, 11];
        let newlist = list.clone().map(|x| x * 2);
        assert_ne!(list, newlist);
        assert_eq!(newlist, ls![16, 22]);
    }
}

pub mod ls_filter {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_filters_an_empty_list() {
        let list: List<i32> = ls![];
        let newlist = list.clone().filter(|x| x % 2 == 0);
        assert_eq!(list, newlist);
    }

    #[test]
    fn it_filters_out_a_list_of_even_numbers() {
        let list = ls![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let newlist = list.clone().filter(|x| x % 2 == 0);
        assert_ne!(list, newlist);
        assert_eq!(newlist, ls![2, 4, 6, 8, 10]);
    }
}

pub mod ls_foldl {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_sums_an_empty_list() {
        let list: List<i32> = ls![];
        let newlist = list.foldl(|a, b| a + b, 0);
        assert_eq!(newlist, 0);
    }

    #[test]
    fn it_sums_a_list() {
        let list = ls![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = list.foldl(|a, b| a + b, 0);
        assert_eq!(result, 55);
    }
}

pub mod ls_foldr {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_sums_a_list() {
        let list = ls![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = list.foldr(|a, b| a + b, 0);
        assert_eq!(result, 55);
    }
}

pub mod ls_reverse {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_reverses_list() {
        let list = ls![1, 2, 3, 4, 5];
        assert_eq!(ls![5, 4, 3, 2, 1], list.reverse());

        let list: List<u8> = ls![];
        assert_eq!(ls![], list.reverse())
    }
}

pub mod ls_qsort {
    use fp_collections::{list::List, ls};

    #[test]
    fn it_is_identity_for_sorted_list() {
        let list = ls![1, 2, 3, 4, 5];
        assert_eq!(list.clone().qsort(), list);
    }

    #[test]
    fn it_returns_empty_for_empty_list() {
        let list: List<u8> = ls![];
        assert_eq!(list.qsort(), ls![]);
    }

    #[test]
    fn it_sorts_list() {
        let list: List<u8> = ls![32, 99, 1, 200, 23, 6, 12, 12];
        assert_eq!(list.qsort(), ls![1, 6, 12, 12, 23, 32, 99, 200]);
    }
}
