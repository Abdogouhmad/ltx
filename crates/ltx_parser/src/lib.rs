type ParamsType = u64;
type ReturnType = ParamsType;

pub fn add(left: ParamsType, right: ParamsType) -> ReturnType {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
