//! Module for parsing the tokenized code into an AST tree according to the C17 standard.

use core::fmt;
use debug_tree::*;
use std::io::{Error, ErrorKind};

use crate::errors::CompilerError;
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
enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    ThreadLocal,
    Auto,
    Register,
}

#[derive(Debug)]
enum Constant {
    Integer(i64), // TODO: Abstract this later into long, short integers
    Float(f32),   // TODO: Abstract this later into long, short floating point numbers
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
    StorageClassSpecifier(StorageClassSpecifier),
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
    FunctionSpecifier(FunctionSpecifier),
    // TODO: Add alignment specifier, etc. according to C17 standard
}

#[derive(Debug)]
enum Declarator {
    FunctionDeclarator(FunctionDeclarator),
    DirectDeclarator(String), // Currently this is just equivalent to an Identifier (Arrays, Pointers, etc are not considered)
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
    declarator: Node<FunctionDeclarator>,
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
enum BlockItem {
    Declaration(Node<Declaration>),
    Statement(Node<Statement>),
}

#[derive(Debug)]
enum Statement {
    ReturnStatement(Node<Expression>),
    Compound(Vec<Node<BlockItem>>),
}

#[derive(Debug)]
enum ExternalDeclaration {
    FunctionDefinition(FunctionDefinition),
    Declaration(Declaration),
}

// Grammar for Translation Unit according to C17 ISO standard
//      translation-unit:
//           external-declaration
//           translation-unit external-declaration
//
//      external-declaration:
//           function-definition
//           declaration
// This is the topmost Node in the hierarchy of AST as it represents the entire file
#[derive(Debug)]
pub struct TranslationUnit {
    external_declarations: Vec<Node<ExternalDeclaration>>,
}

impl fmt::Display for DeclarationSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeclarationSpecifier::TypeQualifier(typequalifier) => {
                write!(f, "TypeQualifier -> {:?}", typequalifier)
            }
            DeclarationSpecifier::TypeSpecifier(typespec) => {
                write!(f, "TypeSpecifier -> {:?}", typespec)
            }
            DeclarationSpecifier::FunctionSpecifier(funcspec) => {
                write!(f, "FunctionSpecifier -> {:?}", funcspec)
            }
            DeclarationSpecifier::StorageClassSpecifier(storagespec) => {
                write!(f, "StorageClassSpecifier -> {:?}", storagespec)
            }
        }
    }
}

pub fn display_translationunit(tunit: &TranslationUnit) {
    defer_print!();
    add_branch!("TranslationUnit");
    for extdecl in &tunit.external_declarations {
        match &extdecl.node {
            ExternalDeclaration::Declaration(decl) => {
                add_branch!("Declaration {:?}", extdecl.span);
                // Add declaration
                add_branch!("DeclarationSpecifiers");
                for declspec in &decl.specifiers {
                    add_leaf!("{}", declspec);
                }
                // Add declarator
                match &decl.declarator.node {
                    Declarator::FunctionDeclarator(funcdecl) => {
                        add_branch!("FunctionDeclarator");
                        add_leaf!("Identifier -> \"{}\"", funcdecl.identifier);
                        // TODO: Add Parameters
                    }
                    Declarator::DirectDeclarator(identifier) => {
                        add_leaf!("DirectDeclarator -> \"{}\"", identifier);
                    }
                }
            }
            ExternalDeclaration::FunctionDefinition(funcdef) => {
                add_branch!("FunctionDefinition {:?}", extdecl.span);
            }
        }
    }
}

