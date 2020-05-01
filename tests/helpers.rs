
pub mod identity {
    use fp_collections::{helpers::*};
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_returns_what_it_gets() {
        let object = 23;
        assert_eq!(identity(object), object);

        let object = "hello";
        assert_eq!(identity(object), object);

        let object = &[1, 2, 3];
        assert_eq!(identity(object), object);

        let object = ls![1, 2, 3];
        assert_eq!(identity(object.clone()), object);
    }

    #[test]
    fn it_can_be_used_pointfree() {
        let list = ls![1, 2, 3];
        let result = list.clone().map(identity);
        assert_eq!(list, result);
    }
}

pub mod constant {
    use fp_collections::{helpers::*};
    use fp_collections::{list::{List}, ls};

    #[test]
    fn it_returns_a_getter() {
        let object = 23;
        assert_eq!(constant(object)(), object);

        let object = "hello";
        assert_eq!(constant(object)(), object);

        let object = &[1, 2, 3];
        assert_eq!(constant(object)(), object);

        let object = ls![1, 2, 3];
        assert_eq!(constant(object.clone())(), object);
    }

    // TODO: Add ability to use in a pointfree way
    // #[test]
    // fn it_can_be_used_pointfree() {
        // let list = ls![1, 2, 3];
        // let result = list.clone().map(constant(5));
        // assert_eq!(list, ls![5, 5, 5]);
    // }
}

pub mod compose2 {
    use fp_collections::{helpers::*};
    
    #[test]
    fn it_composes_same_types() {
        fn add5(x: i32) -> i32 { x + 5 }
        fn mul2(x: i32) -> i32 { x * 2 }

        let do_math = compose2(add5, mul2);

        assert_eq!(do_math(5), 15);
    }
}
