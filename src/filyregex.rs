use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum Command {
    Win(String),
    Explorer(),
    CopyWin(),
    Quit(),
    FocusLeft(),
    FocusRight(),
    RequestExit(),
    Map(String, Token),
    NoOp,
    Unknown
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    ID(String),
    Str(String),
    Num(f32),
    Extension(Box<Token>),
    Command(String),
    Statement(Vec<Token>),
    Bind,
    Glob,
    Pipe,
    Unknown,
    EOF
}

impl Token {
    pub fn is_atomic(&self) -> bool {
        match self {
            Token::ID(_) | Token::Str(_) | Token::Num(_) | Token::Command(_) | Token::Statement(_) => true,
            _ => false
        }
    }

    pub fn get_string_value(&self) -> String {
        match self {
            Token::ID(value) | Token::Str(value) | Token::Command(value) => String::from(value),
            _ => {String::new()}
        }
    }

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
    
    fn parse_fn(&mut self, func:fn(char) -> bool) -> String {
        let start_index = self.index;

        while func(self.curr_char()) {
            self.next();
        }
        
        let res = &self.src.clone()[start_index as usize ..self.index as usize];
        return String::from(res);

    }
/*
    fn parse_string(&mut self) -> String {
        let start_index = self.index;

        while self.curr_char() != '"' && self.curr_char() != '\0'{
            self.next();
        }
        
        let res = &self.src.clone()[start_index as usize ..self.index as usize];
        return String::from(res);
    }
*/
/*    fn parse_ID(&mut self) -> String {
        let start_index = self.index;

        while (self.curr_char().is_alphabetic() || self.curr_char() == '_'  || self.curr_char().is_digit(10)) && self.curr_char() != '\0' {
            self.next();
        }
        
        let res = &self.src.clone()[start_index as usize ..self.index as usize];
        self.back();
        return String::from(res);
    }
*/
    fn parse_num(&mut self) -> f32 {
        let start_index = self.index;
        let mut is_float = false;

        while (self.curr_char().is_digit(10) || (self.curr_char() == '.' && !is_float))&& self.curr_char() != '\0'{

            if self.curr_char() == '.' {
                is_float = true;
            }

            self.next();
        }
        
        let res = &self.src.clone()[start_index as usize ..self.index as usize];
        self.back();
        return res.parse::<f32>().expect("not a float"); 
    }
    fn lex(&mut self) -> Token {
        self.skip_whitespace();
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
                   let res = self.parse_fn(|c| {
                       (c.is_alphabetic() || c == '_'  || c.is_digit(10)) && c != '\0'
                   });
                self.back();

                return Token::Command(res);
            }
            ' ' | '\t' | '\n' => {
                self.skip_whitespace();
                return self.lex();
            }
            '"' => {
                self.next();
                return Token::Str(self.parse_fn(|c| {
                    c != '"' && c != '\0'
                }));
                //return Token::Str(self.parse_string());
            }
            '|' => {
                self.next();
                let toks = Lexer::run(self.parse_fn(|c| {
                    c != '|' && c != '\0'
                }));
                return Token::Statement(toks);
            }
            '&' => {
                return Token::Bind;
            }
            '.' => {
                return Token::Num(self.parse_num());
            }
            ch => {
                //parse ID
               if ch.is_alphabetic() || ch.is_digit(10) || ch == '_' {
                   let res = Token::ID(self.parse_fn(|c| {
                       (c.is_alphabetic() || c == '_'  || c.is_digit(10)) && c != '\0'
                   }));
                   self.back();
                   return res;
               } 

               if ch.is_digit(10) {
                    return Token::Num(self.parse_num());
               }

               return Token::Unknown;
            }
        }

    }
    
    pub fn run(src:String) -> Vec<Token> {
        let mut lexer = Lexer::new(src);
        
        while lexer.index < lexer.src.len() && lexer.curr_char() != '\0' {
            let tok = lexer.lex();
            if tok != Token::Unknown{
                lexer.tokens.push(tok);
            }
            lexer.next();
        }
        lexer.tokens.push(Token::EOF);
        *lexer.tokens
    }

}

type NodeBranch<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct Node<T> where T: Clone{
    value: T,
    left: NodeBranch<T>,
    right: NodeBranch<T> 
}

