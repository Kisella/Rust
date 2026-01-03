mod tests {
    #[macro_export]
    macro_rules! hashmap {
        ($($key:expr => $value:expr),*$(,)?) => {
            {
                let mut map = std::collections::HashMap::new();
                $(map.insert($key, $value);)*
                map
            }
        }
    }

    #[test]
    fn case_127() {
        let map = hashmap! {
            "blue" => 78,
            "red" => 99,
        };
        println!("{map:#?}")
    }
}
