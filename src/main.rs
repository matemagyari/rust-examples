
fn main() {
    println!("Hello world");


    trait_examples();
    method_examples();
    pattern_matching_examples();
    mutability_examples();
}

fn pattern_matching_examples() {

    //no pattern matching on structures!
    {
        trait A {}
        struct B { x: i32 }
        struct C { x: i32 }

        impl A for B {}
        impl A for C {}

        //todo

//        fn translate(a: &A) -> i32 {
//            match a {
//                B { x } => x,
//                C { x } => x + 1,
//            }
//        }
    }

    {
        trait A {
            fn num(&self) -> i32;
        }

        struct B { x: i32 }
        struct C { x: i32 }

        //todo

//        impl A for B {
//            fn num(&self) -> i32 {
//               self.x;
//            }
//        }
//        impl B for C {
//            fn num(&self) -> i32 {
//                self.x;
//            }
//        }
    }
}


fn mutability_examples() {
    //immutable values
    {
        let x = 1;
        //x = 10 //compiler error. x is immutable
    }

    //mutable values
    {
        let mut x = 1;
        x = 2;
    }

    struct Point { x: i32, y: i32 }

    fn change1(p: Point) {
//        p.x = 1 //cannot mutably borrow immutable field
//        p = Point { x: 1, y: 1}; //re-assignment
    }

    fn change2(mut p: Point) {
        p.x = 1;
        p = Point { x: 1, y: 1 };
    }

    fn change3(p: &Point) {
//        p.x = 1 //cannot mutably borrow immutable field
    }

    fn change4(p: &mut Point) {
        p.x = p.x + 1;
//        p = &mut Point { x: 1, y: 1}; //temporary value only lives until here
//        *p = Point { x: 1, y: 1};
    }

    fn change5(mut p: &mut Point) {
        p.x = p.x + 1;
        *p = Point { x: 1, y: 1 };
    }

    let p1 = Point { x: 0, y: 0 };
    change2(p1);

//    let y = p1.x; //use of moved value: `p1.x`

    let p2 = Point { x: 0, y: 0 };
    change3(&p2);
    let y = p2.x;

    {
        let mut p = &mut Point { x: 0, y: 0 };
        change4(p);
        assert_eq!(p.x, 1)
    }
    {
        let mut p = &mut Point { x: 0, y: 0 };
        change5(p);
        assert_eq!(p.x, 1)
    }
}

fn method_examples() {
    {
        struct A {}
        impl A {
            fn num(&self) -> i32 {
                1
            }
            //can't overload with different parameter list
//            fn num(&self, i: i32) -> i32 {
//
//            }
        }
    }
}

fn trait_examples() {

    //overriding
    {
        trait A {
            fn num(&self) -> i32 {
                1
            }
        }

        struct B {}

        impl A for B {
            fn num(&self) -> i32 {
                2
            }
        }

        let a = B {};
        assert_eq!(a.num(), 2)
    }

    //multiple inheritance
    {
        trait A {
            fn numA(&self) -> i32 {
                1
            }
        }
        trait B {
            fn numB(&self) -> i32 {
                2
            }
        }

        struct C {}

        impl A for C {}
        impl B for C {}

        let c = C {};
        c.numA();
        c.numB();
    }

    {
        trait Creature {
            fn name(&self) -> &'static str;
            fn weapon(&self) -> &'static str {
                "bare hands"
            }
        }

        struct Troll {
            name: &'static str,
        }

        impl Creature for Troll {
            fn name(&self) -> &'static str {
//            format!("Troll {}", self.name)
                self.name
            }

            fn weapon(&self) -> &'static str {
                "club"
            }
        }

        let t = Troll { name: "joe" };

        assert_eq!(t.name(), "joe");
        assert_eq!(t.weapon(), "club")
    }

    {
        trait A {
            fn do_sth(&self) {
                println!("hello")
            }
        }

        trait B: A {
            fn do_sth_else(&self) {
                println!("hello again")
            }
        }

        struct E {}

        impl A for E {} //this is a must. the next line is not enough
    impl B for E {}

        let e1 = E {};

        e1.do_sth();
        e1.do_sth_else();
    }

    {
        struct Wrapper {
            value: i32
        }

        trait ValueExtractor {
            fn value1() -> i32;
            fn value2(self) -> i32;
            fn value3(&self) -> i32;
        }

        impl ValueExtractor for Wrapper {
            fn value1() -> i32 {
                1
            }

            fn value2(self) -> i32 {
                self.value
            }

            fn value3(&self) -> i32 {
                self.value
            }
        }

        let v = Wrapper { value: 5 };
        assert_eq!(1, Wrapper::value1());
        assert_eq!(5, v.value2());
//        assert_eq!(5, v.value3()); // in the previous line it was moved
    }
}
