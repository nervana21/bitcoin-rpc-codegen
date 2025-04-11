pub fn add(left: u64, right: u64) -> u64 { left + right }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod generator {
    pub mod code_generator;
}

pub mod parser {
    pub mod api_parser;
}

pub use generator::code_generator::*;
pub use parser::api_parser::*;
