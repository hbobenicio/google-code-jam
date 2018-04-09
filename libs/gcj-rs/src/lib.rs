pub mod io;

#[cfg(test)]
mod tests {

    use std::io;
    use io::Scanner;

    #[test]
    fn it_works() {
        let scanner = Scanner::new();
    }
}
