use sauce::lexer::Lexer; 

fn main() {

    let code = "grab vibe = 100 |> yell";

    println!("Sauce Lexer Debug\n");
    println!("Input: '{}'", code);

    let lexer = Lexer::new(code);

    for result in lexer {
        match result {
            Ok(spanned) => {
                println!(
                    " [{:?}..{:?}] {:?}", 
                    spanned.span.start, 
                    spanned.span.end, 
                    spanned.token
                );
            }
            Err(e) => {
                println!(" Error: {}", e);
            }
        }
    }
}