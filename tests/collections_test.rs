#[cfg(test)]
mod tests {

    #[test]
    fn test_collections() {
        let x: [i32; 4] = [1, 2, 3, 4];

        let y: Vec<i32> = x
            .iter()
            .map(|&x| x + 1)
            .filter(|&x| x > 2)
            .take(2)
            .collect();

        assert_eq!(y, [3, 4]);

        // no change in the original
        assert_eq!(x, [1, 2, 3, 4])
    }

    #[test]
    fn test_maps() {
        use std::collections::HashMap;

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