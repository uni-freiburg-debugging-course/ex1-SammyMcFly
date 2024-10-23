// use std::io::{Error, ErrorKind};
use std::{fs::read_to_string}; //fmt::format,
use clap::Parser;


#[derive(Debug, Clone)]
enum Token {
    ParenthesisL,
    ParenthesisR,
    Function(String),
    Space,
    Operator(String),
    Number(String),
}

impl Token {
    fn push(&mut self, character: char) {
        match self {
            Token::Function(s) => { s.push(character); },
            Token::Number(s) => { s.push(character); },
            _ => (),
        }
    }
    fn create_negative_number(number: char) -> Result<Self, String> {
        if number.is_digit(10) {
            let mut neg_number: String = '-'.to_string();
            neg_number.push(number);
            Ok(Token::Number(neg_number))
        } else {
            Err(String::from("Could not create Token::Number from char, which is not a number"))
        }
    }
}

#[derive(Debug)]
enum MathOperator {
    Addition,
    Subtraction,
    Multiplication,
}

impl MathOperator {
    fn from(s: &str) -> Result<Self, String> {
        match s {
            "+" => Ok(Self::Addition),
            "-" => Ok(Self::Subtraction),
            "*" => Ok(Self::Multiplication),
            _ => Err(format!("Math operator not supported or invalid")),
        }
    }
}

#[derive(Debug)]
enum ArgumentType {
    Number(i32),
    NestedOperation(Box<Operation>),
    None
}

