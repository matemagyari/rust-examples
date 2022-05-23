#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ownership() {
        fn takes_ownership(some_string: String) { // some_string comes into scope
            println!("{}", some_string);
        } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

        fn makes_copy(some_integer: i32) { // some_integer comes into scope
            println!("{}", some_integer);
        } // Here, some_integer goes out of scope. Nothing special happens.

        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
        // ... and so is no longer valid here
        // println!("{}", s);

        let x = 5; // x comes into scope

        // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
        makes_copy(x);

        assert_eq!(5, x);
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.


    #[test]
    fn test_mutable_references() {
        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }

        let mut s = String::from("hello");
        change(&mut s);

        assert_eq!("hello, world", s)
    }

    #[test]
    fn test_mutable_references_only_one() {
        let mut s = String::from("hello");

        let r1 = &mut s;

        // not allowed
        // let r2 = &mut s;

        assert_eq!("hello, world", s)
    }

    #[test]
    fn test_references() {
        fn calculate_length_on_ref(s: &String) -> usize {
            s.len()
        }

        fn calculate_length(s: String) -> usize {
            s.len()
        }

        let s1 = String::from("hello");
        // only move reference
        let len = calculate_length_on_ref(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    #[test]
    fn test_moving_value() {
        let s1 = String::from("hello");
        // cannot borrow `s1` as mutable, as it is not declared as mutable
        // s1.push_str("");

        // value is moved to s2
        let mut s2 = s1;
        s2.push_str("bello");

        assert_eq!(s2, "hellobello");

        // value is moved to s2
        // assert_eq!(s1, "hellobello");
    }

    #[test]
    fn test_only_one_mutable_reference_allowed() {
        let s1 = String::from("hello");

        // value is moved to s2
        let mut s2 = s1;

        // use of moved value
        // let mut s3  = s1;
    }

    #[test]
    fn test_clone() {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        let s3 = s1.clone();

        assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
        assert_eq!(s3, "hello");
    }
}