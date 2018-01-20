mod help {
    pub fn greet(name: &str) -> String {
        String::from("Hello, ") + name
    }
}

#[cfg(test)]
mod tests {
    use help;
    #[test]
    fn it_works() {
        assert_eq!(help::greet("drats"), "Hello, drats");
    }
}
