#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_enums() {
        enum Event {
            Login { user: i32, pw: i32 },
            Shutdown,
            Misc(i32),
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
        assert_eq!(translate(Event::Login { user: 1, pw: 2 }), 3);
        assert_eq!(translate(Event::Misc(5)), 5);
        assert_eq!(translate(Event::Misc(11)), 1000)
    }
}
