#[derive(PartialEq, Debug)]
struct Shoe {
    размер: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_размер: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                размер: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                размер: 13,
                style: String::from("sandal"),
            },
            Shoe {
                размер: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    размер: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    размер: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
