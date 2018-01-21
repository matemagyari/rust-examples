fn main() {
    println!("Hello world");

    {
        //local type inference
        let x = 12;
        let y: i32 = 12;
        assert_eq!(x, y);
    }

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

    //scopes
    {
        let x = {
            let p = 12.0;
            p + 43.2
        };
        assert_eq!(x, 55.2);
    }

    //shadowing
    {
        let x = 5;
        let x = 4;
        assert_eq!(x, 4);
    }

    //constants
    {
        const X: i32 = 5;
    }

    //tuples
    {
        let (x, y) = (1, "hi");
        assert_eq!(x, 1);

        let z = (1, "hi");

        assert_eq!(z.0, 1);
        assert_eq!(z.1, "hi");

        //deconstructing
        let (z1, z2) = z;

        assert_eq!(z1, 1);
        assert_eq!(z2, "hi");
    }

    //arrays
    {
        let x = [1, 2, 3];
        assert_eq!(x[0], 1)
    }

    //if expression
    {
        let x = if 1 > 2 {
            "hey"
        } else {
            "ho"
        };

        assert_eq!(x, "ho")
    }

    //for loops
    {
        let a = [1, 2, 3, 4];
        for number in a.iter() {
            println!("{}", number)
        }
    }

    closure_examples();
    map_examples();
    collection_examples();
    structure_examples();
    trait_examples();
    method_examples();
    enum_examples()
}

fn enum_examples() {

    enum Event {
        Login { user: i32, pw: i32 },
        Shutdown,
        Misc(i32)
    }

    fn translate(e: Event) -> i32 {
        match e {
            Event::Login { user, pw } => user + pw,
            Event::Shutdown => 0,
            Event::Misc(i) if i < 10 => i,
            _ => 1000
        }
    }

    assert_eq!(translate(Event::Shutdown), 0);
    assert_eq!(translate(Event::Login { user:1, pw:2}), 3);
    assert_eq!(translate(Event::Misc(5)), 5);
    assert_eq!(translate(Event::Misc(11)), 1000)

}

fn closure_examples() {
    {
        //different ways to define the same thing
        let inc: fn(i32) -> i32 = |num| { num + 1 };
        let inc2 = |num: i32| { num + 1 };
        let inc3 = |num: i32| num + 1;

        fn inc4(i: i32) -> i32 { i + 1 }

        assert_eq!(inc(3), 4);
        assert_eq!(inc2(3), 4);
        assert_eq!(inc3(3), 4);
        assert_eq!(inc4(3), 4);

        //todo

        type IntFn = fn(i32) -> i32;

        //take function as parameter
        fn do_twice(f: IntFn, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        assert_eq!(do_twice(inc, 3), 8);

        //return function
        fn returns_incrementer() -> Box<IntFn> {
            Box::new(|x| x + 1)
        }

        let inc6 = returns_incrementer();

        assert_eq!(inc6(3), 4);
    }
}

fn collection_examples() {
    let x = [1, 2, 3, 4];

    let y: Vec<i32> = x
        .iter()
        .map(|&x| x + 1)
        .filter(|&x| x > 2)
        .take(2)
        .collect();

    assert_eq!(y, [3, 4])
}

fn map_examples() {
    use std::collections::HashMap;

    {
        let mut m: HashMap<i32, i32> = HashMap::new();
        m.insert(1, 2);
        m.insert(2, 3);

        let option: Option<&i32> = m.get(&1);

        assert_eq!(option.unwrap(), &2);
        //no ide why, but it's not a reference type after map
        assert_eq!(option.map(|x| x + 1).unwrap(), 3);

        assert_eq!(option.unwrap_or(&0), &2);
        assert_eq!(m.get(&3).unwrap_or(&0), &0);
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

        let c = C{};
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

fn structure_examples() {
    {
        struct Person {
            name: String,
            age: i32,
        }

        let p1 = Person {
            name: String::from("joe"),
            age: 12,
        };

        //p1.age = 1; won't compile

        //struct update syntax

        {
            let p_updated = Person {
                ..p1
            };

            assert_eq!(p_updated.age, p1.age)
        }

        {
            let p_updated = Person {
                name: String::from("jack"),
                ..p1
            };

            assert_eq!(p_updated.age, p1.age)
        }
    }

    //add method to type
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 4, y: 3 };

        impl Point {
            fn len(&self) -> f64 {
                let s = (self.x * self.x + self.y * self.y);
                (s as f64).sqrt()
            }

            fn echo(&self, num: i32) -> i32 {
                num
            }
        }

        assert_eq!(p.len(), 5.0);
        assert_eq!(p.echo(10), 10)
    }
}