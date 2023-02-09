use std::fmt::*;

pub struct AsmString<'a> (pub &'a str);

impl<'a> Display for AsmString<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut buffer = String::new();
        let mut flag = false;

        buffer.push('"');
        for c in self.0.chars() {
            match c {
                '\n' => {
                    flag = true;
                    buffer.push_str("\",10");
                },
                _ if flag => {
                    flag = false;
                    buffer.push_str(&format!(",\"{c}"));
                },
                _ => buffer.push(c)
            }
        }

        if !flag {
            buffer.push('"');
        }

        buffer.push_str(", 0");

        f.write_str(&buffer)
    }
}