impl ArgumentType {
    fn evaluate(&self) -> Result<i64, String> {
        match &self {
            Self::Number(n) => Ok(n.clone() as i64),
            Self::NestedOperation(op) => op.evaluate(),
            Self::None => Err(String::from("Called .evaluate() on ArgumentType::None")),
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: MathOperator,
    argument1: ArgumentType,
    argument2: ArgumentType,
}

impl Operation {
    fn build_operation_tree(operation_by_tokens: &[Token]) -> Result<Self, String> {
        // determine indices of first and second argument
        let start_index_arg1: usize = 3;
        let mut start_index_arg2: usize = 0;
        let mut end_index_arg1: usize = 0;
        let end_index_arg2: usize = operation_by_tokens.len()-2;

        let mut level_of_nesting: usize = 0;
        let mut number_of_spaces: usize = 0;
        for (i, token) in operation_by_tokens.into_iter().enumerate() {
            match token {
                Token::ParenthesisL => level_of_nesting += 1,
                Token::ParenthesisR => level_of_nesting -= 1,
                Token::Space => {
                    if level_of_nesting == 1 {
                        number_of_spaces += 1;
                    }
                },
                _ => (),
            }
            if (level_of_nesting == 1) && (number_of_spaces == 2) {
                end_index_arg1 = i-1;
                start_index_arg2 = i+1;
                break;
            } else if (level_of_nesting == 0) && (number_of_spaces == 1) && (end_index_arg1 == 0) {
                end_index_arg1 = i-1;
                break;
            }
        }

        let operator: MathOperator;
        if let Token::Operator(op) = &operation_by_tokens[1] {
            operator = MathOperator::from(op)?;
        } else {
            return Err(String::from("Could not find operator in expected position"));
        }

        let argument1: ArgumentType;
        match &operation_by_tokens[start_index_arg1] {
            Token::ParenthesisL => {
                argument1 = ArgumentType::NestedOperation(Box::new(Self::build_operation_tree(
                    &operation_by_tokens[start_index_arg1..=end_index_arg1])?));
            },
            Token::Number(n) => {
                argument1 = ArgumentType::Number(n.clone().parse::<i32>().unwrap());
            },
            _ => { argument1 = ArgumentType::None; },
        }

        let argument2: ArgumentType;
        if start_index_arg2 == 0 {
            argument2 = ArgumentType::None;
        } else {
            match &operation_by_tokens[start_index_arg2] {
                Token::ParenthesisL => {
                    argument2 = ArgumentType::NestedOperation(Box::new(Self::build_operation_tree(
                        &operation_by_tokens[start_index_arg2..=end_index_arg2])?));
                },
                Token::Number(n) => {
                    argument2 = ArgumentType::Number(n.clone().parse::<i32>().unwrap());
                },
                _ => { argument2 = ArgumentType::None; },
            }
        }
        // Recursively build up object tree representing the levels of operation
        Ok(Self { operator, argument1, argument2, })
    }

    fn evaluate(&self) -> Result<i64, String> {
        match self.operator {
            MathOperator::Addition => {
                match &self.argument1.evaluate() {
                    Ok(number1) => {
                        match &self.argument2.evaluate() {
                            Ok(number2) => Ok(number1 + number2),
                            Err(e) => Err(format!("Could not evaluate argument 2 in operation '+': {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Could not evaluate argument 1 in operation '+': {}", e)),
                }
            },
            MathOperator::Subtraction =>  {
                match &self.argument1.evaluate() {
                    Ok(number1) => {
                        match &self.argument2.evaluate() {
                            Ok(number2) => Ok(number1 - number2),
                            Err(_) => Ok(number1 * (-1)),
                        }
                    },
                    Err(e) => Err(format!("Could not evaluate argument 1 in operation '-': {}", e)),
                }
            },
            MathOperator::Multiplication => {
                match &self.argument1.evaluate() {
                    Ok(number1) => {
                        match &self.argument2.evaluate() {
                            Ok(number2) => Ok(number1 * number2),
                            Err(e) => Err(format!("Could not evaluate argument 2 in operation *: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Could not evaluate argument 1 in operation '*': {}", e)),
                }
            },
        }
    }
}

#[derive(Debug)]
enum SMTFunction {
    Simplify,
}

#[derive(Debug)]
struct SMTExpression {
    function: SMTFunction,
    operation_tree: Operation,
}

impl SMTExpression {
    fn execute(lisp_expr: &String) -> Result<String, String> {
        // execute a SMT expression according to the function and return the result
        let lexed_lisp_expr = Self::lex_lisp_expression(lisp_expr)?;

        let function_arguments;
        let function;
        (function, function_arguments)  = Self::separate_function_and_arguments(lexed_lisp_expr)?;

        let operation_tree;
        match &function {
            SMTFunction::Simplify => {
                operation_tree = Operation::build_operation_tree(&function_arguments)?;
            },

        }
        let smt_expr = Self { function, operation_tree };
        return smt_expr.evaluate_operation_tree();
    }

    fn lex_lisp_expression(s: &String) -> Result<Vec<Token>, String> {
        // extract characters from input string
        let input_characters: Vec<char> = s.chars().collect();

        let mut input_tokens: Vec<Token> = Vec::new();

        // convert characters from char Tokens
        for character in input_characters.into_iter() {
            if character.is_alphabetic() {
                if let Some(Token::Function(_)) = input_tokens.last() {
                    let index_of_last_element = input_tokens.len()-1;
                    input_tokens[index_of_last_element].push(character);
                } else {
                    input_tokens.push(Token::Function(character.to_string()));
                }
                continue;
            } else if character.is_digit(10) {
                if let Some(Token::Number(_)) = input_tokens.last() {
                    let index_of_last_element = input_tokens.len()-1;
                    input_tokens[index_of_last_element].push(character);
                } else if let Some(Token::Operator(o)) = input_tokens.last() {
                    if *o == '-'.to_string() {
                        let index_of_last_element = input_tokens.len()-1;
                        input_tokens[index_of_last_element] = Token::create_negative_number(character)?;
                    }
                } else {
                    input_tokens.push(Token::Number(character.to_string()));
                }
                continue;
            }
            match character {
                '(' => input_tokens.push(Token::ParenthesisL),
                ')' => input_tokens.push(Token::ParenthesisR),
                '+' | '-' | '*' => input_tokens.push(Token::Operator(character.to_string())),
                ' ' => input_tokens.push(Token::Space),
                _ => { return Err(String::from(format!("Could not create Token for symbol: {}", character))); },
            }
        }
        return Ok(input_tokens);
    }

    fn separate_function_and_arguments(lisp_expr_as_tokens: Vec<Token>) -> Result<(SMTFunction, Vec<Token>), String> {
        // extract function and arguments from lexed lisp-expression
        let function;
        match &lisp_expr_as_tokens[1] {
            Token::Function(f) => {
                match f.as_str() {
                    "simplify" => { function = SMTFunction::Simplify; },
                    _ => { return Err(format!("Function not supported")); },
                }
            },
            _ => {
                return Err(format!("Did not find (alphabetical) function at function position in SMT expression"));
            },
        }
        let function_arguments = Vec::from(&lisp_expr_as_tokens[3..lisp_expr_as_tokens.len()-1]);
        return Ok((function, function_arguments));
    }

    fn evaluate_operation_tree(&self) -> Result<String, String> {
        match &self.function {
            SMTFunction::Simplify => {
                let number = &self.operation_tree.evaluate()?;
                if *number < 0 {
                    Ok(String::from(format!("(- {})", number*(-1))))
                } else {
                    Ok(String::from(format!("{number}")))
                }
            },
        }
    }
}

/// Simple program to evaluate LISP-style SMT expressions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path to input file
    // #[arg(short, long)]
    file: String,

    // /// Evaluate single LISP-style SMT expression
    // #[arg(short, long)] //, default=String::from("(simplify (+ (* 2 5) (- (+ 23 (- 4 50)) 4)))"))]
    // command: String,
}


fn main() {
    let args = Args::parse();

    let mut input: Vec<_> = Vec::new();
    for line in read_to_string(args.file).unwrap().lines() {
        input.push(line.to_string())
    }
    for line in input {
        if line != "" {
            let result = match SMTExpression::execute(&line) {
                Ok(s) => s,
                Err(e) => e,
            };
            println!("{result}");
        }
    }

    // if args.command != Default::default() {
    //     let single_arg: String = args.command.clone();
    //     let result = SMTExpression::execute(&single_arg);
    //     println!("{result}");
    // }
}
