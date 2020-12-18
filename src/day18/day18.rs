#[derive(Debug)]
enum Token {
    Mul,
    Add,
    OpenParenthesis,
    CloseParenthesis,
    Num(i64),
}

fn find_closing_parenthesis(tokens: &[Token], at: usize) -> usize {
    let mut count = 0;

    for (idx, token) in tokens[at..].iter().enumerate() {
        match token {
            Token::OpenParenthesis => {
                count += 1;
            }
            Token::CloseParenthesis => {
                count -= 1;
                if count == 0 {
                    return at + idx;
                }
            }
            _ => {}
        }
    }
    unreachable!();
}

fn parse_token(token: &str) -> Token {
    match token {
        "(" => Token::OpenParenthesis,
        "+" => Token::Add,
        "*" => Token::Mul,
        ")" => Token::CloseParenthesis,
        _ => {
            let num = token.parse::<i64>().unwrap();
            Token::Num(num)
        }
    }
}

fn parse_tokens(line: &str) -> Vec<Token> {
    // this is just great.
    let line = line.replace("(", " ( ");
    let line = line.replace(")", " ) ");

    line.split_ascii_whitespace()
        .map(|token| parse_token(token))
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<Token>> {
    let mut res = Vec::new();
    for line in input.trim().lines() {
        res.push(parse_tokens(line));
    }
    res
}

fn evaluate(mut tokens: &[Token]) -> i64 {
    let (mut left_val, idx) = consume(tokens);
    tokens = &tokens[idx..];
    while tokens.len() > 0 {
        let idx = match tokens[0] {
            Token::Mul => {
                // consume tokens as soon as we can
                let (right_val, idx) = consume(&tokens[1..]);
                left_val *= right_val;
                idx + 1
            }
            Token::Add => {
                let (right_val, idx) = consume(&tokens[1..]);
                left_val += right_val;
                idx + 1
            }
            _ => {
                unreachable!();
            }
        };
        tokens = &tokens[idx..];
    }
    left_val
}

// returns (expression_value, tokens_read)
fn consume(tokens: &[Token]) -> (i64, usize) {
    match tokens[0] {
        Token::Mul => {
            unreachable!()
        }
        Token::Add => {
            unreachable!()
        }
        Token::OpenParenthesis => {
            // evaluate the inside of a parenthesis expression
            // and return (value, tokens_read)
            let close_index = find_closing_parenthesis(tokens, 0);
            let val = evaluate(&tokens[1..close_index]);
            (val, close_index + 1)
        }
        Token::CloseParenthesis => {
            unreachable!()
        }
        Token::Num(val) => (val, 1),
    }
}

// exact same code as above, except it evaluates right on mul
// instead of consuming the value
fn evaluate_priority(mut tokens: &[Token]) -> i64 {
    let (mut left_val, idx) = consume_priority(tokens);
    tokens = &tokens[idx..];
    while tokens.len() > 0 {
        let idx = match tokens[0] {
            Token::Mul => {
                // evaluate rest of tokens and multiply after
                // afterwards, the entire token stream is done
                let right_val = evaluate_priority(&tokens[1..]);
                left_val *= right_val;
                tokens.len()
            }
            Token::Add => {
                let (right_val, idx) = consume_priority(&tokens[1..]);
                left_val += right_val;
                idx + 1
            }
            _ => {
                unreachable!();
            }
        };
        tokens = &tokens[idx..];
    }
    left_val
}

fn consume_priority(tokens: &[Token]) -> (i64, usize) {
    match tokens[0] {
        Token::Mul => {
            unreachable!()
        }
        Token::Add => {
            unreachable!()
        }
        Token::OpenParenthesis => {
            let close_index = find_closing_parenthesis(tokens, 0);
            let val = evaluate_priority(&tokens[1..close_index]);
            (val, close_index + 1)
        }
        Token::CloseParenthesis => {
            unreachable!()
        }
        Token::Num(val) => (val, 1),
    }
}

pub fn part1(input: &str) -> i64 {
    let programs = parse_input(input);
    let mut res = 0;
    for tokens in programs {
        res += evaluate(&tokens);
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let programs = parse_input(input);
    let mut res = 0;
    for tokens in programs {
        res += evaluate_priority(&tokens);
    }
    res
}

mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 24650385570008);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 158183007916215);
    }
}
