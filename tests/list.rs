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
        match list.get(3) {
            Some(n) => assert_eq!(*n, 2),
            None => panic!("Expected Some"),
        }
    }

    #[test]
    fn it_returns_none_if_it_doesnt_exists() {
        let list = ls!(5, 4, 3, 2, 1);
        match list.get(8) {
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
