use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Reading the file contents");

    let result = convert(contents);
    println!("{}", result.expect("Unwrap result"));
}

fn convert(code: String) -> Result<String, ()> {
    let mut tokens = code.split_whitespace();//.iter();
    let mut code = String::new();
    let mut indents = 0;
    // println!("tok {:?}", tokens.next());

    loop {
        let tok = tokens.next(); // token を進める
        match tok {
            Some(tok) => {
                match tok {
                    "class" => {
                        let name = tokens.next();
                        code = format!("{}\nconst {} = (", code, name.expect("Unwrap component name"));
                        tokens.next(); // extends
                        let extend = tokens.next(); // 継承元
                        // TODO: 継承元のプロパティを捜査する?
                        // TODO: props の生成に必要?
                        code = format!("{}) => {}", code, "{");
                        indents = indents + 1;
                    },
                    "render()" => {
                        tokens.next(); // {
                        tokens.next(); // return
                        tokens.next(); // (

                        let indent = "  ".repeat(indents); // 2タブ派なので \t は使わない! (素振り)
                        code = format!("{}\n{}return (", code, indent);
                        indents = indents + 1;

                        loop {
                            match tokens.next() {
                                Some(value) => {
                                    if value == ")" {
                                        break;
                                    }

                                    let indent = "  ".repeat(indents);
                                    code = format!("{}\n{}{}", code, indent, value);
                                },
                                None => break,
                            };
                        }

                        indents = indents - 1;
                        let indent = "  ".repeat(indents);
                        code = format!("{}\n{})", code, indent);

                        indents = indents - 1;
                        let indent = "  ".repeat(indents);
                        code = format!("{}\n{}{}", code, indent, "}");
                    },
                    _ => {},
                };
            },
            None => break,
        };
    }

    // Ok(String::new())
    Ok(code)
}
