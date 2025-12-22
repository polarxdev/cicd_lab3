fn main() {
    println!("Hello from Rust CI in branch: {}", std::env::var("BRANCH_NAME").unwrap_or("unknown".to_string()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}