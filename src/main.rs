use std::fs;

//tokenization begins here 
#[derive(Debug)]
enum Token {
    DATA,
    INPUT,
    PROCESS,
    OUTPUT,
    END,
    ID(String),
    NUM(String),
    TRUE,
    FALSE,
    READ,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    VECTOR,
    NUMBER,
    REGRESSIONA,
    REGRESSIONB,
    MEAN,
    STDDEV,
    CORRELATION,
    STRING(String),
    Unknown,
}


fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            'a'..='z' => {
                let id = consume_while(&mut iter, |&c| c.is_ascii_alphabetic());
                match id.as_str() {
                    "data" => tokens.push(Token::DATA),
                    "input" => tokens.push(Token::INPUT),
                    "process" => tokens.push(Token::PROCESS),
                    "output" => tokens.push(Token::OUTPUT),
                    "end" => tokens.push(Token::END),
                    "true" => tokens.push(Token::TRUE),
                    "false" => tokens.push(Token::FALSE),
                    "read" => tokens.push(Token::READ),
                    "vector" => tokens.push(Token::VECTOR),
                    "number" => tokens.push(Token::NUMBER),
                    "regressiona" => tokens.push(Token::REGRESSIONA),
                    "regressionb" => tokens.push(Token::REGRESSIONB),
                    "mean" => tokens.push(Token::MEAN),
                    "stddev" => tokens.push(Token::STDDEV),
                    "correlation" => tokens.push(Token::CORRELATION),
                    _ => tokens.push(Token::ID(id)),
                }
            }
            '0'..='9' => {
                let num = consume_while(&mut iter, |&c| c.is_ascii_digit());
                tokens.push(Token::NUM(num));
            }
            '"' => {
                iter.next(); // consume the opening quote
                let string = consume_while(&mut iter, |&c| c != '"');
                iter.next(); // consume the closing quote
                tokens.push(Token::STRING(string));
            }
            ':' => {
                iter.next();
                tokens.push(Token::COLON);
            }
            ',' => {
                iter.next();
                tokens.push(Token::COMMA);
            }
            '.' => {
                iter.next();
                tokens.push(Token::PERIOD);
            }
            '(' => {
                iter.next();
                tokens.push(Token::LPAREN);
            }
            ')' => {
                iter.next();
                tokens.push(Token::RPAREN);
            }
            '=' => {
                iter.next();
                tokens.push(Token::ASSIGN);
            }
            ' ' | '\t' | '\n' | '\r' => {
                iter.next(); // Skip whitespace
            }
            _ => {
                iter.next();
                // Handle other characters or raise an error for unknown tokens
                tokens.push(Token::Unknown);
            }
        }
    }
    tokens
}

fn consume_while<F>(iter: &mut std::iter::Peekable<std::str::Chars>, condition: F) -> String
where
    F: Fn(&char) -> bool,
{
    let mut result = String::new();
    while let Some(&c) = iter.peek() {
        if condition(&c) {
            result.push(c);
            iter.next();
        } else {
            break;
        }
    }
    result
}

fn main() {
    
    let contents = fs::read_to_string("./test0.da")
    .expect("Something went wrong reading the file");
    //println!("\n{}", contents);
    let _tokens = tokenize(&contents);
    /*for token in _tokens {
        println!("{:?}", token);
    }*/
   println!("; Processing Input File input.da");
   println!("; Lexical analysis Passed");
   parse(_tokens);
   
}
//parsing 
fn parse (tokens: Vec <Token>) {
    let mut iter = tokens.iter().peekable();
    while let Some (token) = iter.next() {
        match token {
            Token::DATA=> {
                if let Some(&Token::COLON)=iter.next() {
                    loop {
                        if let Some(&Token::ID(_) | &Token::COLON | &Token::LPAREN | &Token::COMMA | &Token::STRING(_) | &Token::NUM(_) )=iter.next() {
                            if let Some(&Token::COLON | &Token::ID(_)| &Token::STRING(_)| &Token::COMMA | &Token::NUM(_) | &Token::ASSIGN | &Token::RPAREN)=iter.next() {
                                if let Some(&Token::VECTOR | &Token::NUMBER | &Token::ASSIGN | &Token::COMMA | &Token::RPAREN | &Token::READ | &Token::FALSE | &Token::TRUE | &Token::PROCESS ) =iter.next() {
                                    if let Some(&Token::COMMA | &Token::INPUT| &Token::READ| &Token::TRUE| &Token::FALSE | &Token::LPAREN | &Token::COLON) = iter.peek() {
                                                           
                               //      println!("Data lines Complete");
                                     iter.next();                  
                                    } else {              
                                        
                                        println!("(define xvalues (read-csv ./file.csv #f 0)) \n (define yvalues (read-csv ./file.csv #f 1)) \n GOT ALL the WAY TO PROCESS ");
                                        break;
                                    }
                                } else {
                                    println!("nop1");
                                    break;                
                                }
                            } else {
                                println!("nop2");
                                break;
                            }
                        } else {
                            println!("nop3");
                            break;
                        }
                    }
                } else {
                    println!{"Error in DATA"};
                }
                break;
            }
            Token::INPUT=> {
                if let Some(&Token::COLON)=iter.next() {
                    loop {
                        if let Some(&Token::ID(_))=iter.next() {
                            if let Some(&Token::ASSIGN)=iter.next() {
                                if let Some(&Token::READ)=iter.next() {
                                    if let Some(&Token::LPAREN)=iter.next() {
                                        if let Some(&Token::STRING(_))=iter.next() {
                                            if let Some(&Token::COMMA)=iter.next() {
                                                if let Some(&Token::FALSE | &Token::TRUE)=iter.next() {
                                                    if let Some(&Token::NUM(_))=iter.next() {
                                                        if let Some(&Token::RPAREN)=iter.next() {
                                                            if let Some(&Token::COMMA)=iter.next() {
                                                                println!("(define xvalues (read-csv ./file.csv #f 0))");
                                                            } else {
                                                                break;
                                                            }  
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                        }
                        break;
                    }
                } else {
                    println!{"Error in INPUT"};
                }

                break;
            }
            Token::PROCESS=> {
                if let Some(&Token::COLON)=iter.next() {
                    loop { 
                        if let Some(&Token::ID(_))=iter.next(){
                            if let Some(&Token::ASSIGN)=iter.next() {
                                if let Some(&Token::REGRESSIONA | &Token::REGRESSIONB) = iter.next() {
                                    if let Some(&Token::LPAREN)=iter.next() {
                                        if let Some(&Token::ID(_))=iter.next() {
                                            if let Some(&Token::COMMA)=iter.next() {
                                                if let Some(&Token:: ID(_))=iter.next(){
                                                    if let Some(&Token::RPAREN)=iter.next(){
                                                        if let Some(&Token::COMMA)=iter.next(){
                                                            println!("We got here3");
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }        
                                }
                            }
                        }
                        
                    }
                }
                else {
                    println!{"Error in PROCESS"};
                }
                break;
            }
            Token::OUTPUT=> {
                if let Some(&Token::COLON)=iter.next() {
                    loop {
                        
                    }
                } else {
                    println!{"Error in OUTPUT"};
                }
                break;
            }
            Token::END=> {
                if let Some(&Token::PERIOD) =iter.next() {
                    break;
                }
                break;
            }
            _ => {
                println!("Wrong token found: {:?}", token);
                break;
            }
        }   
      //  println!("infiniti");
    }
}

