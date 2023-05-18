use std::iter::Peekable;

fn main() {
    let lexer = Lexer::from_iter("f(   a   ) { test = abc }".chars());

    for token in lexer {
        println!("{token:?}");
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

        if let Some(char) = self.chars.next() {
            let mut text = String::new();
            text.push(char);

            match char {
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
                    while let Some(w) = self.chars.next_if(|w| w.is_alphabetic()) {
                        text.push(w)
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
