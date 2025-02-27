use regex::Regex;
use std::io::{self, BufRead, Write};
use std::collections::HashSet;
use std::process;
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

fn main() {
    let re = Regex::new(r"^([\s\d]+)\s+(.*)$").unwrap();
    let mut stdout = io::stdout();
    let mut seen_lines = HashSet::new();

    // Load the syntax set and theme set
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // Choose the zsh syntax and a theme
    let syntax = ps.find_syntax_by_extension("zsh").unwrap();

    // Load theme
    let theme = ts.themes["base16-ocean.dark"].clone();

    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        if let Ok(line) = ln {
            if let Some(captures) = re.captures(&line) {
                let num = captures.get(1).map_or("", |m| m.as_str()).trim();
                let cmd = captures.get(2).map_or("", |m| m.as_str()).trim();

                if seen_lines.insert(cmd.to_string()) {
                    let mut h = HighlightLines::new(syntax, &theme); // reset the highlight instance for each line

                    for ll in LinesWithEndings::from(&cmd) {
                        let ranges: Vec<(Style, &str)> = h.highlight_line(ll, &ps).unwrap();
                        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                        let res = writeln!(stdout, "\x1b[38;2;{};{};{}m{}\x1b[0m\t{}", 70, 70, 90, num, escaped);
                        if let Err(_) = res {
                            process::exit(1);
                        }
                    }
                }
            }
        }
    }
}
