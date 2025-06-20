use std::env;
use std::fs;

use crate::shell::RedirectType;

pub fn parse_command_line(input: &str, redirect: &mut RedirectType, file:&mut Option<String>, is_pipe: &mut bool) -> Vec<String> {
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
                    if current_part == "|" { *is_pipe = true; }
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
        if current_part == "|" { *is_pipe = true; }
        parts.push(current_part);
    }

    let n = parts.len();
    if n >= 1 && (parts[n - 1] == ">" || parts[n - 1] == "1>") {
        parts.truncate(n-1);
        // *redirect = true;
        *redirect = RedirectType::None;
        return parts;
    }
    if n >= 2 && (parts[n - 2] == ">" || parts[n - 2] == "1>") {
        *redirect = RedirectType::StdOutToFile;
        *file = Some(parts[n - 1].clone());
        parts.truncate(n - 2);
        return parts;
    }

    if n >= 2 && parts[n - 2] == "2>" {
        *redirect = RedirectType::StdErrToFile;
        *file = Some(parts[n - 1].clone());
        parts.truncate(n - 2);
        return parts;
    }

    if n >= 2 && (parts[n - 2] == ">>" || parts[n - 2] == "1>>") {
        *redirect = RedirectType::AppendStdOutToFile;
        *file = Some(parts[n - 1].clone());
        parts.truncate(n - 2);
        return parts;
    }

    if n >= 1 && (parts[n - 1] == ">>" || parts[n - 1] == "1>>") {
        parts.truncate(n-1);
        // *redirect = true;
        *redirect = RedirectType::None;
        return parts;
    }

    if n >= 2 && parts[n - 2] == "2>>" {
        *redirect = RedirectType::AppendStdErrToFile;
        *file = Some(parts[n - 1].clone());
        parts.truncate(n - 2);
        return parts;
    }
    return parts;
}

pub fn get_executable() -> Vec<String> {

    let mut commands = Vec::new();
    if let Ok(path_var) = env::var("PATH") {
        for dir in env::split_paths(&path_var) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        // On Unix, check for executable permission; on Windows, check for .exe, .bat, etc.
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            if path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
                                if let Some(stem) = path.file_stem() {
                                    let cmd = stem.to_string_lossy().to_string();
                                    // print!("{cmd} ,");
                                    commands.push(cmd);
                                }
                            }
                        }
                        #[cfg(windows)]
                        {
                            if let Some(ext) = path.extension() {
                                if ext == "exe" || ext == "bat" || ext == "cmd" {
                                    use std::ffi::OsStr;

                                    if let Some(stem) = path.file_stem() {
                                        let cmd = stem.to_string_lossy().to_string();
                                        commands.push(cmd);
                                    }
                                    // print!("{} ,", );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return commands;
}
