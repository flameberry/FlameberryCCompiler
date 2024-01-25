//! Module for parsing the tokenized code into an AST tree according to the C17 standard.

use core::fmt;
use debug_tree::*;

use crate::errors::{CompilerError, CompilerErrorKind};
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

// TODO: Make this debug mode only to save performance in release mode
pub fn display_translationunit(tunit: &TranslationUnit) {
    defer_print!();
    add_branch!("TranslationUnit");
    for extdecl in &tunit.external_declarations {
        match &extdecl.node {
            ExternalDeclaration::Declaration(decl) => {
                add_branch!("Declaration {}", extdecl.span);
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
                        add_leaf!(
                            "Identifier -> \"{}\" {}",
                            funcdecl.identifier,
                            Span::new(
                                decl.declarator.span.start,
                                decl.declarator.span.start + funcdecl.identifier.len()
                            )
                        );

                        // Add Parameters
                        add_branch!("FunctionParameters");
                        for param in &funcdecl.parameters {
                            // Add FunctionParameter
                            add_branch!("FunctionParameter {}", param.span);
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
                                        add_leaf!(
                                            "DirectDeclarator -> \"{}\" {}",
                                            paramidentifier,
                                            paramdecl.span
                                        )
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
                        add_leaf!(
                            "DirectDeclarator -> \"{}\" {}",
                            identifier,
                            decl.declarator.span
                        );
                    }
                }
            }
            ExternalDeclaration::FunctionDefinition(funcdef) => {
                add_branch!("FunctionDefinition {}", extdecl.span);
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

pub struct SyntaxAnalyzer<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
}

impl<'a> SyntaxAnalyzer<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
        SyntaxAnalyzer { tokenizer }
    }

    /// The main parse function that uses the tokenizer to generate an Abstract Syntax Tree
    pub fn parse(&mut self) -> Result<TranslationUnit, CompilerError> {
        let mut tranlation_unit = TranslationUnit {
            external_declarations: Vec::new(),
        };

        loop {
            match self.tokenizer.peek_token()? {
                Some(_) => {
                    // 1. Parse a Declaration
                    let declaration = self.parse_declaration()?;

                    // Expect a semicolon (Function Definitions will be handled later)
                    self.accept_token(TokenType::Semicolon)?;

                    // Calculate the span for the declaration
                    let declspan_start = declaration.specifiers[0].span.start; // Here we assume that a declaration will always have atleast one specifier
                    let declspan = Span::new(declspan_start, declaration.declarator.span.end);

                    tranlation_unit.external_declarations.push(Node::new(
                        ExternalDeclaration::Declaration(declaration),
                        declspan,
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

    /// Parses a C declaration
    fn parse_declaration(&mut self) -> Result<Declaration, CompilerError> {
        let mut specifiers: Vec<Node<DeclarationSpecifier>> = Vec::new();

        while let Some((token, start, end)) = self.tokenizer.peek_token()? {
            match token {
                // keyword.clone() is used to avoid borrowing issues while returning an error containing `keyword` information
                TokenType::Keyword(keyword) => match keyword2declspec(keyword.clone()) {
                    Some(declspec) => {
                        specifiers.push(Node::new(declspec, Span::new(start, end)));
                        // Consume the peeked token as it is a Declaration Specifier
                        self.tokenizer.next_token()?;
                    }
                    None => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SyntaxError,
                            message: format!("Unexpected keyword: {:?}", keyword),
                            location: Some(start),
                        })
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
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SyntaxError,
                            message: format!(
                                "Unexpected Identifer: `{}`, Expected a Declaration Specifier",
                                identifier
                            ),
                            location: Some(start),
                        });
                    }
                }

                // This is the case where the program doesn't contain any identifer
                // const void*  ;
                //            ^^ Missing Identifier
                _ => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: format!("Unexpected Token: {:?}", token),
                        location: Some(start),
                    })
                }
            }
        }
        // Reaching this line implies that the `TokenType::Identifier` was not encountered before all the tokens were consumed
        Err(CompilerError {
            kind: CompilerErrorKind::SyntaxError,
            message: "Unexpected end of file".to_string(),
            location: None,
        })
    }

    fn parse_declarator(&mut self) -> Result<Node<Declarator>, CompilerError> {
        match self.tokenizer.next_token()? {
            Some((token, start, end)) => match token {
                TokenType::Identifier(identifier) => {
                    // Decide whether it's a function declarator or a direct declarator
                    match self.tokenizer.peek_token()? {
                        Some((TokenType::OpenParenthesis, _, _)) => {
                            self.tokenizer.next_token()?; // Consume the OpenParenthesis
                            let parameters = self.parse_parameters()?; // TODO: Does this copy the entire vector? If yes find a way to avoid that
                            self.accept_token(TokenType::CloseParenthesis)?; // Consume the CloseParenthesis

                            // Create the function declarator
                            let fdeclarator = FunctionDeclarator {
                                identifier,
                                parameters,
                            };

                            // Calculate the end of span of the Function Declarator
                            let fdeclarator_end = if let Some(param) = fdeclarator.parameters.last() { param.span.end } else { end };

                            // Return the final function declarator node
                            Ok(Node::new(
                                Declarator::FunctionDeclarator(fdeclarator),
                                Span::new(start, fdeclarator_end),
                            ))
                        },
                        Some((TokenType::Semicolon | TokenType::Comma, _, _)) => Ok(Node::new(
                            Declarator::DirectDeclarator(identifier),
                            Span::new(start, end),
                        )),
                        Some((_, start, _)) => Err(CompilerError{
                            kind: CompilerErrorKind::SyntaxError,
                            message: "Unexpected token, expected a `(` (Function Declarator), or `;` (Direct Declarator)".to_string(), 
                            location: Some(start)
                        }),
                        None => Err(CompilerError{
                            kind: CompilerErrorKind::SyntaxError,
                            message: "Unexpected token, expected a `(` (Function Declarator), or `;` (Direct Declarator), instead encountered an End of File".to_string(), 
                            location: None
                        })
                    }
                }
                _ => panic!("Internal Error: Expected Identifier, but found no token!"),
            },
            None => panic!("Internal Error: Expected Identifier, but found no token!"),
        }
    }

    fn parse_parameters(&mut self) -> Result<Vec<Node<FunctionParameter>>, CompilerError> {
        match self.tokenizer.peek_token()? {
            Some((token, _, _)) => match token {
                // If we reach this line of code, then the function declarator contains no parameters
                // void function();
                //              ^^ Empty parameter list
                TokenType::CloseParenthesis => Ok(Vec::new()),

                // Parse the parameter list
                _ => {
                    let mut parameters = Vec::new();

                    let mut expect_parameter = false;

                    while !matches!(
                        self.tokenizer.peek_token()?,
                        Some((TokenType::CloseParenthesis, _, _))
                    ) {
                        let parameterdecl = self.parse_parameter_decl()?;
                        parameters.push(parameterdecl);

                        // Handle accepting a comma here
                        // No need to handle the case where next token is not , or )
                        // As the parse_parameter_decl() exits only when it encounters one of the above two tokens
                        // In case of next token being None, This function will return the parameters but the calling function will expect a )
                        // And then propagate an error, as it's not this function's responsibility to parse ) which is a part of the Declarator and not the Parameter List
                        match self.tokenizer.peek_token()? {
                            Some((TokenType::Comma, _, _)) => {
                                self.tokenizer.next_token()?;
                                expect_parameter = true;
                            }
                            _ => expect_parameter = false,
                        }
                    }

                    // This is to handle a case where the below C code should not be considered valid
                    // void function(const float param1, )
                    //                                  ^^ Missing parameter
                    // I.e. when a comma is consumed, but the next token is ) then the while loop will exit and return parameters successfully
                    // But that is not valid C syntax, a comma cannot be present if no parameter is present after it
                    if expect_parameter {
                        return Err(CompilerError{
                            kind: CompilerErrorKind::SyntaxError,
                            message: "Expected type specifier for parameter declaration after `,` instead got `)`".to_string(),
                            location: None
                        });
                    }
                    // Return the parameters
                    Ok(parameters)
                }
            },
            // This line will be reached when the file ends abruptly with a half function declaration
            // void function(
            //              ^^ End of file
            None => Err(CompilerError {
                kind: CompilerErrorKind::SyntaxError,
                message: "Missing `)` in the function declaration/definition".to_string(),
                location: None,
            }),
        }
    }

    fn parse_parameter_decl(&mut self) -> Result<Node<FunctionParameter>, CompilerError> {
        let mut specifiers: Vec<Node<DeclarationSpecifier>> = Vec::new();
        while let Some((token, start, end)) = self.tokenizer.peek_token()? {
            match token {
                // keyword.clone() is used to avoid borrowing issues while returning an error containing `keyword` information
                TokenType::Keyword(keyword) => match keyword2declspec(keyword.clone()) {
                    Some(declspec) => {
                        // Push back the declaration specifiers
                        specifiers.push(Node::new(declspec, Span::new(start, end)));

                        // Consume the peeked token as it is a Declaration Specifier
                        self.tokenizer.next_token()?;
                    }
                    None => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SyntaxError,
                            message: format!("Unexpected keyword: {:?}", keyword),
                            location: Some(start),
                        })
                    }
                },
                TokenType::Identifier(identifier) => {
                    // Once we hit an identifier
                    // Parse the expected direct declarator (Function pointers will be handled in the future)
                    if specifiers.len() != 0 {
                        // Currently we only support DirectDeclarators in parameter declaration
                        let declarator = Node::new(
                            Declarator::DirectDeclarator(identifier),
                            Span::new(start, end),
                        );
                        // Create the FunctionParameter with the Identifier as we hit an Identifier
                        let parameter = FunctionParameter {
                            specifiers,
                            declarator: Some(declarator),
                        };

                        // Consume the identifier token as it is a part of the parameter declaration
                        self.tokenizer.next_token()?;

                        // Calculate the span start for the parameter
                        let param_start = if let Some(specifier) = parameter.specifiers.first() {
                            specifier.span.start
                        } else {
                            start
                        };

                        // Return the function parameter node
                        return Ok(Node::new(parameter, Span::new(param_start, end)));
                    } else {
                        // This should happen when the program contains something like
                        // function(param1, const float param2)
                        //          ^^ Missing Declaration Specifiers (like int, void, etc.)
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SyntaxError,
                            message: format!(
                                "Unexpected Identifer: `{}`, Expected a Declaration Specifier",
                                identifier
                            ),
                            location: Some(start),
                        });
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

                        // Calculate the span for the parameter
                        let param_span = Span::new(parameter.specifiers[0].span.start, end);
                        // Return the Parameter Node
                        return Ok(Node::new(parameter, param_span));
                    } else {
                        // This line is reached when the C code should look something like:
                        // void function(, const float param2)
                        //               ^^ Missing parameter declaration
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SyntaxError,
                            message: "Expected a type specifier for parameter declaration, instead found: `,` or `)`"
                                .to_string(),
                                location: Some(start)
                        });
                    }
                }
                _ => break,
            }
        }
        // This line will be reached when neither a keyword, identifier, nor a , or ) are encountered
        // Or there are suddenly no tokens to parse
        Err(CompilerError {
            kind: CompilerErrorKind::SyntaxError,
            message: "Expected a type specifier, or `,` or `)`".to_string(),
            location: None,
        })
    }

    fn parse_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        match self.tokenizer.next_token()? {
            Some((token, start, end)) => match token {
                TokenType::Integer(integer) => Ok(Node::new(
                    Expression::Constant(Node::new(Constant::Integer(integer), Span::none())),
                    Span::none(),
                )),
                _ => Err(CompilerError {
                    kind: CompilerErrorKind::SyntaxError,
                    message: "Currently only integer constants are supported as expressions"
                        .to_string(),
                    location: Some(start),
                }),
            },
            _ => Err(CompilerError {
                kind: CompilerErrorKind::SyntaxError,
                message: "Expected expression, instead found end of file".to_string(),
                location: None,
            }),
        }
    }

    fn accept_token_if<F>(&mut self, mut predicate: F) -> Result<(), CompilerError>
    where
        F: FnMut(TokenType) -> bool,
    {
        match self.tokenizer.next_token()? {
            Some((token, start, _)) => {
                // Token is cloned to avoid borrowing
                if predicate(token.clone()) {
                    Ok(())
                } else {
                    Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: format!("Failed to accept token: {:?}, predicate failed", token),
                        location: Some(start),
                    })
                }
            }
            None => Err(CompilerError {
                kind: CompilerErrorKind::SyntaxError,
                message: "Expected a token".to_string(),
                location: None,
            }),
        }
    }

    fn accept_token(&mut self, tokentype: TokenType) -> Result<(), CompilerError> {
        match self.tokenizer.next_token()? {
            Some((token, start, _)) => {
                if token == tokentype {
                    Ok(())
                } else {
                    Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: format!(
                            "Expected token: {:?}, instead found: {:?}",
                            tokentype, token
                        ),
                        location: Some(start),
                    })
                }
            }
            None => Err(CompilerError {
                kind: CompilerErrorKind::SyntaxError,
                message: format!("Expected token: {:?}", tokentype),
                location: None,
            }),
        }
    }
}
