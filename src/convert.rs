enum IndentMode {
    INC,
    DEC,
    NONE,
}

trait Handle {
    fn update(&mut self, mode: IndentMode) -> String;
}

impl Handle for usize {
    fn update(&mut self, mode: IndentMode) -> String {
        *self = match mode {
            IndentMode::INC => *self + 1,
            IndentMode::DEC => *self - 1,
            IndentMode::NONE => *self,
        };
        "    ".repeat(*self)
    }
}

pub fn convert(code: String) -> Result<String, ()> {
    let mut tokens = code.split_whitespace();
    let mut code: String = String::new();
    let mut indents: usize = 0;

    indents.update(IndentMode::INC);
    indents.update(IndentMode::DEC);

    loop {
        let tok = tokens.next(); // token を進める
        match tok {
            Some(tok) => {
                match tok {
                    "class" => {
                        let name = tokens.next();
                        code = format!("{}const {} = (", code, name.expect("Unwrap component name"));
                        tokens.next(); // extends
                        let extend = tokens.next(); // 継承元
                        // TODO: 継承元のプロパティを捜査する?
                        // TODO: props の生成に必要?
                        code = format!("{}) => {}", code, "{");
                        indents.update(IndentMode::INC);
                    },
                    "render()" => {
                        tokens.next(); // {
                        tokens.next(); // return
                        tokens.next(); // (

                        code = format!("{}\n{}return (", code, indents.update(IndentMode::NONE));
                        indents.update(IndentMode::INC);

                        loop {
                            match tokens.next() {
                                Some(value) => {
                                    if value == ")" { break; }
                                    code = format!("{}\n{}{}", code, indents.update(IndentMode::NONE), value);
                                },
                                None => break,
                            };
                        }

                        code = format!("{}\n{})", code, indents.update(IndentMode::DEC));
                        code = format!("{}\n{}{}", code, indents.update(IndentMode::DEC), "}");
                    },
                    _ => {},
                };
            },
            None => break,
        };
    }

    code = format!("{}\n", code);
    Ok(code)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use super::*;

    fn load_file(path: impl AsRef<str>) -> Result<String, ()> {
        let mut file = File::open(path.as_ref()).expect("File not found");
        let mut contents: String = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(_) => Err(()),
        }
    }

    #[test]
    fn test_case_render_only() {
        let target = load_file("./test/cls_render_only.tsx");
        let answer = load_file("./test/fun_render_only.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }
}