impl<T: std::clone::Clone> Node<T> {
    pub fn new(value:T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None
        }
    }

    pub fn new_with_branches(value:T, left:NodeBranch<T>, right:NodeBranch<T>) -> Node<T>{
        Node {
            value,
            left,
            right
        }
    }

    pub fn package(self) -> Option<Box<Self>> {
        return Some(Box::new(self.clone()));
    }
}

trait UnwrapNode {
    fn unwrap(&self) -> Token;
}

impl UnwrapNode for Option<Box<Node<Token>>> {
    fn unwrap(&self) -> Token {
        return match self {
             Some(n) => n.value.clone(),
             None => Token::Unknown
        };

    }
}


struct Parser {
    tokens: Arc<[Token]>,
    index:usize,
    nodes: Vec<Node<Token>>,
    curr_token: Token
} 

impl Parser {
    pub fn new(tokens: Arc<[Token]>) -> Parser {
        let tok = if tokens.len() > 0 { tokens[0].clone() } else { Token::Unknown };

        Parser {
            tokens: tokens.clone().into(),
            index: 0,
            nodes: Vec::new(),
            curr_token: tok,
        }
    }

    fn step(&mut self) {
        self.index += 1;
        self.curr_token = if self.index < self.tokens.len() {
            self.tokens[self.index].clone()
        } else {
            Token::EOF
        };
    }

    fn factor(&mut self) -> Node<Token> {
        let token = self.curr_token.clone(); // Clone instead of borrowing
        match token {
            Token::Str(_) | Token::Command(_) | Token::ID(_) | Token::Num(_) | Token::Statement(_) | Token::Glob => {
                self.step();
                Node::new(token)
            }
            _ => {
                self.step();
                Node::new(Token::EOF)
            }
        }
    }

    fn term(&mut self) -> Node<Token> {
        let factor = self.factor(); // Get a factor node
        match self.curr_token {
            Token::Extension(_) => {
                Node::new_with_branches(self.curr_token.clone(), factor.package(), None)
            }
            _ => {
                factor
            }
        }

    }

    fn expr(&mut self) -> Node<Token> {
        let term = self.term(); // Get a term node
        match self.curr_token {
            Token::Pipe | Token::Bind => {
                let tok = self.curr_token.clone();
                self.step();
                Node::new_with_branches(tok, term.package(), self.expr().package()) 
            },
            _ => term, // Return term as it is
        }
    }

    pub fn run(tokens: Arc<[Token]>) -> Vec<Node<Token>>{
        let mut parser = Parser::new(tokens.clone());
        if tokens.len() <= 0 {
            return vec![];
        }
        while parser.curr_token != Token::EOF {
            let res = parser.expr();

            parser.nodes.push(res);
        }
        parser.nodes.clone()
    }
}
/*
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
*/

fn name_to_command(name:String, piped_value:String) -> Command {
    match &name as &str {
        "win" => {
            if piped_value != String::new(){
                return Command::Win(piped_value);
            }
            Command::Win(String::from(name))
        },
        "q" => Command::Quit(),
        "e" => Command::Explorer(),
        "c" => Command::CopyWin(),
        "lf" => Command::FocusLeft(),
        "rf" => Command::FocusRight(),
        "exit" => Command::RequestExit(),
         _ => Command::Unknown 
    }
}

fn eval_into_commands(nodes:Vec<Node<Token>>) -> Vec<Command> {
    nodes.into_iter().map(|n| {
        match n.value {
            Token::Command(name) => name_to_command(name, String::new()),
            Token::Pipe => {
                if !n.left.clone().unwrap().value.is_atomic(){
                    return Command::Unknown;
                }

                match n.right.unwrap().value {
                    Token::Command(name) => name_to_command(name, n.left.unwrap().value.get_string_value()),
                    _ => {Command::Unknown}
                }
            },
            _ => Command::Unknown
        }
    }).collect()
}

pub fn execute_fily_regex(curr_dir:Option<String>, src:String) -> Vec<Command> {
    let tokens = Lexer::run(src);
    let nodes = Parser::run(tokens.clone().into());
//    println!("{:#?}", tokens);
//    println!("{:#?}", nodes);
    eval_into_commands(nodes)
}

