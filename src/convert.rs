extern crate regex;
use regex::Regex;

// インデントに使う文字を指定する（現在は4タブ）
const INDENT: &str = "    ";

enum IndentMode {
    INC,
    DEC,
}

trait Handle {
    fn update(&mut self, mode: IndentMode) -> String;
    fn get(self) -> String;
}

impl Handle for usize {
    fn update(&mut self, mode: IndentMode) -> String {
        *self = match mode {
            IndentMode::INC => *self + 1,
            IndentMode::DEC => *self - 1,
        };
        INDENT.repeat(*self)
    }

    fn get(self) -> String {
        INDENT.repeat(self)
    }
}

// TODO: リファクタリングする（取りあえずは動くもの最優先する）
pub fn convert(code: String) -> Result<String, ()> {
    // 詰めて書かれたコードに対応するためにこれらのトークンを空白に変換しておく（仮）
    // 前処理として Linter を走らせるという手もあるかも（要検証）
    let code = code.replace("(", " ");
    let code = code.replace(")", " ");
    let code = code.replace("{", " ");
    let code = code.replace("}", " ");
    // DOMの前後に改行を入れる
    let reg = Regex::new(r">([^\n])").unwrap();
    let code = reg.replace_all(&code, ">\n$1");
    let reg = Regex::new(r"([^\n]+)</").unwrap();
    let code = reg.replace_all(&code, "$1\n</");
    // 前処理した上で分解する
    let mut tokens = code.split_whitespace();
    let mut code: String = String::new();
    let mut indents: usize = 0;

    loop {
        // token を進める
        let tok = tokens.next();
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
                    "render" => {
                        // return までの token を一気に無視（これで空白の有無に関わらず動くはず）
                        loop {
                            let tok = tokens.next();
                            if tok.unwrap().contains("return") { break; }
                        }

                        code = format!("{}\n{}return (", code, indents.get());
                        indents.update(IndentMode::INC);

                        loop {
                            match tokens.next() {
                                Some(value) => {
                                    if value.contains(">") {
                                        let mut dom_tokens = value.split(">");
                                        loop {
                                            let data = dom_tokens.next();
                                            match data {
                                                Some(data) => {
                                                    let mut data = data.split("<");
                                                    data.next().unwrap();
                                                    match data.next() {
                                                            Some(dom) => {
                                                                if dom.contains("/") {
                                                                    // 終了タグ
                                                                    code = format!("{}\n{}<{}>", code, indents.update(IndentMode::DEC), dom);
                                                                } else {
                                                                    // 開始タグ
                                                                    code = format!("{}\n{}<{}>", code, indents.get(), dom);
                                                                    indents.update(IndentMode::INC);
                                                                }
                                                            },
                                                            None => {},
                                                        };
                                                },
                                                None => break,
                                            };
                                        }
                                    } else {
                                        code = format!("{}\n{}{}", code, indents.get(), value);
                                    }
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
    fn test_case_render_only_with_whitespace() {
        let target = load_file("./test/render_only/cls_1.tsx");
        let answer = load_file("./test/render_only/fun_1.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_render_only_without_whitespace() {
        let target = load_file("./test/render_only/cls_2.tsx");
        let answer = load_file("./test/render_only/fun_2.tsx"); // fun_1 と全く同じ
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_multi_contents_with_whitespace() {
        let target = load_file("./test/multi_contents/cls_1.tsx");
        let answer = load_file("./test/multi_contents/fun_1.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_multi_contents_without_whitespace() {
        let target = load_file("./test/multi_contents/cls_2.tsx");
        let answer = load_file("./test/multi_contents/fun_2.tsx"); // fun_1 と全く同じ
        assert_eq!(convert(target.unwrap()), answer);
    }
}
