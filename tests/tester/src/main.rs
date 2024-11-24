use rusted_webkit::html;

fn main() {
    let html_source = include_str!("../../hello-world/index.html");
    dbg!(html::lexer::lex_html(html_source));
}
