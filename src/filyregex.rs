

#[derive(Debug, PartialEq)]
pub enum Command {
    Win(String),
    Explorer(),
    CopyWin(),
    Quit(),
    FocusLeft(),
    FocusRight(),
    RequestExit(),
    NoOp,
    Unknown
}

#[derive(Debug)]
enum Token {
    ID(String),
    Str(String),
    Extension(Box<Token>),
    Command(String),
    Glob,
    Pipe,
    Unknown,
    EOF
}

struct Lexer {
    src:String,
    index:usize,
    tokens:Box<Vec<Token>>
}

impl Lexer {
    
    pub fn new(src:String) -> Lexer {
        Lexer {
            src,
            index: 0,
            tokens: Box::new(Vec::new())
        }
    }

    fn curr_char(&self) -> char {        
        match self.src.chars().nth(self.index) {
            Some(c) => c,
            None => '\0'
        }

    }
    
    fn next(&mut self) {
        self.index += 1;
    }
    
    fn back(&mut self) {
        self.index -= 1;
    }

    fn skip_whitespace(&mut self) {
        while self.curr_char() == ' ' || self.curr_char() == '\t' || self.curr_char() == '\n' {
            self.next();
        }
    }
    fn parse_string(&mut self) -> String {
        let start_index = self.index;

        while self.curr_char() != '"' && self.curr_char() != '\0'{
            self.next();
        }
        
        let res = &self.src.clone()[start_index as usize ..self.index as usize];
        return String::from(res);
    }

    fn parse_ID(&mut self) -> String {
        let start_index = self.index;

        while (self.curr_char().is_alphabetic() || self.curr_char() == '_'  || self.curr_char().is_digit(10)) && self.curr_char() != '\0'{
            self.next();
        }
        
        let res = &self.src.clone()[start_index as usize ..self.index as usize];
        self.back();
        return String::from(res);
    }

    fn lex(&mut self) -> Token {
        match self.curr_char() {
            '>' => {
                return Token::Pipe;
            },
            '*' => {
                return Token::Glob;
            }
            '.' => {
                self.next();
                return Token::Extension(Box::new(self.lex()));
            }
            ':' => {
                self.next();
                return Token::Command(self.parse_ID());
            }
            ' ' | '\t' | '\n' => {
                self.skip_whitespace();
                return self.lex();
            }
            '"' => {
                self.next();
                return Token::Str(self.parse_string());
            }
            ch => {
               if ch.is_alphabetic() || ch.is_digit(10) || ch == '_' {
                   return Token::ID(self.parse_ID());
               } 

               return Token::Unknown;
            }
        }

    }
    
    pub fn run(src:String) -> Vec<Token> {
        let mut lexer = Lexer::new(src);
        
        while lexer.index < lexer.src.len() && lexer.curr_char() != '\0' {
            let tok = lexer.lex();
            lexer.tokens.push(tok);
            lexer.next();
        }

        *lexer.tokens
    }

}

fn eval_into_commands(tokens:Vec<Token>) -> Vec<Command> {
    tokens.into_iter().map(|t| {
        match t {
            Token::Command(c) => {
                match &c as &str {
                    "win" => {
                        Command::Win(String::from(c))
                    },
                    "q" => Command::Quit(),
                    "e" => Command::Explorer(),
                    "c" => Command::CopyWin(),
                    "lf" => Command::FocusLeft(),
                    "rf" => Command::FocusRight(),
                    "exit" => Command::RequestExit(),
                    _ => Command::Unknown
                }
            },

            _ => Command::Unknown
        }
    }).collect()
}

pub fn execute_fily_regex(curr_dir:Option<String>, src:String) -> Vec<Command> {
    let tokens = Lexer::run(src);
    eval_into_commands(tokens)
}

