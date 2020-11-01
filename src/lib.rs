mod list;
#[cfg(test)]
mod list_tests {
    use crate::list::List;
    #[test]
    fn is_empty_on_default() {
        let list:List<usize> = List::default();
        assert!(list.is_empty());
    }

    #[test]
    fn is_leaf() {
        let list = List::leaf(0);
        assert!(list.is_leaf());
    }

    #[test]
    fn push_to_default_is_leaf() {
        let mut list = List::default();
        list.push(0);
        assert!(list.is_leaf());
    }

    #[test]
    fn pop_from_leaf_is_empty() {
        let mut list:List<usize> = List::leaf(0);
        list.pop();
        assert!(list.is_empty());
    }

    #[test]
    fn push_pop_same_value() {
        let elem = 0usize;
        let mut list:List<usize> = List::default();
        list.push(elem);
        assert_eq!(list.pop(),Some(elem));
    }

    #[test]
    fn verify_depth_append_elems() {
        assert_eq!((List::default() >> 1 >> 2).depth(),2);
    }

    #[test]
    fn verify_depth_push_elems() {
        assert_eq!((List::default() << 1 << 2).depth(),2);
    }

    #[test]
    fn push_and_verify_last() {
        let list = List::leaf(0) << 1 << 2 << 3;
        assert_eq!(list.last().get_head(), Some(&0))
    }

    #[test]
    fn append_and_verify_last() {
        let list = List::leaf(0) >> 1 >> 2 >> 3;
        assert_eq!(list.last().get_head(), Some(&3))
    }

    #[test]
    fn tail_on_leaf_is_none() {
        assert!(List::leaf(0).get_tail().is_none());
    }

    #[test]
    fn tail_on_default_is_none() {
        let default:List<usize> = List::default();
        assert!(default.get_tail().is_none());
    }

    #[test]
    fn at_gives_expected_values() {
        let list = List::leaf(0) >> 1 >> 2 >> 3 >> 4;
        assert_eq!(list.at(0),&Some(0));
        assert_eq!(list.at(1),&Some(1));
        assert_eq!(list.at(2),&Some(2));
        assert_eq!(list.at(3),&Some(3));
        assert_eq!(list.at(4),&Some(4));
        assert!(list.at(5).is_none());
    }

    #[test]
    fn at_on_default_gives_none() {
        let default:List<usize> = List::default();
        assert!(default.at(0).is_none());
        assert!(default.at(1).is_none());
        assert!(default.at(2).is_none());
        assert!(default.at(3).is_none());
    }

    #[test]
    fn insert_at_puts_value_in_correct_pos() {
        let mut list = List::leaf(0usize) >> 10 >> 20 >> 30 >> 40;
        assert_eq!(list.at(0),&Some(0));
        assert_eq!(list.at(1),&Some(10));
        assert_eq!(list.at(2),&Some(20));
        assert_eq!(list.at(3),&Some(30));
        assert_eq!(list.at(4),&Some(40));
        assert!(list.at(5).is_none());
        list.insert_at(2, 15);
        assert_eq!(list.at(0),&Some(0));
        assert_eq!(list.at(1),&Some(10));
        assert_eq!(list.at(2),&Some(15));
        assert_eq!(list.at(3),&Some(20));
        assert_eq!(list.at(4),&Some(30));
        assert_eq!(list.at(5),&Some(40));
        assert!(list.at(6).is_none());
    }

    #[test]
    fn insert_at_the_0_appends() {
        let mut list = List::leaf(10usize) >> 20 >> 30;
        list.insert_at(0, 1);
        assert_eq!(list.at(0),&Some(1));
        assert_eq!(list.at(1),&Some(10));
        assert_eq!(list.at(2),&Some(20));
        assert_eq!(list.at(3),&Some(30));
        assert!(list.at(4).is_none());
    }