pub struct Parser<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
        Parser { tokenizer }
    }

    pub fn parse(&mut self) -> Result<TranslationUnit, CompilerError> {
        let mut tranlation_unit = TranslationUnit {
            external_declarations: Vec::new(),
        };

        loop {
            match self.tokenizer.peek_token()? {
                Some(token) => {
                    // An External Declaration can have:
                    //      1. Function Definition
                    //      2. Declaration

                    // 1. Parse a Declaration
                    let declaration = self.parse_declaration()?;

                    // Expect a semicolon (Function Definitions will be handled later)
                    self.accept_semicolon();

                    tranlation_unit.external_declarations.push(Node::new(
                        ExternalDeclaration::Declaration(declaration),
                        Span::none(),
                    ));

                    // 2. Check for Function Definition

                    // match token {
                    //     // External Declaration
                    //     TokenType::Keyword(Keyword::Int) => {
                    //         let typespecifier = TypeSpecifier::Int;

                    //         let declarator = self.parse_declarator();

                    //         // The next token is expected to be a declarator

                    //         // 1. Check till next token is an Identifier
                    //         // 2. Check if next token is `(`
                    //         // 3.     If Yes then
                    //         // 4.         Parse the parameters
                    //         // 5.         Check if next token is `{`
                    //         // 6.             If Yes then
                    //         // 7.                 It is a function definition
                    //         // 8.                 Parse the body of function till `}` is encountered
                    //         // 9.             If No then
                    //         // 10.                It is a function declaration
                    //         // 11.                Consume a semicolon
                    //         // 12.    If No then
                    //         // 13.        It must be a variable declaration
                    //         // 14.        Consume a semicolon
                    //         // 15. Push back an External Declaration Node to Translation Unit
                    //     }

                    //     TokenType::Keyword(Keyword::Return) => {
                    //         let expression = self
                    //             .parse_expr()
                    //             .unwrap_or_else(|err| panic!("Expected expression: {}", err));

                    //         let statement: Node<Statement> =
                    //             Node::new(Statement::ReturnStatement(expression), Span::none());

                    //         self.accept_semicolon();

                    //         println!("Successfully parsed Return Statement: {:?}", statement);
                    //     }
                    //     _ => panic!("Unexpected token: {:?}", token),
                    // }
                }
                None => break,
            }
        }
        // Return the entire translation unit AKA the root node of the parse tree
        Ok(tranlation_unit)
    }

    fn parse_declaration(&mut self) -> Result<Declaration, CompilerError> {
        // Grammar:
        //      declaration:
        //           declaration-specifiers init-declarator-listopt ;
        //           static_assert-declaration
        //      declaration-specifiers:
        //           storage-class-specifier declaration-specifiersopt
        //           type-specifier declaration-specifiersopt
        //           type-qualifier declaration-specifiersopt
        //           function-specifier declaration-specifiersopt
        //           alignment-specifier declaration-specifiersopt
        //      init-declarator-list:
        //           init-declarator
        //           init-declarator-list , init-declarator
        //      init-declarator:
        //           declarator
        //           declarator = initializer

        let mut specifiers: Vec<Node<DeclarationSpecifier>> = Vec::new();

        while let Ok(Some(token)) = self.tokenizer.peek_token() {
            match token {
                TokenType::Keyword(keyword) => {
                    match keyword {
                        // Storage Classifiers
                        Keyword::Auto => specifiers.push(Node::new(
                            DeclarationSpecifier::StorageClassSpecifier(
                                StorageClassSpecifier::Auto,
                            ),
                            Span::none(),
                        )),
                        Keyword::Register => specifiers.push(Node::new(
                            DeclarationSpecifier::StorageClassSpecifier(
                                StorageClassSpecifier::Register,
                            ),
                            Span::none(),
                        )),
                        Keyword::Extern => specifiers.push(Node::new(
                            DeclarationSpecifier::StorageClassSpecifier(
                                StorageClassSpecifier::Extern,
                            ),
                            Span::none(),
                        )),
                        Keyword::Static => specifiers.push(Node::new(
                            DeclarationSpecifier::StorageClassSpecifier(
                                StorageClassSpecifier::Static,
                            ),
                            Span::none(),
                        )),
                        Keyword::Typedef => specifiers.push(Node::new(
                            DeclarationSpecifier::StorageClassSpecifier(
                                StorageClassSpecifier::Typedef,
                            ),
                            Span::none(),
                        )),

                        // Type Qualifiers
                        Keyword::Const => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeQualifier(TypeQualifier::Const),
                            Span::none(),
                        )),
                        Keyword::Volatile => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeQualifier(TypeQualifier::Volatile),
                            Span::none(),
                        )),

                        // Type Specifiers
                        Keyword::Void => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Void),
                            Span::none(),
                        )),
                        Keyword::Char => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Char),
                            Span::none(),
                        )),
                        Keyword::Short => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Short),
                            Span::none(),
                        )),
                        Keyword::Int => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int),
                            Span::none(),
                        )),
                        Keyword::Long => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Long),
                            Span::none(),
                        )),
                        Keyword::Float => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Float),
                            Span::none(),
                        )),
                        Keyword::Double => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Double),
                            Span::none(),
                        )),
                        Keyword::Signed => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Signed),
                            Span::none(),
                        )),
                        Keyword::Unsigned => specifiers.push(Node::new(
                            DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Unsigned),
                            Span::none(),
                        )),

                        // Function Specifiers
                        Keyword::Inline => specifiers.push(Node::new(
                            DeclarationSpecifier::FunctionSpecifier(FunctionSpecifier::Inline),
                            Span::none(),
                        )),

                        // Unknown keyword present in declaration specifier
                        _ => {
                            return Err(CompilerError::SyntaxError(format!(
                                "Unexpected keyword: {:?}",
                                keyword
                            )))
                        }
                    }
                    // Consume the peeked token as it is a Declaration Specifier
                    self.tokenizer.next_token()?;
                }

                TokenType::Identifier(identifier) => {
                    // Once we hit an identifier (we should always hit one, if the program is syntactically right)
                    // Parse the declarator
                    if specifiers.len() != 0 {
                        let declarator = self.parse_declarator()?;
                        return Ok(Declaration {
                            specifiers,
                            declarator,
                        });
                    } else {
                        // This should happen when the program contains something like
                        // identifier() {}
                        // ^^ Missing Declaration Specifiers (like int, void, etc.)
                        return Err(CompilerError::SyntaxError(format!(
                            "Unexpected Identifer: `{}`, Expected a Declaration Specifier",
                            identifier
                        )));
                    }
                }

                // This is the case where the program doesn't contain any identifer
                // const void*  ;
                //            ^^ Missing Identifier
                _ => {
                    return Err(CompilerError::UnexpectedTokenError(format!(
                        "Unexpected Token: {:?}",
                        token
                    )))
                }
            }
        }
        // Reaching this line implies that the `TokenType::Identifier` was not encountered before all the tokens were consumed
        Err(CompilerError::SyntaxError(
            "Unexpected end of file".to_string(),
        ))
    }

    fn parse_parameters(&mut self) -> Result<Vec<Node<FunctionParameter>>, CompilerError> {
        debug_assert!(self.tokenizer.next_token()?.unwrap() == TokenType::OpenParenthesis);

        match self.tokenizer.next_token()? {
            Some(token) => match token {
                // Parse parameter list
                // Currently only function with no parameters are supported
                TokenType::CloseParenthesis => Ok(Vec::new()),
                _ => Err(CompilerError::UnexpectedTokenError(
                    "Only functions with no parameters are supported".to_string(),
                )),
            },
            None => Err(CompilerError::UnexpectedTokenError(
                "Missing `)` in the function declaration/definition".to_string(),
            )),
        }
    }

    fn parse_declarator(&mut self) -> Result<Node<Declarator>, CompilerError> {
        match self.tokenizer.next_token()? {
            Some(token) => match token {
                TokenType::Identifier(identifier) => {
                    // Decide whether it's a function declarator or a direct declarator
                    match self.tokenizer.peek_token()? {
                        Some(TokenType::OpenParenthesis) => {
                            // Parse the parameters
                            let parameters = self.parse_parameters()?; // TODO: Does this copy the entire vector? If yes find a way to avoid that
                            // Create the function declarator
                            let fdeclarator = FunctionDeclarator {
                                identifier,
                                parameters,
                            };
                            // Return the final function declarator node
                            Ok(Node::new(
                                Declarator::FunctionDeclarator(fdeclarator),
                                Span::none(),
                            ))
                        }
                        Some(TokenType::Semicolon) => Ok(Node::new(
                            Declarator::DirectDeclarator(identifier),
                            Span::none(),
                        )),
                        _ => Err(CompilerError::UnexpectedTokenError("Unexpected token, expected a `(` (Function Declarator), or `;` (Direct Declarator)".to_string())),
                    }
                }
                _ => panic!("Internal Error: Expected Identifier, but found no token!"),
            },
            None => panic!("Internal Error: Expected Identifier, but found no token!"),
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
