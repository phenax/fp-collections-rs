
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