    #[test]
    fn insert_at_the_depth_pushes() {
        let mut list = List::leaf(10usize) >> 20 >> 30;
        list.insert_at(3, 40);
        assert_eq!(list.at(0),&Some(10));
        assert_eq!(list.at(1),&Some(20));
        assert_eq!(list.at(2),&Some(30));
        assert_eq!(list.at(3),&Some(40));
        assert!(list.at(4).is_none());
        list.insert_at(30, 50);
        assert_eq!(list.at(4),&Some(50));
        assert!(list.at(5).is_none());
    }

    #[test]
    fn split_works() {
        let mut orig = List::leaf(0usize) >> 1 >> 2 >> 3 >> 4 >> 5;
        assert_eq!(orig.depth(),6);
        let split = *orig.split(2).unwrap();
        assert_eq!(orig.depth(),2);
        assert_eq!(orig.at(0),&Some(0));
        assert_eq!(orig.at(1),&Some(1));
        assert!(orig.at(2).is_none());

        assert_eq!(split.depth(),4);
        assert_eq!(split.at(0),&Some(2));
        assert_eq!(split.at(1),&Some(3));
        assert_eq!(split.at(2),&Some(4));
        assert_eq!(split.at(3),&Some(5));
        assert!(split.at(4).is_none());
    }

    #[test]
    fn splice_works() {
        let mut orig = List::leaf(0usize) >> 1 >> 2;
        let other = List::leaf(3usize) >> 4 >> 5;
        assert_eq!(orig.depth(),3);
        assert!(orig.at(3).is_none());
        assert!(orig.at(4).is_none());
        assert!(orig.at(5).is_none());
        orig.splice(other);
        assert_eq!(orig.depth(),6);
        assert_eq!(orig.at(0),&Some(0));
        assert_eq!(orig.at(1),&Some(1));
        assert_eq!(orig.at(2),&Some(2));
        assert_eq!(orig.at(3),&Some(3));
        assert_eq!(orig.at(4),&Some(4));
        assert_eq!(orig.at(5),&Some(5));
    }

    #[test]
    fn splice_before_works() {
        let mut orig = List::leaf(0usize) >> 1 >> 2;
        let other = List::leaf(3usize) >> 4 >> 5;
        assert_eq!(orig.depth(),3);
        assert!(orig.at(3).is_none());
        assert!(orig.at(4).is_none());
        assert!(orig.at(5).is_none());
        orig.splice_before(other);
        assert_eq!(orig.depth(),6);
        assert_eq!(orig.at(0),&Some(3));
        assert_eq!(orig.at(1),&Some(4));
        assert_eq!(orig.at(2),&Some(5));
        assert_eq!(orig.at(3),&Some(0));
        assert_eq!(orig.at(4),&Some(1));
        assert_eq!(orig.at(5),&Some(2));
    }

    #[test]
    fn equality_works() {
        assert_eq!(List::leaf(0usize),List::leaf(0usize));
        assert_ne!(List::leaf(0usize),List::leaf(0usize) >> 1);
        assert_ne!(List::leaf(0usize),List::leaf(1usize));

        let left = List::default() >> 0 >> 1 >> 2 >> 3;
        let right = List::default() << 3 << 2 << 1 << 0;

        assert_eq!(left,right);

        let left = List::default() >> 0 >> 1 >> 2 >> 3 >> 4;
        let right = List::default() << 3 << 2 << 1 << 0;

        assert_ne!(left,right);

        let left = List::default() >> 0usize >> 1 >> 2 >> 3;
        let right = List::default() << 3 << 2 << 1 << 0;

        assert_eq!(left,right);

        let left = List::default() >> "H" >> "e" >> "l" >> "l" >> "o";
        let right = List::default() << "o" << "l" << "l" << "e" << "H";

        assert_eq!(left,right);
    }

    #[test]
    fn test_from_vec() {
        let vec = &vec![0,1,2,3];
        let list = List::from(vec.clone());
        assert_eq!(list.depth(),vec.len());
        assert_eq!(list.at(0).as_ref(),vec.get(0));
        assert_eq!(list.at(1).as_ref(),vec.get(1));
        assert_eq!(list.at(2).as_ref(),vec.get(2));
        assert_eq!(list.at(3).as_ref(),vec.get(3));
        assert_eq!(list.at(4).as_ref(),vec.get(4));
    }
}
