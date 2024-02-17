//! Module for parsing the tokenized code into an AST tree according to the C17 standard.

use crate::errors::{CompilerError, CompilerErrorKind};
use crate::node::{FileLocation, Node, Span};
use crate::tokenizer::{Keyword, TokenType, Tokenizer};

use crate::ast::*;

/// Maps TokenType::Keyword -> SpecifierQualifier
fn keyword2specififerqualifier(keyword: &Keyword) -> Option<SpecifierQualifier> {
    let specqual = match keyword {
        // Type Qualifiers
        Keyword::Const => SpecifierQualifier::TypeQualifier(TypeQualifier::Const),
        Keyword::Restrict => SpecifierQualifier::TypeQualifier(TypeQualifier::Restrict),
        Keyword::Volatile => SpecifierQualifier::TypeQualifier(TypeQualifier::Volatile),
        Keyword::_Atomic => SpecifierQualifier::TypeQualifier(TypeQualifier::Atomic),

        // Type Specifiers
        Keyword::Void => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Void),
        Keyword::Char => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Char),
        Keyword::Short => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Short),
        Keyword::Int => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Int),
        Keyword::Long => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Long),
        Keyword::Float => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Float),
        Keyword::Double => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Double),
        Keyword::Signed => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Signed),
        Keyword::Unsigned => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Unsigned),
        Keyword::_Bool => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Bool),
        Keyword::_Complex => SpecifierQualifier::TypeSpecifier(TypeSpecifier::Complex),

        _ => return None,
    };
    Some(specqual)
}

/// Maps TokenType::Keyword -> DeclarationSpecifier
fn keyword2declspec(keyword: &Keyword) -> Option<DeclarationSpecifier> {
    let declspec = match keyword {
        // Storage Class Specifiers
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
        Keyword::_Thread_local => {
            DeclarationSpecifier::StorageClassSpecifier(StorageClassSpecifier::ThreadLocal)
        }
        Keyword::Typedef => {
            DeclarationSpecifier::StorageClassSpecifier(StorageClassSpecifier::Typedef)
        }

        // Type Qualifiers
        Keyword::Const => DeclarationSpecifier::TypeQualifier(TypeQualifier::Const),
        Keyword::Restrict => DeclarationSpecifier::TypeQualifier(TypeQualifier::Restrict),
        Keyword::Volatile => DeclarationSpecifier::TypeQualifier(TypeQualifier::Volatile),
        Keyword::_Atomic => DeclarationSpecifier::TypeQualifier(TypeQualifier::Atomic),

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
        Keyword::_Bool => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Bool),
        Keyword::_Complex => DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Complex),

        // Function Specifiers
        Keyword::Inline => DeclarationSpecifier::FunctionSpecifier(FunctionSpecifier::Inline),
        Keyword::_Noreturn => DeclarationSpecifier::FunctionSpecifier(FunctionSpecifier::NoReturn),

        // Unknown keyword present in declaration specifier
        _ => return None,
    };
    // Return the declaration specifier type
    Some(declspec)
}

fn token2unaryop(token: &TokenType) -> Option<UnaryOperator> {
    let unaryop = match token {
        TokenType::BitwiseAndOperator => UnaryOperator::Address,
        TokenType::Asterisk => UnaryOperator::Indirection,
        TokenType::Plus => UnaryOperator::Plus,
        TokenType::Minus => UnaryOperator::Minus,
        TokenType::BitwiseComplimentOperator => UnaryOperator::Complement,
        TokenType::LogicalNotOperator => UnaryOperator::Negate,
        _ => return None,
    };
    Some(unaryop)
}

/// Token -> Assignment Binary Operator
fn token2asgnbinaryop(token: &TokenType) -> Option<BinaryOperator> {
    let binaryop = match token {
        TokenType::Equals => BinaryOperator::Assign,
        TokenType::PlusEquals => BinaryOperator::AssignPlus,
        TokenType::MinusEquals => BinaryOperator::AssignMinus,
        TokenType::AsteriskEquals => BinaryOperator::AssignMultiply,
        TokenType::SlashEquals => BinaryOperator::AssignDivide,
        TokenType::PercentEquals => BinaryOperator::AssignModulo,
        TokenType::LeftShiftEqualsOperator => BinaryOperator::AssignShiftLeft,
        TokenType::RightShiftEqualsOperator => BinaryOperator::AssignShiftRight,
        TokenType::BitwiseOrEqualsOperator => BinaryOperator::AssignBitwiseOr,
        TokenType::BitwiseAndEqualsOperator => BinaryOperator::AssignBitwiseAnd,
        TokenType::ExclusiveOrEqualsOperator => BinaryOperator::AssignBitwiseXor,
        _ => return None,
    };
    Some(binaryop)
}

fn is_expr_unary(expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(_)
        | Expression::Constant(_)
        | Expression::StringLiteral(_)
        | Expression::UnaryOperator(_)
        | Expression::SizeofType(_)
        | Expression::SizeofVal(_)
        | Expression::Alignof(_)
        | Expression::Member(_)
        | Expression::Call(_) => true,
        Expression::Cast(_)
        | Expression::BinaryOperator(_)
        | Expression::TernaryOperator(_)
        | Expression::Comma(_) => false,
    }
}

