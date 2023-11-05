pub mod queue;
pub mod env;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn checker() -> String {
    env::ThirdParties::getValues().send_grid_key
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
