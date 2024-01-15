//! Module for parsing the tokenized code into an AST tree according to the C17 standard.

use std::io::{Error, ErrorKind};

use crate::node::{Node, Span};
use crate::tokenizer::{Keyword, TokenType, Tokenizer};

#[derive(Debug)]
enum TypeSpecifier {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Bool,
    Complex,
}

#[derive(Debug)]
enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
    Atomic,
}

#[derive(Debug)]
enum Constant {
    Integer(usize), // TODO: Abstract this later into long, short integers
    Float(f32),     // TODO: Abstract this later into long, short floating point numbers
}

type StringLiteral = Vec<char>;

#[derive(Debug)]
enum Expression {
    // Primary Expression
    Identifier(Node<String>), // Should this be a &'a str?
    Constant(Node<Constant>),
    StringLiteral(Node<StringLiteral>),
    // TODO: Support more expressions
}

#[derive(Debug)]
enum FunctionSpecifier {
    Inline,
    NoReturn,
}

#[derive(Debug)]
enum DeclarationSpecifier {
    TypeSpecifier(Node<TypeSpecifier>),
    TypeQualifier(Node<TypeQualifier>),
    FunctionSpecifier(Node<FunctionSpecifier>),
    // TODO: Add alignment specifier, etc. according to C17 standard
}

#[derive(Debug)]
enum Declarator {
    FunctionDeclarator(Node<FunctionDeclarator>),
    IdentifierDeclarator(Node<String>), // Currently this is just equivalent to an Identifier (Arrays, Pointers, etc are not considered)
}

#[derive(Debug)]
struct FunctionParameter {
    // This struct has the same signature as that of the `Declaration` struct. Should they be merged under one name?
    specifiers: Vec<Node<DeclarationSpecifier>>,
    declarator: Node<Declarator>,
}

#[derive(Debug)]
struct FunctionDeclarator {
    identifier: String,
    parameters: Vec<Node<FunctionParameter>>,
}

#[derive(Debug)]
struct FunctionDefinition {
    specifiers: Vec<Node<DeclarationSpecifier>>,
    declarator: Node<Declarator>,
    body: Node<Statement>, // Function body can be one statement or a compound statement
}

#[derive(Debug)]
struct Declaration {
    // Function Declaration
    // int                      function(DeclarationSpecifier param1, DeclarationSpecifier param2);
    // ^^^                      ^^^
    // DeclarationSpecifier     Declarator

    // Variable Declaration
    // int                      variable;
    // ^^^                      ^^^
    // DeclarationSpecifier     Declarator
    specifiers: Vec<Node<DeclarationSpecifier>>,
    declarator: Node<Declarator>,
}

#[derive(Debug)]
enum ExternalDeclaration {
    FunctionDefinition(Node<FunctionDefinition>),
    Declaration(Node<Declaration>),
}

#[derive(Debug)]
enum BlockItem {
    Declaration(Node<Declaration>),
    Statement(Node<Statement>),
}

#[derive(Debug)]
enum Statement {
    ReturnStatement(Node<Expression>),
    Compound(Vec<Node<BlockItem>>),
}

// This is the topmost Node in the hierarchy of AST as it represents the entire file
#[derive(Debug)]
struct TranslationUnit {
    external_declarations: Vec<Node<ExternalDeclaration>>,
}

pub struct Parser<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
        Parser { tokenizer }
    }

    pub fn parse(&mut self) {
        let mut tranlation_unit = TranslationUnit {
            external_declarations: Vec::new(),
        };

        loop {
            match self.tokenizer.next_token() {
                Ok(Some(token)) => {
                    println!("{:?}", token);

                    // Do the parsing for the token here
                    match token {
                        // External Declaration
                        TokenType::Keyword(Keyword::Int | Keyword::Void) => {
                            // 1. Check till next token is an Identifier
                            // 2. Check if next token is `(`
                            // 3.     If Yes then
                            // 4.         Parse the parameters
                            // 5.         Check if next token is `{`
                            // 6.             If Yes then
                            // 7.                 It is a function definition
                            // 8.                 Parse the body of function till `}` is encountered
                            // 9.             If No then
                            // 10.                It is a function declaration
                            // 11.                Consume a semicolon
                            // 12.    If No then
                            // 13.        It must be a variable declaration
                            // 14.        Consume a semicolon
                            // 15. Push back an External Declaration Node to Translation Unit
                        }

                        TokenType::Keyword(Keyword::Return) => {
                            let expression = self
                                .parse_expr()
                                .unwrap_or_else(|err| panic!("Expected expression: {}", err));

                            let statement: Node<Statement> =
                                Node::new(Statement::ReturnStatement(expression), Span::none());

                            self.accept_semicolon();

                            println!("Successfully parsed Return Statement: {:?}", statement);
                        }
                        _ => panic!("Unexpected token: {:?}", token),
                    }
                }
                Ok(None) => break,
                Err(error) => panic!("{}", error),
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Node<Expression>, Error> {
        match self.tokenizer.next_token() {
            Ok(Some(token)) => match token {
                TokenType::Integer(integer) => Ok(Node::new(
                    Expression::Constant(Node::new(Constant::Integer(integer), Span::none())),
                    Span::none(),
                )),
                _ => Err(Error::from(ErrorKind::UnexpectedEof)),
            },
            _ => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }

    fn accept_semicolon(&mut self) {
        match self.tokenizer.next_token() {
            Ok(Some(token)) => match token {
                TokenType::Semicolon => {}
                _ => panic!("Expected semicolon!"),
            },
            _ => panic!("Expected semicolon!"),
        }
    }
}
