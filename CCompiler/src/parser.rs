//! Module for parsing the tokenized code into an AST tree according to the C17 standard.

use core::fmt;
use debug_tree::*;

use crate::errors::{CompilerError, CompilerErrorKind};
use crate::node::{Node, Span};
use crate::tokenizer::{FloatingPointType, IntegerType, Keyword, TokenType, Tokenizer};

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
    Integer(IntegerType),     // TODO: Abstract this later into long, short integers
    Float(FloatingPointType), // TODO: Abstract this later into long, short floating point numbers
}

type StringLiteral = Vec<char>;

#[derive(Debug)]
pub enum BinaryOperator {
    /// `lhs[rhs]`
    Index,
    /// `lhs * rhs`
    Multiply,
    /// `lhs / rhs`
    Divide,
    /// `lhs % rhs`
    Modulo,
    /// `lhs + rhs`
    Plus,
    /// `lhs - rhs`
    Minus,
    /// `lhs << rhs`
    ShiftLeft,
    /// `lhs >> rhs`
    ShiftRight,
    /// `lhs < rhs`
    Less,
    /// `lhs > rhs`
    Greater,
    /// `lhs <= rhs`
    LessOrEqual,
    /// `lhs >= rhs`
    GreaterOrEqual,
    /// `lhs == rhs`
    Equals,
    /// `lhs != rhs`
    NotEquals,
    /// `lhs & rhs`
    BitwiseAnd,
    /// `lhs ^ rhs`
    BitwiseXor,
    /// `lhs | rhs`
    BitwiseOr,
    /// `lhs && rhs`
    LogicalAnd,
    /// `lhs || rhs`
    LogicalOr,
    /// `lhs = rhs`
    Assign,
    /// `lhs *= rhs`
    AssignMultiply,
    /// `lhs /= rhs`
    AssignDivide,
    /// `lhs %= rhs`
    AssignModulo,
    /// `lhs += rhs`
    AssignPlus,
    /// `lhs -= rhs`
    AssignMinus,
    /// `lhs <<= rhs`
    AssignShiftLeft,
    /// `lhs >>= rhs`
    AssignShiftRight,
    /// `lhs &= rhs`
    AssignBitwiseAnd,
    /// `lhs ^= rhs`
    AssignBitwiseXor,
    /// `lhs |= rhs`
    AssignBitwiseOr,
}

#[derive(Debug)]
struct BinaryOperatorExpression {
    operator: Node<BinaryOperator>,
    lhs: Node<Expression>,
    rhs: Node<Expression>,
}

#[derive(Debug)]
enum Expression {
    Identifier(String), // TODO: This should be a pointer to the symbol table entry of the identifier
    Constant(Constant),
    StringLiteral(StringLiteral),
    BinaryOperator(Box<BinaryOperatorExpression>),
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
    declarator: Option<Node<Declarator>>,
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
    // int                      function(DeclarationSpecifiers param1, DeclarationSpecifiers param2);
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
    Declaration(Declaration),
    Statement(Statement),
}

