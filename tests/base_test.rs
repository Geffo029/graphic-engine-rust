use graphic_engine::{self, add, init};

#[test]
fn test_add() {
	assert_eq!(5, add(2, 3));
}

#[test]
fn complete_test() {
	assert_eq!(5, add(2, 3));
	assert_eq!(5, add(2, 3));
	assert_eq!(4, add(2, 3));
	assert_eq!(Ok(()), init());

}