#[derive(Default)]
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(src),
        }
    }

    /// The main parse function that uses the tokenizer to generate an Abstract Syntax Tree
    pub fn parse(&mut self) -> Result<TranslationUnit, CompilerError> {
        let mut tranlation_unit = TranslationUnit {
            external_declarations: Vec::new(),
        };

        loop {
            match self.tokenizer.peek_token()? {
                Some(_) => {
                    // Parse a Declaration
                    // Irrespective of the next part of the program being a declaration or a function definition...
                    // We need to parse some declaration-like code
                    let declaration = self.parse_declaration()?;

                    match self.tokenizer.next_token()? {
                        Some((token, start, end)) => match token {
                            TokenType::OpenBrace => {
                                // In case of a function definition...
                                // It is compulsory to have exactly 1 declarator which is the function declarator
                                if declaration.init_declarators.len() != 1 {
                                    return Err(CompilerError {
                                        kind: CompilerErrorKind::SyntaxError,
                                        message: "Unexpected `{`, expected a semicolon".to_string(),
                                        location: Some(start),
                                    });
                                }

                                // As it is confirmed by the previous if statement that there is only 1 declarator
                                // We can grab it from the init declarator list
                                let fdeclarator = &declaration
                                    .init_declarators
                                    .first()
                                    .unwrap()
                                    .node
                                    .declarator;

                                // Parse a function definition
                                if let Declarator::FunctionDeclarator(fdecl) = &fdeclarator.node {
                                    // A function body must be a compound statement
                                    let funcbody = self.parse_compound_stmt()?;
                                    // Consume the CloseBrace
                                    let (_, brace_end) =
                                        self.accept_token(TokenType::CloseBrace)?;

                                    // Calculate the span of the function definition
                                    // The span of the function definition is from the start of the declaration
                                    // to the end of the definition, i.e., the CloseBrace end index
                                    let declspan_start = declaration.specifiers[0].span.start;
                                    let funcdef_span = Span::new(declspan_start, brace_end);

                                    // Create and push the function definition that we just parsed
                                    tranlation_unit.external_declarations.push(Node::new(
                                        ExternalDeclaration::FunctionDefinition(
                                            FunctionDefinition {
                                                specifiers: declaration.specifiers,
                                                declarator: Node::new(
                                                    // Cloning here to avoid ownership issues
                                                    // Will this be a performance overhead?
                                                    fdecl.clone(),
                                                    fdeclarator.span,
                                                ),
                                                body: funcbody,
                                            },
                                        ),
                                        funcdef_span,
                                    ));
                                } else {
                                    // If `{` is encountered, then it must be a function definition
                                    // Reaching this line of code indicates the C code looks like this:
                                    // const int variable_declaration{};
                                    //                               ^^ Unexpected `{`
                                    return Err(CompilerError {
                                        kind: CompilerErrorKind::SyntaxError,
                                        message: "Unexpected token: `{`, statement is not a valid function declaration".to_string(),
                                        location: Some(start),
                                    });
                                }
                            }
                            TokenType::Semicolon => {
                                // Calculate the span of the declaration
                                // span = start of the first DeclarationSpecifier in the declaration -> end of the semicolon
                                let declspan_start =
                                    declaration.specifiers.first().unwrap().span.start; // Here we assume that a declaration will always have atleast one specifier
                                let declspan = Span::new(declspan_start, end);

                                // Create an ExternalDeclaration and push it
                                tranlation_unit.external_declarations.push(Node::new(
                                    ExternalDeclaration::Declaration(declaration),
                                    declspan,
                                ));
                            }
                            _ => {
                                return Err(CompilerError {
                                    kind: CompilerErrorKind::SyntaxError,
                                    message: format!(
                                        "Expected a `;` or `{{` instead got: {:?}",
                                        token
                                    ),
                                    location: Some(start),
                                })
                            }
                        },
                        _ => {
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SyntaxError,
                                message: "Expected a `;` or `{` instead got end of file"
                                    .to_string(),
                                location: None,
                            })
                        }
                    }
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
                TokenType::Keyword(keyword) => match keyword2declspec(&keyword) {
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
                        let init_declarator_list = self.parse_init_declarator_list()?;
                        return Ok(Declaration {
                            specifiers,
                            init_declarators: init_declarator_list,
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

    fn parse_init_declarator_list(&mut self) -> Result<Vec<Node<InitDeclarator>>, CompilerError> {
        // init-declarator-list:
        //      init-declarator
        //      init-declarator-list , init-declarator

        let mut init_declarators: Vec<Node<InitDeclarator>> = Vec::new();
        loop {
            match self.tokenizer.peek_token()? {
                Some((_, _, _)) => {
                    // Parse the declarator
                    let declarator = self.parse_declarator()?;

                    let initializer;
                    let span;

                    // Check for an intializer and calculate the total span of the InitDeclarator
                    if let Some((TokenType::Equals, _, _)) = self.tokenizer.peek_token()? {
                        // Consume the Assignment Operator
                        self.tokenizer.next_token()?;

                        // Parse the initializer, as we have confirmed the presence of an =, i.e., Assignment Operator
                        let parsed_initializer = self.parse_initializer()?;

                        // Calculate the total span of the initializer
                        // span = start of declarator -> end of initializer
                        span = Span::new(declarator.span.start, parsed_initializer.span.end);

                        // Finally store the initializer
                        initializer = Some(parsed_initializer);
                    } else {
                        // As there is no initializer, the span is the same as the declarator
                        span = Span::new(declarator.span.start, declarator.span.end);

                        // No Assignment Operator means no initailizer is present
                        initializer = None;
                    }

                    // Create and push the InitDeclarator
                    init_declarators.push(Node::new(
                        InitDeclarator {
                            declarator,
                            initializer,
                        },
                        span,
                    ));

                    // Check for a Comma, so that we can break the loop if we expect more InitDeclarators to be parsed
                    if let Some((TokenType::Comma, _, _)) = self.tokenizer.peek_token()? {
                        // Consume the Comma
                        self.tokenizer.next_token()?;
                    } else {
                        break;
                    };
                }
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message:
                            "Expected a semicolon, or an init-declarator, instead found end of file"
                                .to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(init_declarators)
    }

    fn parse_initializer(&mut self) -> Result<Node<Initializer>, CompilerError> {
        // initializer:
        //      assignment-expression
        //      { initializer-list }
        //      { initializer-list , }
        match self.tokenizer.peek_token()? {
            Some((TokenType::OpenBrace, _, _)) => {
                // Parse Initializer-List
                todo!()
            }
            Some(_) => {
                // Parse an expression
                let expression = self.parse_expr()?;

                // Create and return an initializer using the parsed expression
                Ok(Node::new(
                    Initializer::AssignmentExpression(expression.node),
                    expression.span,
                ))
            }
            None => Err(CompilerError {
                kind: CompilerErrorKind::SyntaxError,
                message:
                    "Expected assignment expression or an initializer list, instead got end of file"
                        .to_string(),
                location: None,
            }),
        }
    }

    fn parse_declarator(&mut self) -> Result<Node<Declarator>, CompilerError> {
        // declarator:
        //      pointeropt direct-declarator
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
                        Some((TokenType::Semicolon | TokenType::Comma | TokenType::Equals, _, _)) => Ok(Node::new(
                            Declarator::DirectDeclarator(identifier),
                            Span::new(start, end),
                        )),
                        Some((next, start, _)) => Err(CompilerError{
                            kind: CompilerErrorKind::SyntaxError,
                            message: format!("Unexpected token: {:?}, expected a `(` (Function Declarator), or `;` (Direct Declarator)", next),
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

    fn parse_direct_declarator(&mut self) {
        // direct-declarator:
        //      identifier
        //      ( declarator )
        //      direct-declarator [ type-qualifier-listopt assignment-expressionopt ]
        //      direct-declarator [ static type-qualifier-listopt assignment-expression ]
        //      direct-declarator [ type-qualifier-list static assignment-expression ]
        //      direct-declarator [ type-qualifier-listopt * ]
        //      direct-declarator ( parameter-type-list )
        //      direct-declarator ( identifier-listopt )
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
                TokenType::Keyword(keyword) => match keyword2declspec(&keyword) {
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

    fn parse_statement(&mut self) -> Result<Node<Statement>, CompilerError> {
        match self.tokenizer.peek_token()? {
            Some((token, start, end)) => {
                match token {
                    TokenType::Keyword(keyword) => {
                        // Consume the next token in case of all keywords except Sizeof and _Alignof
                        // Because they will be part of expressions that should be consumed by the parse_expr() function only
                        if keyword != Keyword::Sizeof && keyword != Keyword::_Alignof {
                            self.tokenizer.next_token()?;
                        }

                        // Parse the statements starting with keywords
                        match keyword {
                            Keyword::Case => {
                                // labeled-statement:
                                //      case constant-expression : statement

                                let constexpr = self.parse_constant_expr()?;
                                // Consume a colon
                                self.accept_token(TokenType::Colon)?;
                                // Parse a statement
                                let casestmt = self.parse_statement()?;

                                // Calculate the span of the case statement
                                // Span = Start of the `case` keyword -> End of the case statement
                                let span = Span::new(start, casestmt.span.end);

                                // Create and return the Case Statement
                                Ok(Node::new(
                                    Statement::CaseStatement(Box::new(CaseStatement {
                                        constexpr,
                                        statement: casestmt,
                                    })),
                                    span,
                                ))
                            }
                            Keyword::Default => {
                                // labeled-statement:
                                //      default : statement

                                // Consume a colon
                                self.accept_token(TokenType::Colon)?;

                                // Parse a statement
                                let defaultstmt = self.parse_statement()?;

                                // Calculate the span of the default statement
                                // Span = Start of the `default` keyword -> End of the default statement
                                let span = Span::new(start, defaultstmt.span.end);

                                // Create and return the Default Statement
                                Ok(Node::new(
                                    Statement::DefaultStatement(Box::new(defaultstmt)),
                                    span,
                                ))
                            }
                            Keyword::If => {
                                // if (<expression>) <statement>
                                //    ^ Accept this OpenParenthesis
                                self.accept_token(TokenType::OpenParenthesis)?;

                                // if (<expression>) <statement>
                                //      ^^^ Parse this expression
                                let if_expr = self.parse_expr()?;

                                // if (<expression>) <statement>
                                //                 ^ Accept this CloseParenthesis
                                self.accept_token(TokenType::CloseParenthesis)?;

                                // if (<expression>) <statement>
                                //                    ^^^ Parse this statement
                                let if_block = self.parse_statement()?;

                                let else_block;
                                let stmt_span;

                                // Check for an else statement and parse it
                                // Also calculate the span for the entire if statement
                                if let Some((TokenType::Keyword(Keyword::Else), _, _)) =
                                    self.tokenizer.peek_token()?
                                {
                                    // Consume the Else Token once it is confirmed that it is really an Else Token
                                    self.tokenizer.next_token()?;
                                    // Parse the else statement
                                    let else_stmt = self.parse_statement()?;
                                    // The span of the entire if statement =
                                    // Start of if keyword -> End of else statement
                                    stmt_span = Span::new(start, else_stmt.span.end);
                                    else_block = Some(else_stmt);
                                } else {
                                    // There is no else statement
                                    else_block = None;
                                    // The span of the entire if statement =
                                    // Start of if keyword -> End of if statement
                                    stmt_span = Span::new(start, if_block.span.end);
                                }

                                // Create and return the If statement
                                Ok(Node::new(
                                    Statement::IfStatement(Box::new(IfStatement {
                                        expression: if_expr,
                                        if_block,
                                        else_block,
                                    })),
                                    stmt_span,
                                ))
                            }
                            Keyword::Switch => {
                                // selection-statement:
                                //      switch ( expression ) statement

                                // Consume the OpenParenthesis
                                self.accept_token(TokenType::OpenParenthesis)?;
                                // Parse the Switch Expression
                                let switchexpr = self.parse_expr()?;
                                // Consume the CloseParenthesis
                                self.accept_token(TokenType::CloseParenthesis)?;
                                // Parse the switch block/statement
                                let switchstmt = self.parse_statement()?;

                                // Calculate span of the entire switch statement
                                // Span of switch statement = (start of the switch keyword, end of the switch statement)
                                let span = Span::new(start, switchstmt.span.end);
                                // Create and return the switch statement
                                Ok(Node::new(
                                    Statement::SwitchStatement(Box::new(SwitchStatement {
                                        expression: switchexpr,
                                        statement: switchstmt,
                                    })),
                                    span,
                                ))
                            }
                            Keyword::While => {
                                // Accept a OpenParenthesis
                                self.accept_token(TokenType::OpenParenthesis)?;
                                // Parse the condition inside the while (<expression>) statement
                                //                                       ^^^
                                let expression = self.parse_expr()?;
                                // Accept a CloseParenthesis
                                self.accept_token(TokenType::CloseParenthesis)?;
                                // Parse the while statement
                                let block = self.parse_statement()?;

                                // Calculate the span of the while statement
                                // Span = Start of the `while` keyword -> End of the statement
                                let span = Span::new(start, block.span.end);

                                Ok(Node::new(
                                    Statement::WhileStatement(Box::new(WhileStatement {
                                        expression,
                                        statement: block,
                                    })),
                                    span,
                                ))
                            }
                            Keyword::Do => {
                                // iteration-statement:
                                //      do statement while ( expression ) ;

                                // Parse the do statement
                                let dostmt = self.parse_statement()?;

                                // Accept `while (`
                                self.accept_token(TokenType::Keyword(Keyword::While))?;
                                self.accept_token(TokenType::OpenParenthesis)?;
                                // Parse the while expression
                                let doexpr = self.parse_expr()?;
                                // Accept `)`
                                self.accept_token(TokenType::CloseParenthesis)?;
                                // Accept `;`
                                let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;

                                // Calculate span of the entire while statement
                                // Span of the while statement = Start of do keyword -> End of semicolon after `while` keyword
                                let span = Span::new(start, semicolon_end);

                                // Create and return the DoWhileStatement
                                Ok(Node::new(
                                    Statement::DoWhileStatement(Box::new(DoWhileStatement {
                                        statement: dostmt,
                                        expression: doexpr,
                                    })),
                                    span,
                                ))
                            }
                            Keyword::For => {
                                // Accept `(`
                                self.accept_token(TokenType::OpenParenthesis)?;

                                let forinitializer: Node<ForInitializer>;
                                match self.tokenizer.peek_token()? {
                                    Some((token, peek_start, _)) => {
                                        // Check if the next token is a DeclarationSpecifier
                                        if let TokenType::Keyword(keyword) = token {
                                            if let Some(_) = keyword2declspec(&keyword) {
                                                // If yes, Expect a ForInitializer::Declaration
                                                let declaration = self.parse_declaration()?;

                                                // Accept a `;`
                                                let (_, semicolon_end) =
                                                    self.accept_token(TokenType::Semicolon)?;

                                                let forinit =
                                                    ForInitializer::Declaration(declaration);
                                                let forinit_span =
                                                    Span::new(peek_start, semicolon_end);

                                                forinitializer = Node::new(forinit, forinit_span);
                                            } else {
                                                // This must happen when code may have something like:
                                                // for (return; i = 0; i++)
                                                //      ^^ Unexpected keyword which is not a DeclarationSpecifier
                                                return Err(CompilerError {
                                                    kind: CompilerErrorKind::SyntaxError,
                                                    message: format!("Expected a declaration specifier or an expression, instead got unexpected Keyword: {:?}", keyword),
                                                    location: Some(peek_start)
                                                });
                                            }
                                        } else if token == TokenType::Semicolon {
                                            // Consume the first semicolon which follows the ForInitializer
                                            self.tokenizer.next_token()?;

                                            // Else if the next token is a Semicolon, then return an Empty Initializer
                                            forinitializer = Node::new(
                                                ForInitializer::Empty,
                                                Span::new(peek_start, peek_start),
                                            );
                                        } else {
                                            // Else expect an expression (like an assignment expression)
                                            let expression = self.parse_expr()?;

                                            // Accept a `;`
                                            let (_, semicolon_end) =
                                                self.accept_token(TokenType::Semicolon)?;

                                            // Calculate the span of the ForInitializer
                                            // Span = Start of expression -> End of the semicolon
                                            let span =
                                                Span::new(expression.span.start, semicolon_end);

                                            // Create and store the ForInitializer
                                            forinitializer = Node::new(
                                                ForInitializer::Expression(expression.node),
                                                span,
                                            );
                                        }
                                    }
                                    None => {
                                        // This error should occur when we encounter an end of file instead of an initializer
                                        return Err(CompilerError {
                                            kind: CompilerErrorKind::SyntaxError,
                                            message:
                                                "Expected a for initializer, instead got end of file"
                                                    .to_string(),
                                            location: None,
                                        });
                                    }
                                }

                                // Parse the For Condition
                                let condition = if let Some((TokenType::Semicolon, _, _)) =
                                    self.tokenizer.peek_token()?
                                {
                                    // If the next token is semicolon then the condition is None
                                    None
                                } else {
                                    // Else parse the condition expression
                                    Some(self.parse_expr()?)
                                };

                                // Accept a semicolon irrespective of the presence of a for condition
                                self.accept_token(TokenType::Semicolon)?;

                                // Parse the For Step Expression
                                let step = if let Some((TokenType::CloseParenthesis, _, _)) =
                                    self.tokenizer.peek_token()?
                                {
                                    // If the next token is `)` then the step expression is None
                                    None
                                } else {
                                    // Else parse the step expression
                                    Some(self.parse_expr()?)
                                };

                                // Accept a `)`
                                self.accept_token(TokenType::CloseParenthesis)?;

                                // Parse the actual for statement
                                let statement = self.parse_statement()?;

                                // Calculate the span of the entire ForStatement
                                // Span = Start of the `for` keyword -> End of the statement
                                let span = Span::new(start, statement.span.end);

                                // Create and return the actual For Statement
                                Ok(Node::new(
                                    Statement::ForStatement(Box::new(ForStatement {
                                        initializer: forinitializer,
                                        condition,
                                        step,
                                        statement,
                                    })),
                                    span,
                                ))
                            }
                            Keyword::Return => {
                                // Return -> Jump Statement
                                let expression = self.parse_expr()?;
                                let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;

                                // Calculate span of the entire return statement
                                // Span of return statement = (start of the return keyword, end of the semicolon token)
                                let span = Span::new(start, semicolon_end);
                                // Create and return the actual return statement
                                Ok(Node::new(Statement::ReturnStatement(expression), span))
                            }
                            Keyword::Break => {
                                // Accept a semicolon
                                let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;
                                // Create and return a Break Statement
                                Ok(Node::new(
                                    Statement::BreakStatement,
                                    Span::new(start, semicolon_end),
                                ))
                            }
                            Keyword::Continue => {
                                // Accept a semicolon
                                let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;
                                // Create and return a Continue Statement
                                Ok(Node::new(
                                    Statement::ContinueStatement,
                                    Span::new(start, semicolon_end),
                                ))
                            }
                            Keyword::Goto => {
                                // jump-statement:
                                //      goto identifier ;

                                // Force the next token to be an identifier
                                match self.tokenizer.next_token()? {
                                    Some((TokenType::Identifier(identifier), id_start, id_end)) => {
                                        // Accept a `;`
                                        let (_, semicolon_end) =
                                            self.accept_token(TokenType::Semicolon)?;

                                        // Create and return a goto statement
                                        Ok(Node::new(
                                            Statement::GotoStatement(Node::new(
                                                identifier,
                                                Span::new(id_start, id_end),
                                            )),
                                            // Span of the entire goto statement = Start of goto keyword -> End of semicolon
                                            Span::new(start, semicolon_end),
                                        ))
                                    }
                                    // This case will occur when code is something like:
                                    // goto  ;
                                    //      ^^ Missing Identifier
                                    Some((_, start, _)) => Err(CompilerError {
                                        kind: CompilerErrorKind::SyntaxError,
                                        message: "Expected an identifier".to_string(),
                                        location: Some(start),
                                    }),
                                    None => Err(CompilerError {
                                        kind: CompilerErrorKind::SyntaxError,
                                        message: "Expected an identifier, instead got end of file"
                                            .to_string(),
                                        location: None,
                                    }),
                                }
                            }
                            Keyword::Sizeof | Keyword::_Alignof => {
                                let expression = self.parse_expr()?;

                                // Accept `;`
                                let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;

                                // Calculate the span of the entire expression statement
                                // Span = Start of the expression -> End of the semicolon
                                let span = Span::new(start, semicolon_end);
                                return Ok(Node::new(
                                    Statement::ExpressionStatement(Some(expression)),
                                    span,
                                ));
                            }
                            _ => Err(CompilerError {
                                kind: CompilerErrorKind::SyntaxError,
                                message: format!(
                                    "Unexpected start of a statement with keyword: {:?}",
                                    keyword
                                ),
                                location: Some(start),
                            }),
                        }
                    }
                    TokenType::Identifier(identifier) => {
                        // Either it can be a labeled statement
                        // Or an expression statement
                        // In both cases we can parse it as an expression
                        let expression = self.parse_expr()?;

                        // Then decide the type of the statement
                        // Based on whether the expression is just an identifier or not
                        if let Expression::Identifier(_) = &expression.node {
                            if let Some((TokenType::Colon, _, _)) = self.tokenizer.peek_token()? {
                                // labeled-statement:
                                //      identifier : statement
                                self.tokenizer.next_token()?;

                                // Parse the labeled statement
                                let statement = self.parse_statement()?;

                                // Calculate the span of the entire labeled statement
                                // Span = Start of the identifier -> End of the statement
                                let span = Span::new(start, statement.span.end);

                                return Ok(Node::new(
                                    Statement::LabeledStatement(Box::new(LabeledStatement {
                                        identifier: Node::new(identifier, Span::new(start, end)),
                                        statement,
                                    })),
                                    span,
                                ));
                            }
                        }
                        // Else the statement is an expression-statement with the following grammar:
                        //      expression-statement:
                        //           expressionopt ;

                        // Accept `;`
                        let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;

                        // Calculate the span of the entire expression statement
                        // Span = Start of the expression -> End of the semicolon
                        let span = Span::new(start, semicolon_end);
                        return Ok(Node::new(
                            Statement::ExpressionStatement(Some(expression)),
                            span,
                        ));
                    }
                    TokenType::OpenBrace => {
                        // Consume the OpenBrace
                        self.tokenizer.next_token()?;
                        // Parse a compound statement
                        let compound_stmt = self.parse_compound_stmt()?;
                        // Accept a closing brace
                        self.accept_token(TokenType::CloseBrace)?;
                        // Return the parsed compound statement
                        Ok(Node::new(compound_stmt.node, compound_stmt.span))
                    }
                    TokenType::Semicolon => {
                        // Consume the semicolon
                        self.tokenizer.next_token()?;

                        // If the statement starts with a semicolon then we store it as an empty expression statement
                        // The span of this statement will be the (start, end) of the semicolon token
                        Ok(Node::new(
                            Statement::ExpressionStatement(None),
                            Span::new(start, end),
                        ))
                    }
                    _ => {
                        // expression-statement:
                        //      expressionopt ;

                        // If no specific token is encountered then we expect an expression statement
                        let expression = self.parse_expr()?;

                        // Accept `;`
                        let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;

                        // Calculate the span of the entire expression statement
                        // Span = Start of the expression -> End of the semicolon
                        let span = Span::new(start, semicolon_end);
                        return Ok(Node::new(
                            Statement::ExpressionStatement(Some(expression)),
                            span,
                        ));
                    }
                }
            }
            _ => todo!(),
        }
    }

    /// Note: This function doesn't consume either of OpenBrace and CloseBrace tokens associated with it.
    /// It is the caller's responsibility to check for OpenBrace and consume a CloseBrace after calling this function.
    fn parse_compound_stmt(&mut self) -> Result<Node<Statement>, CompilerError> {
        // compound-statement:
        //      { block-item-listopt }
        // block-item-list:
        //      block-item
        //      block-item-list block-item
        // block-item:
        //      declaration
        //      statement
        let mut blockitems: Vec<Node<BlockItem>> = Vec::new();

        let span_start = self.tokenizer.get_lineinfo();

        while !matches!(
            self.tokenizer.peek_token()?,
            Some((TokenType::CloseBrace, _, _))
        ) {
            match self.tokenizer.peek_token()? {
                Some((token, start, _)) => {
                    let mut is_declaration = false;

                    // The logic here is that a BlockItem can be either a declaration or a statement
                    // If it is a declaration then it should start with a DeclarationSpecifier
                    if let TokenType::Keyword(keyword) = token {
                        if let Some(_) = keyword2declspec(&keyword) {
                            is_declaration = true;
                            // Parse a declaration
                            let declaration = self.parse_declaration()?;
                            // Accept a semicolon after the declaration
                            let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;
                            // Create a block item using the parsed declaration
                            // The span of the block item =
                            // start of the first token -> end of the semicolon after the declaration
                            blockitems.push(Node::new(
                                BlockItem::Declaration(declaration),
                                Span::new(start, semicolon_end),
                            ));
                        }
                    }
                    // If not then it must be a statement, hence we parse it here
                    if !is_declaration {
                        // Parse a statement
                        let statement = self.parse_statement()?;
                        // Create a block item using the node and span of the statement
                        // The span of the block item will be the same as that of the statement
                        blockitems.push(Node::new(
                            BlockItem::Statement(statement.node),
                            statement.span,
                        ));
                    }
                }
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected a declaration or a statement, instead got end of file"
                            .to_string(),
                        location: None,
                    })
                }
            };
        }

        // Calculate the span of the compound statement
        let span = if !blockitems.is_empty() {
            // If the compound statement is not empty then the span is the start index of the first statement
            // and the end of the last statement
            Span::new(span_start, blockitems.last().unwrap().span.end)
        } else {
            // peek_token() has to return some token as the while loop before this will only exit
            // when the next token is CloseBrace
            // If it were to return None token then it would've been handled in the while loop itself
            let start = self.tokenizer.peek_token()?.unwrap().1;
            // If the compound statement is empty then the span is the start index of the CloseBrace
            Span::new(span_start, start)
        };

        // Create and return the compound statement
        Ok(Node::new(Statement::CompoundStatement(blockitems), span))
    }

    /// Note: This function doesn't consume a semicolon at the end.
    /// That must be handled by the calling function
    fn parse_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // expression:
        //      assignment-expression
        //      expression , assignment-expression
        let mut expressions: Vec<Node<Expression>> = Vec::new();
        while let Some(_) = self.tokenizer.peek_token()? {
            let expression = self.parse_assignment_expr()?;
            expressions.push(expression);

            if let Some((TokenType::Comma, _, _)) = self.tokenizer.peek_token()? {
                // Consume the `,`
                self.tokenizer.next_token()?;
            } else {
                break;
            }
        }

        if expressions.len() == 1 {
            Ok(expressions.pop().unwrap())
        } else {
            let span = Span::new(
                expressions.first().unwrap().span.start,
                expressions.last().unwrap().span.end,
            );
            Ok(Node::new(Expression::Comma(expressions), span))
        }
    }

    /// Recursively parses an Assignment Expression
    fn parse_assignment_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // assignment-expression:
        //      conditional-expression
        //      unary-expression assignment-operator assignment-expression
        let expression = self.parse_conditional_expr()?;

        // Check if the expression is a unary expression
        // TODO: This if statement will never be hit, as parsing of unary expressions is not implemented yet
        if is_expr_unary(&expression.node) {
            // Parse the second production of the grammar
            if let Some((token, start, end)) = self.tokenizer.peek_token()? {
                // Check if the token is some assignment operator
                if let Some(operator) = token2asgnbinaryop(&token) {
                    // If yes then consume that token
                    self.tokenizer.next_token()?;
                    // And then parse another assignment expression
                    let rhs = self.parse_assignment_expr()?;
                    // Calculate the span for the expression
                    // Span = Start of lhs -> End of rhs
                    let span = Span::new(expression.span.start, rhs.span.end);
                    // Create a binary operator expression with the assignment operator and lhs and rhs
                    return Ok(Node::new(
                        Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                            operator: Node::new(operator, Span::new(start, end)),
                            lhs: expression,
                            rhs,
                        })),
                        span,
                    ));
                }
            }
        }
        Ok(expression)
    }

    /// This is a wrapper function which calls parse_conditional_expr() under the hood
    /// As according to the C17 grammar a constant-expression is any conditional-expression (syntactically)
    fn parse_constant_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // constant-expression:
        //      conditional-expression
        self.parse_conditional_expr()
    }

    /// Follows recursive pattern to parse the conditional expression grammar
    fn parse_conditional_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // conditional-expression:
        //      logical-OR-expression
        //      logical-OR-expression ? expression : conditional-expression
        let expression = self.parse_logical_OR_expr()?;

        match self.tokenizer.peek_token()? {
            Some((token, _, _)) => match token {
                TokenType::QuestionMark => {
                    // Consume the Question Mark
                    self.tokenizer.next_token()?;
                    // Parse the if expression
                    let if_expr = self.parse_expr()?;
                    // Accept a Colon as it is compulsory to have both the if and else expressions
                    self.accept_token(TokenType::Colon)?;
                    // Parse the else expression
                    let else_expr = self.parse_conditional_expr()?;

                    // Create and return a Ternary Operator Expression
                    // Span of Ternary Operator Expression =
                    // Start of condition -> End of else expression
                    let span = Span::new(expression.span.start, else_expr.span.end);
                    Ok(Node::new(
                        Expression::TernaryOperator(Box::new(TernaryOperatorExpression {
                            condition: expression,
                            if_expr,
                            else_expr,
                        })),
                        span,
                    ))
                }
                _ => Ok(expression),
            },
            None => {
                return Err(CompilerError {
                    kind: CompilerErrorKind::SyntaxError,
                    message: "Expected expression, instead got end of file".to_string(),
                    location: None,
                })
            }
        }
    }

    #[allow(non_snake_case)]
    fn parse_logical_OR_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // logical-OR-expression:
        //      logical-AND-expression
        //      logical-OR-expression || logical-AND-expression
        let mut expression = self.parse_logical_AND_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::LogicalOrOperator => {
                        // Consume the ExclusiveOrOperator token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        let rhs = self.parse_logical_AND_expr()?;

                        let operator = BinaryOperator::LogicalOr;

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    #[allow(non_snake_case)]
    fn parse_logical_AND_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // logical-AND-expression:
        //      inclusive-OR-expression
        //      logical-AND-expression && inclusive-OR-expression
        let mut expression = self.parse_inclusive_or_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::LogicalAndOperator => {
                        // Consume the ExclusiveOrOperator token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        let rhs = self.parse_inclusive_or_expr()?;

                        let operator = BinaryOperator::LogicalAnd;

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    fn parse_inclusive_or_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // inclusive-OR-expression:
        //      exclusive-OR-expression
        //      inclusive-OR-expression | exclusive-OR-expression
        let mut expression = self.parse_exor_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::BitwiseOrOperator => {
                        // Consume the ExclusiveOrOperator token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        let rhs = self.parse_exor_expr()?;

                        let operator = BinaryOperator::BitwiseOr;

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    fn parse_exor_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // exclusive-OR-expression:
        //      AND-expression
        //      exclusive-OR-expression ^ AND-expression
        let mut expression = self.parse_AND_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::ExclusiveOrOperator => {
                        // Consume the ExclusiveOrOperator token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        let rhs = self.parse_AND_expr()?;

                        let operator = BinaryOperator::BitwiseXor;

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    #[allow(non_snake_case)]
    fn parse_AND_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // AND-expression:
        //      equality-expression
        //      AND-expression & equality-expression
        let mut expression = self.parse_equality_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::BitwiseAndOperator => {
                        // Consume the BitwiseAndOperator token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        let rhs = self.parse_equality_expr()?;

                        let operator = BinaryOperator::BitwiseAnd;

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    fn parse_equality_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // equality-expression:
        //      relational-expression
        //      equality-expression == relational-expression
        //      equality-expression != relational-expression
        let mut expression = self.parse_relational_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::EqualityOperator | TokenType::NotEqualsOperator => {
                        // Consume the EqualityOperator/NotEqualsOperator token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        let rhs = self.parse_relational_expr()?;

                        let operator = if token == TokenType::EqualityOperator {
                            BinaryOperator::Equals
                        } else {
                            BinaryOperator::NotEquals
                        };

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    fn parse_relational_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // relational-expression:
        //      shift-expression
        //      relational-expression < shift-expression
        //      relational-expression > shift-expression
        //      relational-expression <= shift-expression
        //      relational-expression >= shift-expression
        let mut expression = self.parse_shift_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::LessThanOperator
                    | TokenType::LessThanEqualsOperator
                    | TokenType::GreaterThanOperator
                    | TokenType::GreaterThanEqualsOperator => {
                        // Consume the LessThanOperator/LessThanEqualsOperator/GreaterThanOperator/GreaterThanEqualsOperator token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        let rhs = self.parse_shift_expr()?;

                        let operator = if token == TokenType::LessThanOperator {
                            BinaryOperator::Less
                        } else if token == TokenType::LessThanEqualsOperator {
                            BinaryOperator::LessOrEqual
                        } else if token == TokenType::GreaterThanOperator {
                            BinaryOperator::Greater
                        } else {
                            BinaryOperator::GreaterOrEqual
                        };

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    fn parse_shift_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // shift-expression:
        //      additive-expression
        //      shift-expression « additive-expression
        //      shift-expression » additive-expression
        let mut expression = self.parse_additive_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::LeftShiftOperator | TokenType::RightShiftOperator => {
                        // Consume the LeftShift/RightShift token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        let rhs = self.parse_additive_expr()?;

                        let operator = if token == TokenType::LeftShiftOperator {
                            BinaryOperator::ShiftLeft
                        } else {
                            BinaryOperator::ShiftRight
                        };

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    fn parse_additive_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // additive-expression:
        //      multiplicative-expression
        //      additive-expression + multiplicative-expression
        //      additive-expression - multiplicative-expression
        let mut expression = self.parse_multiplicative_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::Plus | TokenType::Minus => {
                        // Consume the Plus/Minus token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        // For now it is assumed to be a primary expression
                        let rhs = self.parse_multiplicative_expr()?;

                        let operator = if token == TokenType::Plus {
                            BinaryOperator::Plus
                        } else {
                            BinaryOperator::Minus
                        };

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // multiplicative-expression:
        //      cast-expression
        //      multiplicative-expression * cast-expression
        //      multiplicative-expression / cast-expression
        //      multiplicative-expression % cast-expression
        let mut expression = self.parse_cast_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::Asterisk | TokenType::Slash | TokenType::Percent => {
                        // Consume the Plus/Minus token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        // For now it is assumed to be a primary expression
                        let rhs = self.parse_cast_expr()?;

                        let operator = if token == TokenType::Asterisk {
                            BinaryOperator::Multiply
                        } else if token == TokenType::Slash {
                            BinaryOperator::Divide
                        } else {
                            BinaryOperator::Modulo
                        };

                        let span = Span::new(expression.span.start, rhs.span.end);
                        expression = Node::new(
                            Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                                operator: Node::new(operator, Span::new(start, end)),
                                lhs: expression,
                                rhs,
                            })),
                            span,
                        );
                    }
                    _ => break,
                },
                None => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: "Expected expression, instead got end of file".to_string(),
                        location: None,
                    })
                }
            }
        }
        Ok(expression)
    }

    fn parse_cast_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // cast-expression:
        //      unary-expression
        //      ( type-name ) cast-expression
        let mut typenames: Vec<Node<TypeName>> = Vec::new();
        let mut start_arr: Vec<FileLocation> = Vec::new();

        while let Some((TokenType::OpenParenthesis, paren_start, _)) =
            self.tokenizer.peek_token()?
        {
            // Consume the `(`
            self.tokenizer.next_token()?;

            let mut is_typename = false;
            if let Some((TokenType::Keyword(keyword), _, _)) = self.tokenizer.peek_token()? {
                if let Some(_) = keyword2declspec(&keyword) {
                    // Parse a typename
                    let typename = self.parse_type_name()?;
                    self.accept_token(TokenType::CloseParenthesis)?;

                    typenames.push(typename);
                    start_arr.push(paren_start);
                    is_typename = true;
                }
            }

            if !is_typename {
                let mut expression = self.parse_expr()?;
                let (_, paren_end) = self.accept_token(TokenType::CloseParenthesis)?;

                let expr_start = expression.span.start.clone();
                expression = self.parse_postfix_operators_with_init_expr(expression, expr_start)?;

                // Pop all the typenames and create cast expressions
                for _ in 0..typenames.len() {
                    // Calculate the span of the cast expression
                    // Span = Start of the corresponding OpenParenthesis -> End of the expression's CloseParenthesis
                    let span = Span::new(start_arr.pop().unwrap(), paren_end);
                    // Create and store the cast expression
                    expression = Node::new(
                        Expression::Cast(Box::new(CastExpression {
                            typename: typenames.pop().unwrap(),
                            expression,
                        })),
                        span,
                    )
                }
                return Ok(expression);
            }
        }

        // Parse a Unary Expression
        let mut expression = self.parse_unary_expr()?;

        // Pop all the typenames and create cast expressions
        for _ in 0..typenames.len() {
            // Calculate the span of the cast expression
            // Span = Start of the corresponding OpenParenthesis -> End of the expression's CloseParenthesis
            let span = Span::new(start_arr.pop().unwrap(), expression.span.end);
            // Create and store the cast expression
            expression = Node::new(
                Expression::Cast(Box::new(CastExpression {
                    typename: typenames.pop().unwrap(),
                    expression,
                })),
                span,
            )
        }
        Ok(expression)
    }

    /// This function currently parses only typenames without an abstract-declarator
    fn parse_type_name(&mut self) -> Result<Node<TypeName>, CompilerError> {
        // type-name:
        //       specifier-qualifier-list abstract-declaratoropt
        //
        // specifier-qualifier-list:
        //      type-specifier specifier-qualifier-listopt
        //      type-qualifier specifier-qualifier-listopt
        //      alignment-specifier specifier-qualifier-listopt
        let mut specifier_qualifier_list: Vec<Node<SpecifierQualifier>> = Vec::new();
        while let Some((token, start, end)) = self.tokenizer.peek_token()? {
            match token {
                TokenType::Keyword(keyword) => match keyword2specififerqualifier(&keyword) {
                    Some(spec_qual) => {
                        // Push the specifier/qualifier
                        specifier_qualifier_list.push(Node::new(spec_qual, Span::new(start, end)));
                        // Consume the specifier/qualifier token
                        self.tokenizer.next_token()?;
                    }
                    None => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SyntaxError,
                            message: "Unexpected keyword: {}, expected a Specifier-Qualifier"
                                .to_string(),
                            location: Some(start),
                        })
                    }
                },
                _ => {
                    // TODO: Parse an abstract-declarator here

                    // Ensure that there is atleast one specifier-qualifier
                    if specifier_qualifier_list.len() == 0 {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SyntaxError,
                            message: "No specifier-qualifiers are present in type-name".to_string(),
                            location: Some(start),
                        });
                    }

                    // Calculate the span of the entire type-name
                    // Span = Start of the first specifier-qualifier -> End of the last specifier-qualifier
                    let span = Span::new(
                        specifier_qualifier_list.first().unwrap().span.start,
                        specifier_qualifier_list.last().unwrap().span.end,
                    );

                    // Create and return a TypeName
                    return Ok(Node::new(
                        TypeName {
                            specifier_qualifier_list,
                            abstract_declarator: None,
                        },
                        span,
                    ));
                }
            }
        }
        // This error will occur when the file abruptly ends without the finishing of the type-name
        // (const int*
        //            ^ End of File
        Err(CompilerError {
            kind: CompilerErrorKind::SyntaxError,
            message: "Expected a type-name, instead got end of file".to_string(),
            location: None,
        })
    }

    /// Recursively parses an unary expression
    fn parse_unary_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // unary-expression:
        //      postfix-expression
        //      ++ unary-expression
        //      -- unary-expression
        //      unary-operator cast-expression
        //      sizeof unary-expression
        //      sizeof ( type-name )
        //      _Alignof ( type-name )
        match self.tokenizer.peek_token()? {
            Some((token, start, end)) => match token {
                TokenType::IncrementOperator => {
                    // Consume the `++`
                    self.tokenizer.next_token()?;
                    let unary_expr = self.parse_unary_expr()?;

                    // Calculate the span of the entire unary expression
                    let span = Span::new(start, unary_expr.span.end);
                    // Create and return the Unary Operator Expression
                    Ok(Node::new(
                        Expression::UnaryOperator(Box::new(UnaryOperatorExpression {
                            operator: Node::new(UnaryOperator::PreIncrement, Span::new(start, end)),
                            operand: unary_expr,
                        })),
                        span,
                    ))
                }
                TokenType::DecrementOperator => {
                    // Consume the `--`
                    self.tokenizer.next_token()?;
                    let unary_expr = self.parse_unary_expr()?;

                    // Calculate the span of the entire unary expression
                    let span = Span::new(start, unary_expr.span.end);
                    // Create and return the Unary Operator Expression
                    Ok(Node::new(
                        Expression::UnaryOperator(Box::new(UnaryOperatorExpression {
                            operator: Node::new(UnaryOperator::PreDecrement, Span::new(start, end)),
                            operand: unary_expr,
                        })),
                        span,
                    ))
                }
                TokenType::Keyword(Keyword::Sizeof) => {
                    // Consume the Sizeof Keyword
                    self.tokenizer.next_token()?;

                    // Check if the next token is `(`
                    if let Some((TokenType::OpenParenthesis, paren_start, _)) =
                        self.tokenizer.peek_token()?
                    {
                        self.tokenizer.next_token()?;
                        // If Yes then check if the next_token is a Keyword
                        if let Some((TokenType::Keyword(keyword), _, _)) =
                            self.tokenizer.peek_token()?
                        {
                            // If Yes then check if the Keyword is a Specifier-Qualifier
                            if let Some(_) = keyword2specififerqualifier(&keyword) {
                                // If Yes then it must be a type-name, so parse a type-name
                                let type_name = self.parse_type_name()?;
                                // Accept a `)`
                                let (_, paren_end) =
                                    self.accept_token(TokenType::CloseParenthesis)?;
                                // Calculate the span of the entire sizeof expression
                                // Span = Start of the sizeof Keyword -> End of the CloseParenthesis
                                let span = Span::new(start, paren_end);
                                // Create and return a SizeofType Expression
                                return Ok(Node::new(
                                    Expression::SizeofType(Box::new(type_name)),
                                    span,
                                ));
                            }
                        }
                        // Else If the token is not a specifier-qualifier keyword
                        // Then parse an expression

                        // In case of an expression, there need not be Parenthesis enclosing the expression
                        // But in the current line of code we already have ensured a presence of OpenParenthesis
                        // So that OpenParenthesis becomes a part of the expression and not a part of the sizeof operator
                        // In the unary-expression inside sizeof operator we have to ensure somehow that the postfix operators get noticed
                        // This is because we consume the first OpenParenthesis before calling `parse_expr()`
                        // As a result after parsing an expression we also check for postfix operators
                        // Also we won't check for any other operators like +, -, *, /, %, &&, ==, etc. because they don't come under unary expressions

                        let mut expression = self.parse_expr()?;
                        // But remember that we consumed an open parenthesis at the start
                        self.accept_token(TokenType::CloseParenthesis)?;

                        // Check for any number of postfix operators keeping expression as the already parsed part
                        expression =
                            self.parse_postfix_operators_with_init_expr(expression, paren_start)?;

                        // Calculate the span of the entire SizeofVal Expression
                        // Span = Start of Sizeof Keyword -> End of Expression inside Sizeof
                        let span = Span::new(start, expression.span.end);
                        // Create and return the SizeofVal Expression
                        Ok(Node::new(Expression::SizeofVal(Box::new(expression)), span))
                    } else {
                        // Parse a unary expression
                        let unary_expr = self.parse_unary_expr()?;
                        // Calculate the span of the entire unary expression
                        // Span = Start of first token in the expression -> End of Expression
                        let span = Span::new(start, unary_expr.span.end);
                        // Create and return a unary expression
                        Ok(Node::new(Expression::SizeofVal(Box::new(unary_expr)), span))
                    }
                }
                TokenType::Keyword(Keyword::_Alignof) => {
                    // Consume the Alignof Keyword
                    self.tokenizer.next_token()?;
                    // Accept a `(`
                    self.accept_token(TokenType::OpenParenthesis)?;
                    // Parse a typename
                    let type_name = self.parse_type_name()?;
                    // Accept a `)`
                    let (_, paren_end) = self.accept_token(TokenType::CloseParenthesis)?;
                    // Calculate the span of the entire Alignof Expression
                    // Span = Start of the Alignof Keyword -> End of parenthesis
                    let span = Span::new(start, paren_end);
                    // Create and return an Alignof Expression
                    Ok(Node::new(Expression::Alignof(Box::new(type_name)), span))
                }
                tokentype => {
                    if let Some(unary_op) = token2unaryop(&tokentype) {
                        // Consume the unary operator
                        self.tokenizer.next_token()?;
                        let unary_expr = self.parse_unary_expr()?;

                        // Calculate the span of the entire unary expression
                        let span = Span::new(start, unary_expr.span.end);
                        // Create and return the Unary Operator Expression
                        Ok(Node::new(
                            Expression::UnaryOperator(Box::new(UnaryOperatorExpression {
                                operator: Node::new(unary_op, Span::new(start, end)),
                                operand: unary_expr,
                            })),
                            span,
                        ))
                    } else {
                        self.parse_postfix_expr()
                    }
                }
            },
            None => Err(CompilerError {
                kind: CompilerErrorKind::SyntaxError,
                message: "Expected an expression, instead got end of file".to_string(),
                location: None,
            }),
        }
    }

    fn parse_postfix_operators_with_init_expr(
        &mut self,
        mut expression: Node<Expression>,
        expr_start: FileLocation,
    ) -> Result<Node<Expression>, CompilerError> {
        // postfix-expression:
        //      primary-expression
        //      postfix-expression [ expression ]
        //      postfix-expression ( argument-expression-listopt )
        //      postfix-expression . identifier
        //      postfix-expression -> identifier
        //      postfix-expression ++
        //      postfix-expression --
        while let Some((token, start, end)) = self.tokenizer.peek_token()? {
            match token {
                TokenType::IncrementOperator => {
                    // Consume the `++`
                    self.tokenizer.next_token()?;
                    // Calculate span of the entire postfix expression
                    // Span = Start of the previously parsed expression -> End of postfix operator
                    let span = Span::new(expr_start, end);
                    // Create and store a postfix expression using the already passed expression plus the postfix operator
                    expression = Node::new(
                        Expression::UnaryOperator(Box::new(UnaryOperatorExpression {
                            operator: Node::new(
                                UnaryOperator::PostIncrement,
                                Span::new(start, end),
                            ),
                            operand: expression,
                        })),
                        span,
                    );
                }
                TokenType::DecrementOperator => {
                    // Consume the `--`
                    self.tokenizer.next_token()?;
                    // Calculate span of the entire postfix expression
                    // Span = Start of the previously parsed expression -> End of postfix operator
                    let span = Span::new(expr_start, end);
                    // Create and store a postfix expression using the already passed expression plus the postfix operator
                    expression = Node::new(
                        Expression::UnaryOperator(Box::new(UnaryOperatorExpression {
                            operator: Node::new(
                                UnaryOperator::PostDecrement,
                                Span::new(start, end),
                            ),
                            operand: expression,
                        })),
                        span,
                    );
                }
                TokenType::DotOperator | TokenType::ArrowOperator => {
                    self.tokenizer.next_token()?;
                    // Expect an identifier
                    match self.tokenizer.next_token()? {
                        Some((TokenType::Identifier(identifier), id_start, id_end)) => {
                            // Calculate span of the entire postfix expression
                            // Span = Start of the previously parsed expression -> End of postfix operator
                            let span = Span::new(expr_start, id_end);
                            // Store the right member access operator
                            let operator = if token == TokenType::DotOperator {
                                MemberOperator::Direct
                            } else {
                                MemberOperator::Indirect
                            };
                            // Create and store a postfix expression using the already passed expression plus the postfix operator
                            expression = Node::new(
                                Expression::Member(Box::new(MemberExpression {
                                    operator: Node::new(operator, Span::new(start, end)),
                                    expression,
                                    identifier: Node::new(identifier, Span::new(id_start, id_end)),
                                })),
                                span,
                            );
                        }
                        Some((unexpected, unexpected_start, _)) => {
                            // This error will occur when there is no identifier specified after dot/arrow operator
                            // struct_instance-> ;
                            // struct_instance.  ;
                            //                  ^ Missing identifiers
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SyntaxError,
                                message: format!(
                                    "Expected an identifier, instead got {:?}",
                                    unexpected
                                ),
                                location: Some(unexpected_start),
                            });
                        }
                        None => {
                            // This error will occur when there is an end of file and no identifier specified after dot/arrow operator
                            // struct_instance->
                            // struct_instance.
                            //                  ^^ End of file, instead of identifiers
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SyntaxError,
                                message: "Expected an identifier, instead got end of file"
                                    .to_string(),
                                location: None,
                            });
                        }
                    }
                }
                TokenType::OpenSquareBracket => {
                    // Consume the `[`
                    self.tokenizer.next_token()?;
                    // Parse the expression that will be used as index into the array
                    let index_expr = self.parse_expr()?;
                    // Accept a `]`
                    let (_, bracket_end) = self.accept_token(TokenType::CloseSquareBracket)?;
                    // Calculate span of the entire postfix expression
                    // Span = Start of the previously parsed expression -> End of postfix operator
                    let span = Span::new(expr_start, bracket_end);
                    // Create and return a BinaryOperatorExpression with Indexing `[]` as the binary operator
                    // And the already passed expression as the LHS and index_expr as RHS
                    expression = Node::new(
                        Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
                            operator: Node::new(BinaryOperator::Index, Span::new(start, end)),
                            lhs: expression,
                            rhs: index_expr,
                        })),
                        span,
                    );
                }
                TokenType::OpenParenthesis => {
                    // Consume the `(`
                    self.tokenizer.next_token()?;
                    // Parse the Argument Expression List
                    let argument_expr_list = self.parse_argument_expression_list()?;
                    // Conume the `)`
                    let (_, paren_end) = self.accept_token(TokenType::CloseParenthesis)?;

                    // Calculate the span of the entire postfix expression
                    // Span = Start of the previously parsed expression -> End of Parenthesis
                    let span = Span::new(expr_start, paren_end);
                    // Create and return a CallExpression with it's arguments
                    expression = Node::new(
                        Expression::Call(Box::new(CallExpression {
                            callee: expression,
                            argument_expr_list,
                        })),
                        span,
                    )
                }
                _ => {
                    // No need to modify the already parsed expression
                    // As there is no postfix operator to be taken care of
                    // Hence return the expression
                    return Ok(expression);
                }
            }
        }
        Err(CompilerError {
            kind: CompilerErrorKind::SyntaxError,
            message: "Expected a postfix-expression, instead found end of file".to_string(),
            location: None,
        })
    }

    fn parse_postfix_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // postfix-expression:
        //      primary-expression
        //      postfix-expression [ expression ]
        //      postfix-expression ( argument-expression-listopt )
        //      postfix-expression . identifier
        //      postfix-expression -> identifier
        //      postfix-expression ++
        //      postfix-expression --
        //      ( type-name ) { initializer-list }
        //      ( type-name ) { initializer-list , }
        let expression = self.parse_primary_expr()?;
        let expr_start = expression.span.start.clone();
        // Parse the postfix operators
        self.parse_postfix_operators_with_init_expr(expression, expr_start)
    }

    /// Iteratively parses argument expression list
    fn parse_argument_expression_list(&mut self) -> Result<Vec<Node<Expression>>, CompilerError> {
        // argument-expression-list:
        //       assignment-expression
        //       argument-expression-list , assignment-expression
        let mut argument_expr_list: Vec<Node<Expression>> = Vec::new();

        let mut expect_argument = false;

        while !matches!(
            self.tokenizer.peek_token()?,
            Some((TokenType::CloseParenthesis, _, _))
        ) {
            // An argument is simply an assignment expression
            let assignment_expr = self.parse_assignment_expr()?;
            // Push the parsed argument
            argument_expr_list.push(assignment_expr);

            if let Some((TokenType::Comma, _, _)) = self.tokenizer.peek_token()? {
                expect_argument = true;
                self.tokenizer.next_token()?;
            } else {
                expect_argument = false;
            }
        }

        // This is to handle a case where the below C code should not be considered valid
        // function_call(const float param1, )
        //                                  ^^ Missing argument
        // I.e. when a comma is consumed, but the next token is ) then the while loop will exit and return arguments successfully
        // But that is not valid C syntax, a comma cannot be present if no argument is present after it
        if expect_argument {
            return Err(CompilerError {
                kind: CompilerErrorKind::SyntaxError,
                message:
                    "Expected assignment expression for argument in the function call after `,` instead got `)`"
                        .to_string(),
                location: None,
            });
        }
        // Return the argument expression list
        Ok(argument_expr_list)
    }

    fn parse_primary_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
        // primary-expression:
        //      identifier
        //      constant
        //      string-literal
        //      ( expression )
        //      generic-selection
        let expression: Node<Expression> = match self.tokenizer.next_token()? {
            Some((token, start, end)) => match token {
                TokenType::Identifier(identifier) => {
                    Node::new(Expression::Identifier(identifier), Span::new(start, end))
                }
                TokenType::Integer(integer) => Node::new(
                    Expression::Constant(Constant::Integer(integer)),
                    Span::new(start, end),
                ),
                TokenType::FloatingPoint(floatingpoint) => Node::new(
                    Expression::Constant(Constant::Float(floatingpoint)),
                    Span::new(start, end),
                ),
                TokenType::Character(ch) => Node::new(
                    Expression::Constant(Constant::Character(ch)),
                    Span::new(start, end),
                ),
                TokenType::StringLiteral(strliteral) => {
                    Node::new(Expression::StringLiteral(strliteral), Span::new(start, end))
                }
                TokenType::OpenParenthesis => {
                    // TODO: This is probably never gonna be reached now as cast expressions have ( type-name ) in there grammar
                    // And we check if inside the Parenthesis is a type-name, if not then we parse an expression
                    // So all such expressions inside parenthesis type of expressions will be parsed in the `parse_cast_expression` function

                    // Parse the entire expression inside the Parenthesis
                    // A key thinking behind this is that the `parse_expr()` function
                    // will keep parsing till it encounters something other than a recognized operator, constant or an identifier or another OpenParenthesis
                    // So when encountered a CloseParenthesis, the function should return the existing expression, hence no need to check for that
                    let expr = self.parse_expr()?;
                    self.accept_token(TokenType::CloseParenthesis)?;
                    expr
                }
                _ => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SyntaxError,
                        message: format!("Expected expression, instead found: {:?}", token),
                        location: Some(start),
                    })
                }
            },
            _ => {
                return Err(CompilerError {
                    kind: CompilerErrorKind::SyntaxError,
                    message: "Expected expression, instead found end of file".to_string(),
                    location: None,
                })
            }
        };
        Ok(expression)
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

    /// Forces the next token to be the given `tokentype`
    /// Returns (start, end) both being character indices in the file
    fn accept_token(
        &mut self,
        tokentype: TokenType,
    ) -> Result<(FileLocation, FileLocation), CompilerError> {
        match self.tokenizer.next_token()? {
            Some((token, start, end)) => {
                if token == tokentype {
                    Ok((start, end))
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
