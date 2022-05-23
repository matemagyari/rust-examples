#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct Person {
        name: String,
        age: i32,
    }

    #[test]
    fn test_basics() {
        let p1 = Person {
            name: String::from("joe"),
            age: 12,
        };
    }

    #[test]
    fn test_updates() {

        let p1 = Person {
            name: String::from("joe"),
            age: 12,
        };

        // p1.age = 1; // immutable, won't compile

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

    #[test]
    fn test_add_method() {
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