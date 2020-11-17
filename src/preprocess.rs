extern crate regex;
use regex::Regex;

pub fn convert(code: impl AsRef<str>) -> String {
    let code = code.as_ref();

    // 行末のセミコロンを消す
    let code = code.replace(";\n", "\n");

    // 詰めて書かれた場合に対応して一部 token の前後に空白を挿入
    let reg = Regex::new(r"([\(\)\{\}])").unwrap();
    let code = reg.replace_all(&code, " $1 ");

    // DOMの前後に改行を入れる
    let reg = Regex::new(r">([^\n])").unwrap();
    let code = reg.replace_all(&code, ">\n$1");
    let reg = Regex::new(r"([^\n]+)</").unwrap();
    let code = reg.replace_all(&code, "$1\n</");

    code.to_string()
}
