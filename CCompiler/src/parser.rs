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
    // This key difference between this struct and `Declaration` struct is that the `declarator` is Optional here
    specifiers: Vec<Node<DeclarationSpecifier>>,
    declarator: Option<Node<Declarator>>, // Th
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

/// Grammar for Translation Unit according to C17 ISO standard:
///
///      translation-unit:
///           external-declaration
///           translation-unit external-declaration
///
///      external-declaration:
///           function-definition
///           declaration
///
/// This is the topmost Node in the hierarchy of AST as it represents the entire file
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
                {
                    // Wrapped around brackets to ensure the tree indentation
                    add_branch!("DeclarationSpecifiers");
                    for declspec in &decl.specifiers {
                        add_leaf!("{}", declspec);
                    }
                }
                // Add declarator
                match &decl.declarator.node {
                    Declarator::FunctionDeclarator(funcdecl) => {
                        add_branch!("FunctionDeclarator");
                        add_leaf!("Identifier -> \"{}\"", funcdecl.identifier);

                        // Add Parameters
                        add_branch!("FunctionParameters");
                        for param in &funcdecl.parameters {
                            // Add FunctionParameter
                            add_branch!("FunctionParameter {:?}", param.span);
                            {
                                // Wrapped around brackets to ensure the tree indentation
                                add_branch!("DeclarationSpecifiers");
                                for declspec in &param.node.specifiers {
                                    add_leaf!("{}", declspec);
                                }
                            }

                            // Add Parameter Declarator
                            match &param.node.declarator {
                                Some(paramdecl) => match &paramdecl.node {
                                    Declarator::DirectDeclarator(paramidentifier) => {
                                        add_leaf!("DirectDeclarator -> \"{}\"", paramidentifier)
                                    }
                                    _ => panic!(
                                        "Parameter Declarator should not be Function Declarator!"
                                    ),
                                },
                                None => add_leaf!("DirectDeclarator -> None"),
                            }
                        }
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

/// Maps TokenType::Keyword -> DeclarationSpecifier
fn keyword2declspec(keyword: Keyword) -> Option<DeclarationSpecifier> {
    let declspec = match keyword {
        // Storage Classifiers
        Keyword::Auto => DeclarationSpecifier::StorageClassSpecifier(StorageClassSpecifier::Auto),
        Keyword::Register => {
            DeclarationSpecifier::StorageClassSpecifier(StorageClassSpecifier::Register)
        }
        Keyword::Extern => {
            DeclarationSpecifier::StorageClassSpecifier(StorageClassSpecifier::Extern)
        }
        Keyword::Static => {
            DeclarationSpecifier::StorageClassSpecifier(StorageClassSpecifier::Static)
        }
        Keyword::Typedef => {
            DeclarationSpecifier::StorageClassSpecifier(StorageClassSpecifier::Typedef)
        }

        // Type Qualifiers
        Keyword::Const => DeclarationSpecifier::TypeQualifier(TypeQualifier::Const),
        Keyword::Volatile => DeclarationSpecifier::TypeQualifier(TypeQualifier::Volatile),

        // Type Specifiers
        Keyword::Void => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Void),
        Keyword::Char => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Char),
        Keyword::Short => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Short),
        Keyword::Int => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int),
        Keyword::Long => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Long),
        Keyword::Float => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Float),
        Keyword::Double => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Double),
        Keyword::Signed => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Signed),
        Keyword::Unsigned => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Unsigned),

        // Function Specifiers
        Keyword::Inline => DeclarationSpecifier::FunctionSpecifier(FunctionSpecifier::Inline),

        // Unknown keyword present in declaration specifier
        _ => return None,
    };
    // Return the declaration specifier type
    Some(declspec)
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
                Some(_) => {
                    // An External Declaration can have:
                    //      1. Function Definition
                    //      2. Declaration

                    // 1. Parse a Declaration
                    let declaration = self.parse_declaration()?;

                    // Expect a semicolon (Function Definitions will be handled later)
                    self.accept_token(TokenType::Semicolon)?;

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

        while let Some(token) = self.tokenizer.peek_token()? {
            match token {
                // keyword.clone() is used to avoid borrowing issues while returning an error containing `keyword` information
                TokenType::Keyword(keyword) => match keyword2declspec(keyword.clone()) {
                    Some(declspec) => {
                        specifiers.push(Node::new(declspec, Span::none()));
                        // Consume the peeked token as it is a Declaration Specifier
                        self.tokenizer.next_token()?;
                    }
                    None => {
                        return Err(CompilerError::SyntaxError(format!(
                            "Unexpected keyword: {:?}",
                            keyword
                        )))
                    }
                },

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

    fn parse_declarator(&mut self) -> Result<Node<Declarator>, CompilerError> {
        match self.tokenizer.next_token()? {
            Some(token) => match token {
                TokenType::Identifier(identifier) => {
                    // Decide whether it's a function declarator or a direct declarator
                    match self.tokenizer.peek_token()? {
                        Some(TokenType::OpenParenthesis) => {
                            self.tokenizer.next_token()?; // Consume the OpenParenthesis
                            let parameters = self.parse_parameters()?; // TODO: Does this copy the entire vector? If yes find a way to avoid that
                            self.accept_token(TokenType::CloseParenthesis)?; // Consume the CloseParenthesis

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
                        Some(TokenType::Semicolon | TokenType::Comma) => Ok(Node::new(
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

    fn parse_parameters(&mut self) -> Result<Vec<Node<FunctionParameter>>, CompilerError> {
        match self.tokenizer.peek_token()? {
            Some(token) => match token {
                // If we reach this line of code, then the function declarator contains no parameters
                // void function()
                //              ^^ Empty parameter list
                TokenType::CloseParenthesis => Ok(Vec::new()),

                // Parse the parameter list
                _ => {
                    let mut parameters = Vec::new();

                    let mut expect_parameter = false;

                    while self.tokenizer.peek_token()? != Some(TokenType::CloseParenthesis) {
                        let parameterdecl = self.parse_parameter_decl()?;
                        parameters.push(parameterdecl);

                        // Handle accepting a comma here
                        // No need to handle the case where next token is not , or )
                        // As the parse_parameter_decl() exits only when it encounters one of the above two tokens
                        // In case of next token being None, This function will return the parameters but the calling function will expect a )
                        // And then propagate an error, as it's not this function's responsibility to parse ) which is a part of the Declarator and not the Parameter List
                        if self.tokenizer.peek_token()? == Some(TokenType::Comma) {
                            self.tokenizer.next_token()?;
                            expect_parameter = true;
                        } else {
                            expect_parameter = false;
                        }
                    }

                    // This is to handle a case where the below C code should not be considered valid
                    // void function(const float param1, )
                    //                                  ^^ Missing parameter
                    // I.e. when a comma is consumed, but the next token is ) then the while loop will exit and return parameters successfully
                    // But that is not valid C syntax, a comma cannot be present if no parameter is present after it
                    if expect_parameter {
                        return Err(CompilerError::SyntaxError(
                            "Expected type specifier for parameter declaration after `,` instead got `)`".to_string(),
                        ));
                    }
                    // Return the parameters
                    Ok(parameters)
                }
            },
            // This line will be reached when the file ends abruptly with a half function declaration
            // void function(
            //              ^^ End of file
            None => Err(CompilerError::UnexpectedTokenError(
                "Missing `)` in the function declaration/definition".to_string(),
            )),
        }
    }

    fn parse_parameter_decl(&mut self) -> Result<Node<FunctionParameter>, CompilerError> {
        let mut specifiers: Vec<Node<DeclarationSpecifier>> = Vec::new();
        while let Some(token) = self.tokenizer.peek_token()? {
            match token {
                // keyword.clone() is used to avoid borrowing issues while returning an error containing `keyword` information
                TokenType::Keyword(keyword) => match keyword2declspec(keyword.clone()) {
                    Some(declspec) => {
                        // Push back the declaration specifiers
                        specifiers.push(Node::new(declspec, Span::none()));

                        // Consume the peeked token as it is a Declaration Specifier
                        self.tokenizer.next_token()?;
                    }
                    None => {
                        return Err(CompilerError::SyntaxError(format!(
                            "Unexpected keyword: {:?}",
                            keyword
                        )))
                    }
                },
                TokenType::Identifier(identifier) => {
                    // Once we hit an identifier
                    // Parse the expected direct declarator (Function pointers will be handled in the future)
                    if specifiers.len() != 0 {
                        // Currently we only support DirectDeclarators in parameter declaration
                        let declarator =
                            Node::new(Declarator::DirectDeclarator(identifier), Span::none());
                        // Create the FunctionParameter with the Identifier as we hit an Identifier
                        let parameter = FunctionParameter {
                            specifiers,
                            declarator: Some(declarator),
                        };

                        // Consume the identifier token as it is a part of the parameter declaration
                        self.tokenizer.next_token()?;

                        // Return the function parameter node
                        return Ok(Node::new(parameter, Span::none()));
                    } else {
                        // This should happen when the program contains something like
                        // function(param1, const float param2)
                        //          ^^ Missing Declaration Specifiers (like int, void, etc.)
                        return Err(CompilerError::SyntaxError(format!(
                            "Unexpected Identifer: `{}`, Expected a Declaration Specifier",
                            identifier
                        )));
                    }
                }
                TokenType::Comma | TokenType::CloseParenthesis => {
                    // This line will be reached when no identifier has been reached yet
                    // But the next token seems to be either a , or ) which is the end of the parameter declaration
                    if specifiers.len() != 0 {
                        // Instead of reaching TokenType::Identifier we have reached a parameter end
                        // Hence make a FunctionParameter with no declarator
                        // This is the key difference between a normal declaration and a function parameter declaration
                        let parameter = FunctionParameter {
                            specifiers,
                            declarator: None,
                        };
                        return Ok(Node::new(parameter, Span::none()));
                    } else {
                        // This line is reached when the C code should look something like:
                        // void function(, const float param2)
                        //               ^^ Missing parameter declaration
                        return Err(CompilerError::SyntaxError(
                            "Expected a type specifier for parameter declaration, instead found: `,` or `)`"
                                .to_string(),
                        ));
                    }
                }
                _ => break,
            }
        }
        // This line will be reached when neither a keyword, identifier, nor a , or ) are encountered
        // Or there are suddenly no tokens to parse
        Err(CompilerError::SyntaxError(
            "Unexpected end of file".to_string(),
        ))
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

    fn accept_token_if<F>(&mut self, mut predicate: F) -> Result<(), CompilerError>
    where
        F: FnMut(TokenType) -> bool,
    {
        match self.tokenizer.next_token()? {
            Some(token) => {
                // Token is cloned to avoid borrowing
                if predicate(token.clone()) {
                    Ok(())
                } else {
                    Err(CompilerError::UnexpectedTokenError(format!(
                        "Failed to accept token: {:?}, predicate failed",
                        token
                    )))
                }
            }
            None => Err(CompilerError::UnexpectedTokenError(
                "Expected a token".to_string(),
            )),
        }
    }

    fn accept_token(&mut self, tokentype: TokenType) -> Result<(), CompilerError> {
        match self.tokenizer.next_token()? {
            Some(token) => {
                if token == tokentype {
                    Ok(())
                } else {
                    Err(CompilerError::UnexpectedTokenError(format!(
                        "Expected token: {:?}, instead found: {:?}",
                        tokentype, token
                    )))
                }
            }
            None => Err(CompilerError::UnexpectedTokenError(format!(
                "Expected token: {:?}",
                tokentype
            ))),
        }
    }
}
