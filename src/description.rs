// i need to represend the field

pub fn instantialize_field() -> [[bool; 10]; 21] {
    let field = [[false; 10]; 21];
    println!("{:?}", field);
    field
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instantialize_field() {
        assert_eq!(instantialize_field(), [[false; 10]; 21]);
    }
}
