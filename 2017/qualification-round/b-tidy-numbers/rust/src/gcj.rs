use std::io::Stdin;
use std::io;

/// Obtém um i32 a partir da entrada padrão.
/// 
/// # Exemplos
/// 
/// ```
/// use gcj::stdin_get_i32;
/// 
/// let meu_int = stdin_get_i32();
/// ```
pub fn stdin_get_i32(tmp_buff: &mut String) -> i32 {
    io::stdin()
        .read_line(tmp_buff)
        .unwrap();

    tmp_buff
        .trim()
        .parse::<i32>()
        .unwrap()
}

pub trait Scanner {
    fn get_i32(&self, tmp_buff: &mut String) -> i32;
}

impl Scanner for Stdin {
    fn get_i32(&self, tmp_buff: &mut String) -> i32 {
        io::stdin()
            .read_line(tmp_buff)
            .unwrap();

        tmp_buff
            .trim()
            .parse::<i32>()
            .unwrap()
    }
}
