use fp_collections::{list::{List}, ls};

#[test]
fn it_creates_list() {
    let list = ls!(1, 2);
    let debug_str = format!("{:?}", list);
    assert_eq!(debug_str, "Cons(1, Cons(2, Nil))");
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
        match newlist.head() {
            Some(n) => assert_eq!(*n, 9),
            None => panic!("Expected Some"),
        }
        match list.head() {
            Some(_) => panic!("Expected None"),
            None => (),
        }
    }

    #[test]
    fn it_prepends_an_item_to_the_list() {
        let list = ls!(2, 1);
        let newlist = list.clone().prepend(3);
        match newlist.head() {
            Some(n) => assert_eq!(*n, 3),
            None => panic!("Expected Some"),
        }
        match list.head() {
            Some(n) => assert_eq!(*n, 2),
            None => panic!("Expected Some"),
        }
    }
}
