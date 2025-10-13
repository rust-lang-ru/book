#[derive(PartialEq, Debug)]
struct Shoe {
    размер: u32,
    вид: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, размер_обуви: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == размер_обуви).collect()
}

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                размер: 10,
                вид: String::from("sneaker"),
            },
            Shoe {
                размер: 13,
                вид: String::from("sandal"),
            },
            Shoe {
                размер: 10,
                вид: String::from("boot"),
            },
        ];

        let мой_размер = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_размер,
            vec![
                Shoe {
                    размер: 10,
                    вид: String::from("sneaker")
                },
                Shoe {
                    размер: 10,
                    вид: String::from("boot")
                },
            ]
        );
    }
}
