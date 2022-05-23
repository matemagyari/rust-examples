#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    type IntFn = fn(i32) -> i32;

    #[test]
    fn test_closures() {
        //different ways to define the same thing
        let inc: fn(i32) -> i32 = |num| { num + 1 };
        let inc2 = |num: i32| { num + 1 };
        let inc3 = |num: i32| num + 1;

        fn inc4(i: i32) -> i32 { i + 1 }

        assert_eq!(inc(3), 4);
        assert_eq!(inc2(3), 4);
        assert_eq!(inc3(3), 4);
        assert_eq!(inc4(3), 4);
    }

    #[test]
    fn test_closures_capture_local_context() {

        let x = 5;

        let add_x = |num: i32| { num + x };

        // can't capture dynamic environment in a fn item
        // fn addX2(num: i32) -> i32 {
        //     num + x
        // }

        assert_eq!(add_x(3), 8);
    }

    #[test]
    fn test_pass_fn_as_parameter() {
        let inc: IntFn = |num| { num + 1 };

        //take function as parameter
        fn do_twice(f: IntFn, arg: i32) -> i32 {
            2 * f(arg)
        }

        assert_eq!(do_twice(inc, 3), 8);
    }

    #[test]
    fn test_return_fn() {

        fn returns_incrementer() -> Box<IntFn> {
            Box::new(|x: i32| x * 2)
        }

        let inc: Box<fn(i32) -> i32> = returns_incrementer();

        assert_eq!(inc(3), 6);
    }
}