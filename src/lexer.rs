use crate::error::CalcError;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Identifier(String),

    Plus,
    Minus,
    Mul,
    Div,
    Pow,

    LParen,
    RParen,
    Factorial,

    End,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_ascii_digit() || c == '.' {
            let start = i;

            while i < chars.len()
                && (chars[i].is_ascii_digit() || chars[i] == '.')
            {
                i += 1;
            }

            let text: String = chars[start..i].iter().collect();

            let value = text
                .parse::<f64>()
                .map_err(|_| CalcError::InvalidNumber)?;

            tokens.push(Token::Number(value));
            continue;
        }

        if c.is_ascii_alphabetic() {
            let start = i;

            while i < chars.len() && chars[i].is_ascii_alphabetic() {
                i += 1;
            }

            let ident: String = chars[start..i].iter().collect();
            tokens.push(Token::Identifier(ident));
            continue;
        }

        let token = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' | '×' => Token::Mul,
            '/' | '÷' => Token::Div,
            '^' => Token::Pow,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '!' => Token::Factorial,
            _ => return Err(CalcError::InvalidCharacter(c)),
        };

        tokens.push(token);
        i += 1;
    }

    tokens.push(Token::End);
    Ok(tokens)
}
