extern crate regex;
use regex::Regex;

// インデントに使う文字を指定する（現在は4タブ）
const INDENT: &str = "    ";

#[derive(PartialEq)]
enum IndentMode {
    INC,
    DEC,
}

trait Handle {
    fn set(&mut self, mode: IndentMode);
    fn get(self) -> String;
}

impl Handle for usize {
    fn set(&mut self, mode: IndentMode) {
        if mode == IndentMode::DEC && *self == 0 { return; }
        *self = match mode {
            IndentMode::INC => *self + 1,
            IndentMode::DEC => *self - 1,
        };
    }

    fn get(self) -> String {
        INDENT.repeat(self)
    }
}

// TODO: リファクタリングする（取りあえずは動くもの最優先する）
pub fn convert(code: String) -> Result<String, ()> {
    let code = code.replace(";\n", "\n");
    // 詰めて書かれた場合に対応して一部 token の前後に空白を挿入
    let code = code.replace("(", " ( ");
    let code = code.replace(")", " ) ");
    let code = code.replace("{", " { ");
    let code = code.replace("}", " } ");
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
        let token = tokens.next();
        // dbg!(tok);
        match token {
            Some(token) => {
                match token {
                    "class" => {
                        let name = tokens.next();
                        code = format!("{}const {} = (", code, name.expect("Unwrap component name"));
                        tokens.next(); // extends
                        let extend = tokens.next(); // 継承元
                        // TODO: 継承元のプロパティを捜査する?
                        // TODO: props の生成に必要?
                        code = format!("{}) => {}", code, "{");
                        indents.set(IndentMode::INC);
                    },
                    "render" => {
                        // return までの token を一気に無視（これで空白の有無に関わらず動くはず）
                        loop {
                            let tok = tokens.next();
                            if tok.unwrap().contains("return") { break; }
                        }

                        code = format!("{}\n{}return (", code, indents.get());
                        indents.set(IndentMode::INC);
                        // dbg!(&code);

                        loop {
                            match tokens.next() {
                                Some(value) => {
                                    if value == "(" { continue; }
                                    if value == ")" { break; }

                                    if value.contains("<") {
                                        let mut result = String::new();

                                        let mut dom_tokens = if value.contains(">") {
                                            value.split(">")
                                        } else {
                                            // 要素に attribute がある場合は > までの token を結合して分解
                                            let mut spacing_flag = false;

                                            loop {
                                                let token = tokens.next().expect("Unwrap token");
                                                dbg!(token);

                                                // 代入後には spacing を行う
                                                let space = if token.contains("\"") {
                                                    spacing_flag = !spacing_flag;
                                                    if !spacing_flag { " " } else { "" }
                                                } else { "" };

                                                result = format!("{}{}{}", result, token, space);
                                                if token.contains(">") { break; }
                                            }
                                            result = format!("{} {}", value, result);
                                            result.split(">")
                                        };

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
                                                                    indents.set(IndentMode::DEC);
                                                                    code = format!("{}\n{}<{}>", code, indents.get(), dom);
                                                                } else {
                                                                    // 開始タグ
                                                                    code = format!("{}\n{}<{}>", code, indents.get(), dom);
                                                                    indents.set(IndentMode::INC);
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

                        indents.set(IndentMode::DEC);
                        code = format!("{}\n{})", code, indents.get());
                        indents.set(IndentMode::DEC);
                        code = format!("{}\n{}{}", code, indents.get(), "}");
                    },
                    // Otherwise で method をキャッチする
                    token => {
                        // TODO: method 変換を実装する
                        if token == "{" || token == "}" { continue; }

                        dbg!("星宮とと begin");
                        // dbg!(tok);
                        dbg!(token);
                        dbg!("星宮とと end");

                        // TODO: 引数に対応させる
                        code = format!("{}\n{}const {} = () => {}", code, indents.get(), token, "{");
                        indents.set(IndentMode::INC);

                        // { までの token を無視
                        loop {
                            let tok = tokens.next();
                            if tok.unwrap().contains("{") { break; }
                        }

                        // } までの token を全てメソッドの中身にする
                        let mut bracket_depth: usize = 1;
                        let mut new_line: bool = true;

                        loop {
                            let token = tokens.next();
                            match token {
                                Some(token) => {
                                    if token == "{" {
                                        bracket_depth = bracket_depth + 1;
                                    }

                                    if token == "}" {
                                        bracket_depth = bracket_depth - 1;
                                        if bracket_depth == 0 {
                                            break;
                                        }
                                    }

                                    dbg!(token);
                                    code = match token {
                                        "(" | ")" => {
                                            new_line = token != "(";
                                            format!("{}{}", code, token)
                                        },

                                        _ => format!(
                                            "{}{}{}{}", code,
                                            if new_line { "\n" } else { "" },
                                            if new_line { indents.get() } else { String::new() },
                                            token
                                        ),
                                    };
                                },
                                None => {},
                            };
                        }

                        indents.set(IndentMode::DEC);
                        code = format!("{}\n{}{}", code, indents.get(), "}");
                        dbg!(&code);
                    },
                };
            },
            None => break,
        };
    }

    code = format!("{}\n", code);
    Ok(code)
}

// 全ての test case は結果が等しくなる
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

    // Single Content (no props, only render)

    #[test]
    fn test_case_single_content_with_whitespace() {
        let target = load_file("./test/single_content/case_1.tsx");
        let answer = load_file("./test/single_content/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_single_content_without_whitespace() {
        let target = load_file("./test/single_content/case_2.tsx");
        let answer = load_file("./test/single_content/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_single_content_insane_indents() {
        let target = load_file("./test/single_content/case_3.tsx");
        let answer = load_file("./test/single_content/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    // Multiple Contents (no props, only render)

    #[test]
    fn test_case_multi_contents_with_whitespace() {
        let target = load_file("./test/multi_contents/case_1.tsx");
        let answer = load_file("./test/multi_contents/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_multi_contents_without_whitespace() {
        let target = load_file("./test/multi_contents/case_2.tsx");
        let answer = load_file("./test/multi_contents/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_multi_contents_insane_indents() {
        let target = load_file("./test/multi_contents/case_3.tsx");
        let answer = load_file("./test/multi_contents/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    // Multiple Methods (no props, single content)

    #[test]
    fn test_case_multi_methods_with_whitespace() {
        let target = load_file("./test/multi_methods/case_1.tsx");
        let answer = load_file("./test/multi_methods/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_multi_methods_without_whitespace() {
        let target = load_file("./test/multi_methods/case_2.tsx");
        let answer = load_file("./test/multi_methods/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }

    #[test]
    fn test_case_multi_methods_insane_indents() {
        let target = load_file("./test/multi_methods/case_3.tsx");
        let answer = load_file("./test/multi_methods/result.tsx");
        assert_eq!(convert(target.unwrap()), answer);
    }
}
