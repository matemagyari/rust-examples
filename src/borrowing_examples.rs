pub fn run() {


    struct Person { id: i32 };
    #[derive(Copy, Clone)]
    struct Person2 { id: i32 };

    fn method1(p: Person) {
        println!("{}", p.id)
    }
    fn method2(p: Person2) {
        println!("{}", p.id)
    }

    let p = Person { id: 1 };

    method1(p);
//    method1(p); //value used here after move

    let p2 = Person { id: 1 };
    method2(p2);
    method2(p2); //this one can be copied
}