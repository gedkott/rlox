use std::{
    env,
    fs::File,
    io::{stdin, stdout, Read, Result as IOResult, Write},
    process::exit,
};

fn main() -> IOResult<()> {
    let cli_args: Vec<String> = env::args().collect();
    let num_args = cli_args.len();
    if num_args > 2 {
        stdout().write_all(b"Usage: rlox [script]")?;
        exit(64)
    } else if num_args == 2 {
        let file_path = &cli_args[1];
        run_file(&file_path).map(|_| ())
    } else {
        run_prompt().map(|_| ())
    }
}

fn run_file(path: &str) -> IOResult<String> {
    File::open(path).and_then(|mut f| {
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        Ok(buf)
    })
}

fn run_prompt() -> IOResult<String> {
    loop {
        print!("> ");
        let mut line = String::new();
        stdout()
            .flush()
            .expect("could not flush stdout before reading user input in REPL");
        let num_bytes_read = stdin().read_line(&mut line)?;
        if num_bytes_read == 0 {
            break Ok("".to_string());
        }
        let user_inputted_code = line.trim().to_string();
        run(&user_inputted_code)?;
    }
}

fn run(code: &str) -> IOResult<()> {
    let scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        match token {
            Token::Useless(tfs) => {
                println!("{}", tfs)
            }
        }
    }
    Ok(())
}

enum Token<'token_from_source> {
    Useless(&'token_from_source str),
}

struct Scanner<'source_string> {
    source: &'source_string str,
}

impl<'source_string> Scanner<'source_string> {
    fn new(source: &'source_string str) -> Self {
        Self { source: source }
    }

    fn scan_tokens(&self) -> Vec<Token> {
        self.source
            .split_whitespace()
            .map(|word| Token::Useless(word))
            .collect()
    }
}
