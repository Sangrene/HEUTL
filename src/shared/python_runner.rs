use crate::shared::errors::Error;
use serde_json::Value;
use std::time::SystemTime;
use std::{
    process::{Command, Stdio},
    time::Duration,
};
use sysinfo::{Pid, System};

fn clean_python_script(script: String) -> String {
    let bytes: Vec<char> = script.chars().collect();
    let mut output = String::with_capacity(script.len());
    let mut i = 0usize;

    let mut in_single = false;
    let mut in_double = false;
    let mut escape_next = false;
    let mut at_line_start = true;

    while i < bytes.len() {
        let c = bytes[i];

        // Handle string state
        if in_single {
            output.push(c);
            if !escape_next && c == '\\' {
                escape_next = true;
            } else if !escape_next && c == '\'' {
                in_single = false;
            } else {
                escape_next = false;
            }
            if c == '\n' {
                at_line_start = true;
            } else {
                at_line_start = false;
            }
            i += 1;
            continue;
        }
        if in_double {
            output.push(c);
            if !escape_next && c == '\\' {
                escape_next = true;
            } else if !escape_next && c == '"' {
                in_double = false;
            } else {
                escape_next = false;
            }
            if c == '\n' {
                at_line_start = true;
            } else {
                at_line_start = false;
            }
            i += 1;
            continue;
        }

        // Not in a string here
        if c == '\'' {
            in_single = true;
            output.push(c);
            at_line_start = false;
            i += 1;
            continue;
        }
        if c == '"' {
            in_double = true;
            output.push(c);
            at_line_start = false;
            i += 1;
            continue;
        }

        // Detect and remove print(...) calls when not inside strings
        // Allow leading whitespace before the word print
        if c.is_whitespace() {
            output.push(c);
            at_line_start = c == '\n';
            i += 1;
            continue;
        }

        // If we encounter a comment start, copy until EOL
        if c == '#' {
            // copy rest of line
            while i < bytes.len() {
                let cc = bytes[i];
                output.push(cc);
                i += 1;
                if cc == '\n' {
                    at_line_start = true;
                    break;
                }
            }
            continue;
        }

        // Try to match "print" token
        if c == 'p'
            && i + 4 < bytes.len()
            && bytes[i..].iter().take(5).copied().collect::<String>() == "print"
        {
            let mut j = i + 5;
            // skip spaces
            while j < bytes.len() && bytes[j].is_whitespace() && bytes[j] != '\n' {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == '(' {
                // Found print with opening paren; skip entire call
                let mut k = j;
                let mut depth = 0i32;
                let mut arg_in_single = false;
                let mut arg_in_double = false;
                let mut arg_escape = false;
                while k < bytes.len() {
                    let ch = bytes[k];
                    if arg_in_single {
                        if !arg_escape && ch == '\\' {
                            arg_escape = true;
                        } else if !arg_escape && ch == '\'' {
                            arg_in_single = false;
                        } else {
                            arg_escape = false;
                        }
                    } else if arg_in_double {
                        if !arg_escape && ch == '\\' {
                            arg_escape = true;
                        } else if !arg_escape && ch == '"' {
                            arg_in_double = false;
                        } else {
                            arg_escape = false;
                        }
                    } else {
                        if ch == '\'' {
                            arg_in_single = true;
                        } else if ch == '"' {
                            arg_in_double = true;
                        } else if ch == '(' {
                            depth += 1;
                        } else if ch == ')' {
                            depth -= 1;
                            if depth == 0 {
                                k += 1;
                                break;
                            }
                        }
                    }
                    k += 1;
                }
                // Skip trailing spaces/semicolon and up to end of line (consume newline if present)
                while k < bytes.len() && (bytes[k].is_whitespace() && bytes[k] != '\n') {
                    k += 1;
                }
                if k < bytes.len() && bytes[k] == ';' {
                    k += 1;
                }
                while k < bytes.len() && bytes[k].is_whitespace() && bytes[k] != '\n' {
                    k += 1;
                }
                if k < bytes.len() && bytes[k] == '\n' {
                    k += 1;
                }
                // Set i to k (skipping the print call entirely)
                i = k;
                at_line_start = true;
                continue;
            }
        }

        // Default: copy character
        output.push(c);
        at_line_start = c == '\n';
        i += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_python_script_removes_simple_print() {
        let script = "print('hello')\nprint('world')".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_python_script_removes_print_with_complex_args() {
        let script = "x = 5\nprint(f'Value is {x}')\ny = 10".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "x = 5\ny = 10");
    }

    #[test]
    fn test_clean_python_script_removes_multiline_print() {
        let script = "print(\n    'hello',\n    'world'\n)\nprint('done')".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_python_script_preserves_print_in_strings() {
        let script = "text = 'print(\"hello\")'\nprint('world')".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "text = 'print(\"hello\")'\n");
    }

    #[test]
    fn test_clean_python_script_handles_nested_parentheses() {
        let script = "print(f'Result: {func(1, 2, 3)}')\nx = 5".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "x = 5");
    }

    #[test]
    fn test_clean_python_script_handles_escaped_quotes() {
        let script = "print('He said \\'hello\\'')\nprint(\"She said \\\"world\\\"\")".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_python_script_removes_print_with_semicolon() {
        let script = "print('hello'); print('world'); x = 5".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "x = 5");
    }

    #[test]
    fn test_clean_python_script_preserves_comments() {
        let script = "# This is a comment\nprint('hello')\n# Another comment".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "# This is a comment\n# Another comment");
    }

    #[test]
    fn test_clean_python_script_handles_empty_script() {
        let script = "".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_python_script_handles_no_print_statements() {
        let script = "x = 5\ny = 10\nz = x + y".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "x = 5\ny = 10\nz = x + y");
    }

    #[test]
    fn test_clean_python_script_handles_print_in_comments() {
        let script =
            "# print('this should stay')\nprint('this should go')\n# print('this too')".to_string();
        let result = clean_python_script(script);
        assert_eq!(result, "# print('this should stay')\n# print('this too')");
    }
}

fn limit_process_memory_and_time(pid: Pid, memory_limit: u64, time_limit: u64) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let start_time = SystemTime::now();

    while sys.process(pid).is_some() {
        if sys.process(pid).unwrap().memory() > memory_limit {
            println!("Process memory limit exceeded");
            sys.process(pid).unwrap().kill();
        }
        if start_time.elapsed().unwrap() > Duration::from_secs(time_limit) {
            println!("Process time limit exceeded");
            sys.process(pid).unwrap().kill();
        }
        sys.refresh_all();
    }
}

pub fn run_python_script(script: &String, input: &Value) -> Result<String, Error> {
    let script = clean_python_script(script.clone());
    let handle = Command::new("python/.venv/bin/python")
        .arg("python/container.py")
        .arg(format!("--script={}", script))
        .arg(format!("--input={}", input.to_string()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = Pid::from_u32(handle.id());

    limit_process_memory_and_time(pid, 1024 * 1024 * 1024, 30);

    let output = handle.wait_with_output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout)
        .to_string()
        .trim()
        .to_string();
    Ok(stdout)
}

pub fn run_python_script_output_json(script: &String, input: &Value) -> Result<Value, Error> {
    let result = run_python_script(script, input)?;
    let result = serde_json::from_str::<Value>(&result)?;
    Ok(result)
}
