#[macro_export]
macro_rules! stdin_line {
    () => {{
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        match stdin.read_line(&mut buffer) {
            Err(err) => eprintln!("{:#?}", err),
            _ => {}
        };
    }};
}
