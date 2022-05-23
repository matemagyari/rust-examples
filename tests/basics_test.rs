#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_for_loop() {
        let a = [1,2,3];
        let mut sum = 0;
        for element in a.iter() {
            sum +=element
        }
        assert_eq!(6, sum);
    }

    #[test]
    fn test_basics() {
        {
            //local type inference
            let x = 12;
            let y: i32 = 12;
            assert_eq!(x, y);
        }

        // blocks
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
            assert_eq!(x[0], 1);

            // array size if part of the type
            let mut y: [i32; 3] = [1,2,3];

            // won't compile - wrong size
            // y = [4,5];
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

        // mutability
        {
            struct Person {
                name: String,
                age: i32,
            }

            {
                let p: Person = Person {
                    name: String::from("joe"),
                    age: 12,
                };

                // cannot compile
                // p = Person { name: String::from("jane"), age: 5};

                // cannot compile
                //p.age = 1;
            }

            let mut p: Person = Person {
                name: String::from("joe"),
                age: 12,
            };

            // p itself can be changed
            p = Person {
                name: String::from("jack"),
                age: 11,
            };

            // the fields of the structs can be changed
            p.age = 15;

            assert_eq!("jack", p.name);
            assert_eq!(15, p.age);
        }
    }

}