#[derive(Debug)]
enum Statement {
    LabeledStatement,
    CompoundStatement(Vec<Node<BlockItem>>),
    ExpressionStatement(Expression),
    SelectionStatement,
    IterationStatement,
    // Jump Statements
    GotoStatement,
    ContinueStatement,
    BreakStatement,
    ReturnStatement(Node<Expression>),
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

fn display_expr(expression: &Node<Expression>) {
    match &expression.node {
        Expression::Constant(constant) => {
            add_branch!("Constant");
            match constant {
                Constant::Float(float) => add_leaf!("Float -> {:?} {}", float, expression.span),
                Constant::Integer(int) => add_leaf!("Integer -> {:?}, {}", int, expression.span),
            }
        }
        Expression::Identifier(identifier) => {
            add_leaf!("Identifier -> \"{}\" {}", identifier, expression.span)
        }
        Expression::BinaryOperator(binaryexpr) => {
            add_branch!("BinaryOperatorExpression {}", expression.span);
            {
                add_branch!(
                    "Operator -> {:?} {}",
                    binaryexpr.operator.node,
                    binaryexpr.operator.span
                );
            }
            {
                add_branch!("LHS");
                display_expr(&binaryexpr.lhs);
            }
            {
                add_branch!("RHS");
                display_expr(&binaryexpr.rhs);
            }
        }
        Expression::StringLiteral(_) => todo!(),
    }
}

fn display_statement(statement: &Statement, span: &Span) {
    match &statement {
        Statement::ReturnStatement(expression) => {
            add_branch!("Return Statement {}", span);
            display_expr(&expression);
        }
        Statement::CompoundStatement(block) => {
            add_branch!("Compound Statement {}", span);
            if !block.is_empty() {
                for blockitem in block {
                    match &blockitem.node {
                        BlockItem::Declaration(_) => todo!(),
                        BlockItem::Statement(statement) => {
                            display_statement(&statement, &blockitem.span);
                        }
                    }
                }
            } else {
                add_leaf!("Empty");
            }
        }
        _ => todo!(),
    }
}

/// Display Declaration Specifiers
fn display_declspec(specifiers: &Vec<Node<DeclarationSpecifier>>) {
    add_branch!("DeclarationSpecifiers");
    for declspec in specifiers {
        add_leaf!("{}", declspec);
    }
}

fn display_funcdeclarator(declarator: &FunctionDeclarator, span: Span) {
    add_branch!("FunctionDeclarator");
    add_leaf!(
        "Identifier -> \"{}\" {}",
        declarator.identifier,
        Span::new(span.start, span.start + declarator.identifier.len())
    );

    // Add Parameters
    add_branch!("FunctionParameters");
    for param in &declarator.parameters {
        // Add FunctionParameter
        add_branch!("FunctionParameter {}", param.span);
        display_declspec(&param.node.specifiers);

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
                _ => panic!("Parameter Declarator should not be Function Declarator!"),
            },
            None => add_leaf!("DirectDeclarator -> None"),
        }
    }
}

