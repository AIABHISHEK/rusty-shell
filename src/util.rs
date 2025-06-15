use crate::shell::RedirectType;

pub fn parse_command_line(input: &str, redirect: &mut RedirectType, file:&mut Option<String>) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut chars = input.trim().chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            '\\' if !in_single_quotes && !in_double_quotes => {
                chars.next().map(|c| current_part.push(c));
                // continue;
            }
            '\\' if in_double_quotes => {
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '"' | '\\' | '$' | '`' => {
                            chars.next().map(|c| current_part.push(c));
                        }
                        _ => {
                            current_part.push('\\');
                        }
                    }
                }
            }
            ' ' | '\t' if !in_single_quotes && !in_double_quotes => {
                if !current_part.is_empty() {
                    parts.push(current_part.clone());
                    current_part.clear();
                }
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == ' ' || next_ch == '\t' {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            _ => {
                current_part.push(ch);
            }
        }
    }
    if !current_part.is_empty() {
        parts.push(current_part);
    }

    let n = parts.len();
    if n >= 1 && (parts[n - 1] == ">" || parts[n - 1] == "1>") {
        parts.truncate(n-1);
        // *redirect = true;
        return parts;
    }
    if n >= 2 && (parts[n - 2] == ">" || parts[n - 2] == "1>") {
        *redirect = RedirectType::StdErrToFile;
        *file = Some(parts[n - 1].clone());
        parts.truncate(n - 2);
        return parts;
    }

    if n >= 2 && parts[n - 2] == "2>" {
        *redirect = RedirectType::StdErrToFile;
        *file = Some(parts[n - 1].clone());
        parts.truncate(n - 2);
    }
    return parts;
}
