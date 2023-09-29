use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::iter::Peekable;

fn main() {
    println!("1) Tokenize a string in the program.");
    println!("2) Tokenize a string in the file.");
    print!("Enter a number: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(1) => {
            let lexer = Lexer::from_iter("f(   a   ) { test = abc }".chars());
            for token in lexer {
                println!("{token:?}");
            }
        }
        Ok(2) => {
            let file = File::open("input.test").unwrap();
            let buf_reader = BufReader::new(file);
            let lexer = Lexer::from_iter(Reader {
                content: buf_reader,
            });
            for token in lexer {
                println!("{token:?}");
            }
        }
        _ => {
            println!("Invalid input");
            return;
        }
    }
}

struct Lexer<Chars: Iterator<Item = char>> {
    chars: Peekable<Chars>,
}

impl<Chars: Iterator<Item = char>> Lexer<Chars> {
    fn from_iter(chars: Chars) -> Self {
        Self {
            chars: chars.peekable(),
        }
    }
}

#[derive(Debug)]
struct Reader {
    content: BufReader<File>,
}

impl Iterator for Reader {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let mut buffer: [u8; 1] = [0; 1];
        match self.content.read_exact(&mut buffer) {
            Ok(_) => {
                // NOTE: EOF
                if buffer[0] == 0 {
                    return None;
                }

                return buffer.iter().map(|&x| x as char).next();
            }
            _ => return None,
        }
    }
}

impl<Chars: Iterator<Item = char>> Iterator for Lexer<Chars> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // NOTE: equal
        // while let Some(_) = self.chars.next_if(|x| x.is_whitespace()) {}
        while let Some(n) = self.chars.peek() {
            if n.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }

        if let Some(n) = self.chars.next() {
            let mut text = String::new();
            text.push(n);

            match n {
                '(' => Some(Token {
                    kind: TokenKind::OpenPare,
                    text,
                }),
                ')' => Some(Token {
                    kind: TokenKind::ClosePare,
                    text,
                }),
                '=' => Some(Token {
                    kind: TokenKind::Equal,
                    text,
                }),
                ',' => Some(Token {
                    kind: TokenKind::Comma,
                    text,
                }),
                _ => {
                    while let Some(n) = self.chars.next_if(|x| x.is_alphabetic()) {
                        text.push(n);
                    }

                    Some(Token {
                        kind: TokenKind::Sym,
                        text,
                    })
                }
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    text: String,
}

#[derive(Debug)]
enum TokenKind {
    Sym,
    Equal,
    Comma,
    OpenPare,
    ClosePare,
}
