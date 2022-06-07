use super::Token;
use super::TokenType;
use super::FileManager;
use super::Location;

use crate::errors::*;

const SINGLE_CHAR_SYMBOLS: [&str; 17] = ["+", "-", "*", "/", ";", "(", ")", "{", "}", "=", ",", "<", ">", "!", "~", "*", "&"];
const ONLY_SINGLE_CHAR_SYMBOLS: [&str; 7] = [";", "(", ")", "{", "}", ",", "~"];
const ONLY_DOUBLE_CHAR_SYMBOLS: [&str; 9] = ["++", "--", "==", "+=", "-=", "*=", "/=", "<=", ">="];
const DOUBLE_CHAR_SYMBOLS: [&str; 14] = ["++", "--", "==", "+=", "-=", "*=", "/=", "<=", ">=", "<<", ">>", "<<=", ">>=", "!="];

/// Convert a string into a token_type
pub fn convert_to_token_type(s: String) -> TokenType
{
    if let Ok(number) = s.parse::<u64>()
    {
        TokenType::IntegerLiteral(number)
    }
    else if s.starts_with("#")
    {
        TokenType::PreprocessorDirective(s)
    }
    else if s.starts_with("\"") && s.ends_with("\"")
    {
        let l = s.len();
        TokenType::StringLiteral(s[1..l-1].to_string())
    }
    else if SINGLE_CHAR_SYMBOLS.contains(&s.as_str()) || DOUBLE_CHAR_SYMBOLS.contains(&s.as_str())
    {
        TokenType::Symbol(s)
    }
    else
    {
        TokenType::Identifier(s)
    }
}

/// Push a token to the result vector
pub fn push_token(current: &mut String, last_location: &mut Option<Location>, result: &mut Vec<Token>)
{
    if current.len() != 0 && last_location.is_some()
    {
        result.push(Token::construct(convert_to_token_type(current.clone()), last_location.take().unwrap()));
        *current = String::new();
    }
}

/// Tokenize a file into usable tokens
pub fn tokenize(file: &FileManager) -> CompilerResult<Vec<Token>>
{
    let mut result = Vec::new();

    trace!("Tokenizing {}", file.filename);

    let mut line_count = 1;
    let mut in_multiline_comment = false;
    // TODO: Add string and character tokenization
    // let mut in_string = false;
    // let mut in_character = false;

    for line in file.raw_text.lines()
    {
        let mut column_count = 0;
        let mut last_location: Option<Location> = None;
        let mut current = String::new();

        for c in line.chars()
        {
            column_count += 1;

            if in_multiline_comment
            {
                if c == '*' && current.len() == 0
                {
                    current += "*";
                }
                else if c == '/' && current == "*"
                {
                    current = String::new();
                    last_location = None;
                    in_multiline_comment = false;
                }
                else
                {
                    current = String::new();
                }
                continue;
            }

            if c == ' ' || c == '\t' || c == '\r' || c == '\n'
            {
                push_token(&mut current, &mut last_location, &mut result);
                continue;
            }
            else
            {
                let s = String::from(c);
                let s = s.as_str();

                let mut next_current = current.clone();
                next_current.push(c);

                if next_current == "/*"
                {
                    current = String::new();
                    last_location = None;
                    in_multiline_comment = true;
                    continue;
                }

                if next_current == "//"
                {
                    current = String::new();
                    break;
                }

                if ONLY_DOUBLE_CHAR_SYMBOLS.contains(&current.as_str()) || ONLY_SINGLE_CHAR_SYMBOLS.contains(&s) || 
                    (SINGLE_CHAR_SYMBOLS.contains(&current.as_str()) && !DOUBLE_CHAR_SYMBOLS.contains(&next_current.as_str())) || 
                    (DOUBLE_CHAR_SYMBOLS.contains(&current.as_str()) && !DOUBLE_CHAR_SYMBOLS.contains(&next_current.as_str()))
                {
                    push_token(&mut current, &mut last_location, &mut result);
                }

                if SINGLE_CHAR_SYMBOLS.contains(&s) && !SINGLE_CHAR_SYMBOLS.contains(&current.as_str().chars().last().map(|c| c.to_string()).unwrap_or(String::new()).as_str())
                {
                    push_token(&mut current, &mut last_location, &mut result);
                }

                if last_location.is_none()
                {
                    last_location = Some(file.location(line_count, column_count));
                }

                if ONLY_SINGLE_CHAR_SYMBOLS.contains(&s)
                {
                    push_token(&mut String::from(c), &mut last_location, &mut result)
                }
                else
                {
                    current.push(c);
                }
            }
        }

        push_token(&mut current, &mut last_location, &mut result);

        line_count += 1;
    }

    result.push(Token::eof(file.location(line_count, 1)));

    Ok(result)
}