// TODO: Have this be debug mode only
pub fn display_translationunit(tunit: &TranslationUnit) {
    defer_print!();
    add_branch!("TranslationUnit");
    for extdecl in &tunit.external_declarations {
        match &extdecl.node {
            ExternalDeclaration::Declaration(decl) => {
                add_branch!("Declaration {}", extdecl.span);
                // Add declaration specifiers
                display_declspec(&decl.specifiers);
                // Add declarator
                match &decl.declarator.node {
                    Declarator::FunctionDeclarator(funcdecl) => {
                        display_funcdeclarator(funcdecl, decl.declarator.span);
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
                {
                    add_branch!("FunctionDeclaration");
                    display_declspec(&funcdef.specifiers);
                    display_funcdeclarator(&funcdef.declarator.node, funcdef.declarator.span);
                }

                add_branch!("FunctionBody");
                if let Statement::CompoundStatement(block) = &funcdef.body.node {
                    if block.is_empty() {
                        add_leaf!("Empty");
                    } else {
                        display_statement(&funcdef.body.node, &funcdef.body.span);
                    }
                } else {
                    panic!("FunctionBody must be a compound statement");
                }
            }
        }
    }
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

                    match self.tokenizer.next_token()? {
                        Some((token, start, _)) => match token {
                            TokenType::OpenBrace => {
                                // Parse a function definition
                                if let Declarator::FunctionDeclarator(fdecl) =
                                    declaration.declarator.node
                                {
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
                                                    fdecl,
                                                    declaration.declarator.span,
                                                ),
                                                body: funcbody,
                                            },
                                        ),
                                        funcdef_span,
                                    ));
                                } else {
                                    // This error should be removed in the future
                                    // Because currently init-declarator-list is not supported in a direct declaration
                                    // So if `{` is encountered, then it must be a function definition
                                    // Reaching this line of code indicates the C code looks like this:
                                    // const int variable_declaration{};
                                    //                               ^^ Unexpected `{`
                                    return Err(CompilerError {
                                        kind: CompilerErrorKind::SyntaxError,
                                        message: "Unexpected token: `{`, statement is not a valid function declarat".to_string(),
                                        location: Some(start),
                                    });
                                }
                            }
                            TokenType::Semicolon => {
                                // Calculate the span for the declaration
                                let declspan_start = declaration.specifiers[0].span.start; // Here we assume that a declaration will always have atleast one specifier
                                let declspan =
                                    Span::new(declspan_start, declaration.declarator.span.end);

                                tranlation_unit.external_declarations.push(Node::new(
                                    ExternalDeclaration::Declaration(declaration),
                                    declspan,
                                ));
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
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

    /// Note: This function doesn't consume either of OpenBrace and CloseBrace tokens associated with it.
    /// It is the caller's responsibility to check for OpenBrace and consume a CloseBrace after calling this function.
    fn parse_compound_stmt(&mut self) -> Result<Node<Statement>, CompilerError> {
        let mut statements: Vec<Node<BlockItem>> = Vec::new();

        while !matches!(
            self.tokenizer.peek_token()?,
            Some((TokenType::CloseBrace, _, _))
        ) {
            match self.tokenizer.next_token()? {
                Some((token, start, _)) => match token {
                    TokenType::Keyword(Keyword::Case | Keyword::Default) => {
                        // Labeled Statement
                        todo!()
                    }
                    TokenType::Keyword(Keyword::If | Keyword::Switch) => {
                        // Selection Statement
                        todo!()
                    }
                    TokenType::Keyword(Keyword::While | Keyword::For | Keyword::Do) => {
                        // Iteration Statement
                        todo!()
                    }
                    TokenType::Keyword(Keyword::Return) => {
                        // Return -> Jump Statement
                        let expression = self.parse_expr()?;
                        let (_, semicolon_end) = self.accept_token(TokenType::Semicolon)?;

                        // Calculate span of the entire return statement
                        // Span of return statement = (start of the return keyword, end of the semicolon token)
                        let span = Span::new(start, semicolon_end);
                        // Create and store the actual return statement
                        statements.push(Node::new(
                            BlockItem::Statement(Statement::ReturnStatement(expression)),
                            span,
                        ));
                    }
                    TokenType::Keyword(Keyword::Goto | Keyword::Continue | Keyword::Break) => {
                        todo!()
                    }
                    TokenType::Keyword(keyword) => match keyword2declspec(&keyword) {
                        Some(_) => {
                            // Parse a declaration?
                            todo!()
                        }
                        None => {
                            todo!("Keyword encountered is: {:?}", keyword)
                        }
                    },
                    TokenType::Identifier(_) => {
                        // Either it can be a labeled statement
                        // Or an expression statement
                        todo!()
                    }
                    TokenType::OpenBrace => {
                        // Parse a compound statement
                        let compound_stmt = self.parse_compound_stmt()?;
                        // Create a block item using the node and span of the compound statement
                        // The span of the block item will be the same as that of the compound statement
                        statements.push(Node::new(
                            BlockItem::Statement(compound_stmt.node),
                            compound_stmt.span,
                        ));
                        // Accept a closing brace
                        self.accept_token(TokenType::CloseBrace)?;
                    }
                    _ => todo!("The start of the statement is: {:?}", token),
                },
                _ => todo!(),
            };
        }

        // Calculate the span of the compound statement
        let span = if !statements.is_empty() {
            // If the compound statement is not empty then the span is the start index of the first statement
            // and the end of the last statement
            Span::new(
                statements.first().unwrap().span.start,
                statements.last().unwrap().span.end,
            )
        } else {
            // peek_token() has to return some token as the while loop before this will only exit
            // when the next token is CloseBrace
            // If it were to retunrn None token then it would've been handled in the while loop itself
            let start = self.tokenizer.peek_token()?.unwrap().1;
            // If the compound statement is empty then the span is the start index of the CloseBrace
            Span::new(start, start)
        };

        // Create and return the compound statement
        Ok(Node::new(Statement::CompoundStatement(statements), span))
    }

    /// Note: This function doesn't consume a semicolon at the end.
    /// That must be handled by the calling function
    fn parse_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
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
        let mut expression = self.parse_primary_expr()?;

        // Doing the parsing iteratively instead of recursively
        loop {
            match self.tokenizer.peek_token()? {
                Some((token, start, end)) => match token {
                    TokenType::Asterisk | TokenType::Slash | TokenType::Percent => {
                        // Consume the Plus/Minus token
                        self.tokenizer.next_token()?;
                        // Parse the RHS expression
                        // For now it is assumed to be a primary expression
                        let rhs = self.parse_primary_expr()?;

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

    fn parse_primary_expr(&mut self) -> Result<Node<Expression>, CompilerError> {
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
                TokenType::OpenParenthesis => {
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
    fn accept_token(&mut self, tokentype: TokenType) -> Result<(usize, usize), CompilerError> {
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
