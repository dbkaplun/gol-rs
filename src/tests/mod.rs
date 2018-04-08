#![cfg(test)]

pub mod grid;
pub mod gridview;

pub fn trim_lines(s: &str) -> String {
    s.lines().fold(String::new(), |mut s, line| {
        let line = line.trim();
        if !line.is_empty() {
            if !s.is_empty() {
                s.push('\n');
            }
            s.push_str(line)
        }
        s
    })
}
