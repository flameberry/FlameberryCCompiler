use std::fmt;

use debug_tree::*;

use crate::node::{FileLocation, Node, Span};
use crate::tokenizer::{FloatingPointType, IntegerType};

#[derive(Debug, Clone)]
pub enum TypeSpecifier {
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

#[derive(Debug, Clone)]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
    Atomic,
}

#[derive(Debug, Clone)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    ThreadLocal,
    Auto,
    Register,
}

#[derive(Debug)]
pub enum Constant {
    Integer(IntegerType),
    Float(FloatingPointType),
    Character(char),
}

pub type StringLiteral = String;

#[derive(Debug)]
pub enum UnaryOperator {
    /// `operand++`
    PostIncrement,
    /// `operand--`
    PostDecrement,
    /// `++operand`
    PreIncrement,
    /// `--operand`
    PreDecrement,
    /// `&operand`
    Address,
    /// `*operand`
    Indirection,
    /// `+operand`
    Plus,
    /// `-operand`
    Minus,
    /// `~operand`
    Complement,
    /// `!operand`
    Negate,
}

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
pub enum MemberOperator {
    /// operator.
    Direct,
    /// operator->
    Indirect,
}

#[derive(Debug)]
pub struct UnaryOperatorExpression {
    pub operator: Node<UnaryOperator>,
    pub operand: Node<Expression>,
}

#[derive(Debug)]
pub struct BinaryOperatorExpression {
    pub operator: Node<BinaryOperator>,
    pub lhs: Node<Expression>,
    pub rhs: Node<Expression>,
}

#[derive(Debug)]
pub struct TernaryOperatorExpression {
    pub condition: Node<Expression>,
    pub if_expr: Node<Expression>,
    pub else_expr: Node<Expression>,
}

#[derive(Debug)]
pub struct MemberExpression {
    pub operator: Node<MemberOperator>,
    pub expression: Node<Expression>,
    pub identifier: Node<String>,
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: Node<Expression>,
    pub argument_expr_list: Vec<Node<Expression>>,
}

#[derive(Debug)]
pub struct CastExpression {
    pub typename: Node<TypeName>,
    pub expression: Node<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String), // TODO: This should be a pointer to the symbol table entry of the identifier
    Constant(Constant),
    StringLiteral(StringLiteral),
    UnaryOperator(Box<UnaryOperatorExpression>),
    BinaryOperator(Box<BinaryOperatorExpression>),
    TernaryOperator(Box<TernaryOperatorExpression>),
    SizeofType(Box<Node<TypeName>>),
    SizeofVal(Box<Node<Expression>>),
    Alignof(Box<Node<TypeName>>),
    Member(Box<MemberExpression>),
    Call(Box<CallExpression>),
    Cast(Box<CastExpression>),
    Comma(Vec<Node<Expression>>),
}

#[derive(Debug, Clone)]
pub enum FunctionSpecifier {
    Inline,
    NoReturn,
}

#[derive(Debug, Clone)]
pub enum DeclarationSpecifier {
    StorageClassSpecifier(StorageClassSpecifier),
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
    FunctionSpecifier(FunctionSpecifier),
    // TODO: Add alignment specifier, etc. according to C17 standard
}

/// Used to store parsed type-names
#[derive(Debug)]
pub enum SpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
}

#[derive(Debug)]
pub struct TypeName {
    pub specifier_qualifier_list: Vec<Node<SpecifierQualifier>>,
    pub abstract_declarator: Option<Node<Declarator>>,
}

#[derive(Debug, Clone)]
pub enum Declarator {
    FunctionDeclarator(FunctionDeclarator),
    DirectDeclarator(String), // Currently this is just equivalent to an Identifier (Arrays, Pointers, etc are not considered)
}

#[derive(Debug)]
pub enum Statement {
    LabeledStatement(Box<LabeledStatement>),
    CaseStatement(Box<CaseStatement>),
    DefaultStatement(Box<Node<Statement>>),
    CompoundStatement(Vec<Node<BlockItem>>),
    ExpressionStatement(Option<Node<Expression>>),
    IfStatement(Box<IfStatement>),
    SwitchStatement(Box<SwitchStatement>),
    WhileStatement(Box<WhileStatement>),
    DoWhileStatement(Box<DoWhileStatement>),
    ForStatement(Box<ForStatement>),
    BreakStatement,
    ContinueStatement,
    ReturnStatement(Node<Expression>),
    GotoStatement(Node<String>), // Identifier is considered as a String for now
}

#[derive(Debug)]
pub struct LabeledStatement {
    pub identifier: Node<String>,
    pub statement: Node<Statement>,
}

#[derive(Debug)]
pub struct CaseStatement {
    pub constexpr: Node<Expression>,
    pub statement: Node<Statement>,
}

#[derive(Debug)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Statement),
}

#[derive(Debug)]
pub struct IfStatement {
    pub expression: Node<Expression>,
    pub if_block: Node<Statement>,
    pub else_block: Option<Node<Statement>>,
}

#[derive(Debug)]
pub struct SwitchStatement {
    pub expression: Node<Expression>,
    pub statement: Node<Statement>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub expression: Node<Expression>,
    pub statement: Node<Statement>,
}

#[derive(Debug)]
pub struct DoWhileStatement {
    pub statement: Node<Statement>,
    pub expression: Node<Expression>,
}

#[derive(Debug)]
pub struct ForStatement {
    pub initializer: Node<ForInitializer>,
    pub condition: Option<Node<Expression>>,
    pub step: Option<Node<Expression>>,
    pub statement: Node<Statement>,
}

#[derive(Debug)]
pub enum ForInitializer {
    Empty,
    Expression(Expression),
    Declaration(Declaration),
}

#[derive(Debug)]
pub enum ExternalDeclaration {
    FunctionDefinition(FunctionDefinition),
    Declaration(Declaration),
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub specifiers: Vec<Node<DeclarationSpecifier>>,
    pub declarator: Node<FunctionDeclarator>,
    pub body: Node<Statement>, // Function body can be one statement or a compound statement
}

#[derive(Debug, Clone)]
pub struct FunctionDeclarator {
    pub identifier: String,
    pub parameters: Vec<Node<FunctionParameter>>,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    // This key difference between this struct and `Declaration` struct is that the `declarator` is Optional here
    pub specifiers: Vec<Node<DeclarationSpecifier>>,
    pub declarator: Option<Node<Declarator>>,
}

#[derive(Debug)]
pub struct Declaration {
    // Function Declaration
    // int                      function(DeclarationSpecifiers param1, DeclarationSpecifiers param2);
    // ^^^                      ^^^
    // DeclarationSpecifier     Declarator

    // Variable Declaration
    // int                      variable;
    // ^^^                      ^^^
    // DeclarationSpecifier     Declarator
    pub specifiers: Vec<Node<DeclarationSpecifier>>,
    pub init_declarators: Vec<Node<InitDeclarator>>,
}

#[derive(Debug)]
pub struct InitDeclarator {
    pub declarator: Node<Declarator>,
    pub initializer: Option<Node<Initializer>>,
}

#[derive(Debug)]
pub enum Initializer {
    AssignmentExpression(Expression),
}

/// Grammar for Translation Unit according to C17 ISO standard:
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
    pub external_declarations: Vec<Node<ExternalDeclaration>>,
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

impl fmt::Display for SpecifierQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpecifierQualifier::TypeSpecifier(specifier) => write!(f, "{:?}", specifier),
            SpecifierQualifier::TypeQualifier(qualifier) => write!(f, "{:?}", qualifier),
        }
    }
}

pub fn display_typename(type_name: &Node<TypeName>) {
    add_branch!("SpecifierQualifiers");
    for spec_qual in &type_name.node.specifier_qualifier_list {
        add_leaf!("{}", spec_qual);
    }
}

pub fn display_expr(expression: &Expression, span: &Span) {
    match &expression {
        Expression::Constant(constant) => {
            add_branch!("Constant");
            match constant {
                Constant::Float(float) => add_leaf!("Float -> {:?} {}", float, span),
                Constant::Integer(int) => add_leaf!("Integer -> {:?}, {}", int, span),
                Constant::Character(ch) => add_leaf!("Character -> '{}'", ch),
            }
        }
        Expression::StringLiteral(strliteral) => {
            add_leaf!("StringLiteral -> \"{}\" {}", strliteral, span);
        }
        Expression::Identifier(identifier) => {
            add_leaf!("Identifier -> \"{}\" {}", identifier, span)
        }
        Expression::UnaryOperator(unaryexpr) => {
            add_branch!("UnaryOperatorExpression {}", span);
            {
                add_branch!(
                    "Operator -> {:?} {}",
                    unaryexpr.operator.node,
                    unaryexpr.operator.span
                );
            }
            {
                add_branch!("Expression");
                display_expr(&unaryexpr.operand.node, &unaryexpr.operand.span);
            }
        }
        Expression::BinaryOperator(binaryexpr) => {
            add_branch!("BinaryOperatorExpression {}", span);
            {
                add_branch!(
                    "Operator -> {:?} {}",
                    binaryexpr.operator.node,
                    binaryexpr.operator.span
                );
            }
            {
                add_branch!("LHS");
                display_expr(&binaryexpr.lhs.node, &binaryexpr.lhs.span);
            }
            {
                add_branch!("RHS");
                display_expr(&binaryexpr.rhs.node, &binaryexpr.rhs.span);
            }
        }
        Expression::TernaryOperator(ternaryexpr) => {
            add_branch!("TernaryOperatorExpression {}", span);
            {
                add_branch!("Condition");
                display_expr(&ternaryexpr.condition.node, &ternaryexpr.condition.span);
            }
            {
                add_branch!("IfExpression");
                display_expr(&ternaryexpr.if_expr.node, &ternaryexpr.if_expr.span);
            }
            {
                add_branch!("ElseExpression");
                display_expr(&ternaryexpr.else_expr.node, &ternaryexpr.else_expr.span);
            }
        }
        Expression::SizeofType(type_name) => {
            add_branch!("SizeofTypeExpression {}", span);
            {
                add_branch!("TypeName");
                display_typename(&type_name);
            }
        }
        Expression::SizeofVal(unary_exp) => {
            add_branch!("SizeofValExpression {}", span);
            display_expr(&unary_exp.node, &unary_exp.span);
        }
        Expression::Alignof(type_name) => {
            add_branch!("AlignofExpression {}", span);
            {
                add_branch!("TypeName");
                display_typename(&type_name);
            }
        }
        Expression::Member(member_expr) => {
            add_branch!("MemberExpression {}", span);
            {
                add_leaf!(
                    "MemberOperator -> {:?} {}",
                    member_expr.operator.node,
                    member_expr.operator.span
                );
            }
            {
                add_branch!("MemberExpressionExpression");
                display_expr(&member_expr.expression.node, &member_expr.expression.span);
            }
            {
                add_leaf!(
                    "MemberExpressionIdentifier -> \"{}\" {}",
                    member_expr.identifier.node,
                    member_expr.identifier.span
                );
            }
        }
        Expression::Call(call_expr) => {
            add_branch!("CallExpression {}", span);
            {
                add_branch!("CalleeExpression");
                display_expr(&call_expr.callee.node, &call_expr.callee.span);
            }
            {
                add_branch!("ArgumentExpressionList");
                if call_expr.argument_expr_list.is_empty() {
                    add_leaf!("Empty");
                } else {
                    for arg in &call_expr.argument_expr_list {
                        add_branch!("ArgumentExpression");
                        display_expr(&arg.node, &arg.span);
                    }
                }
            }
        }
        Expression::Cast(cast_expr) => {
            add_branch!("CastExpression {}", span);
            {
                add_branch!("Typename");
                display_typename(&cast_expr.typename);
            }
            {
                add_branch!("Expression");
                display_expr(&cast_expr.expression.node, &cast_expr.expression.span);
            }
        }
        Expression::Comma(expressions) => {
            add_branch!("CommaExpression {}", span);
            for expr in expressions {
                display_expr(&expr.node, &expr.span);
            }
        }
    }
}

pub fn display_statement(statement: &Statement, span: &Span) {
    match &statement {
        Statement::LabeledStatement(statement) => {
            add_branch!("LabeledStatement {}", span);
            {
                add_leaf!(
                    "Identifier -> \"{}\" {}",
                    statement.identifier.node,
                    statement.identifier.span
                );
            }
            {
                add_branch!("LabeledBlock");
                display_statement(&statement.statement.node, &statement.statement.span);
            }
        }
        Statement::CaseStatement(casestmt) => {
            add_branch!("CaseStatement {}", span);
            {
                add_branch!("CaseExpression");
                display_expr(&casestmt.constexpr.node, &casestmt.constexpr.span);
            }
            {
                add_branch!("CaseBlock");
                display_statement(&casestmt.statement.node, &casestmt.statement.span);
            }
        }
        Statement::DefaultStatement(statement) => {
            add_branch!("DefaultStatement {}", span);
            {
                add_branch!("DefaultBlock");
                display_statement(&statement.node, &statement.span);
            }
        }
        Statement::ReturnStatement(expression) => {
            add_branch!("ReturnStatement {}", span);
            display_expr(&expression.node, span);
        }
        Statement::CompoundStatement(block) => {
            add_branch!("CompoundStatement {}", span);
            if !block.is_empty() {
                for blockitem in block {
                    match &blockitem.node {
                        BlockItem::Declaration(declaration) => {
                            display_declaration(declaration, &blockitem.span)
                        }
                        BlockItem::Statement(statement) => {
                            display_statement(&statement, &blockitem.span);
                        }
                    }
                }
            } else {
                add_leaf!("Empty");
            }
        }
        Statement::ExpressionStatement(statement) => {
            add_branch!("ExpressionStatement {}", span);
            {
                add_branch!("Expression");
                if let Some(expression) = &statement {
                    display_expr(&expression.node, &expression.span);
                } else {
                    add_leaf!("Empty");
                }
            }
        }
        Statement::IfStatement(if_statement) => {
            add_branch!("IfStatement {}", span);
            {
                add_branch!("IfExpression");
                display_expr(&if_statement.expression.node, &if_statement.expression.span);
            }
            {
                add_branch!("ThenStatement");
                display_statement(&if_statement.if_block.node, &if_statement.if_block.span);
            }
            if let Some(else_stmt) = &if_statement.else_block {
                add_branch!("ElseStatement");
                display_statement(&else_stmt.node, &else_stmt.span);
            }
        }
        Statement::SwitchStatement(statement) => {
            add_branch!("SwitchStatement {}", span);
            {
                add_branch!("SwitchExpression");
                display_expr(&statement.expression.node, &statement.expression.span);
            }
            {
                add_branch!("SwitchBlock");
                display_statement(&statement.statement.node, &statement.statement.span);
            }
        }
        Statement::WhileStatement(while_stmt) => {
            add_branch!("WhileStatement {}", span);
            {
                add_branch!("WhileExpression");
                display_expr(&while_stmt.expression.node, &while_stmt.expression.span);
            }
            {
                add_branch!("WhileBlock");
                display_statement(&while_stmt.statement.node, &while_stmt.statement.span);
            }
        }
        Statement::DoWhileStatement(statement) => {
            add_branch!("DoWhileStatement {}", span);
            {
                add_branch!("DoBlock");
                display_statement(&statement.statement.node, &statement.statement.span);
            }
            {
                add_branch!("DoWhileExpression");
                display_expr(&statement.expression.node, &statement.expression.span);
            }
        }
        Statement::ForStatement(statement) => {
            add_branch!("ForStatement {}", span);
            {
                add_branch!("ForInitializer");
                match &statement.initializer.node {
                    ForInitializer::Empty => add_leaf!("Empty"),
                    ForInitializer::Declaration(decl) => {
                        display_declaration(&decl, &statement.initializer.span)
                    }
                    ForInitializer::Expression(expression) => {
                        display_expr(&expression, &statement.initializer.span);
                    }
                }
            }
            {
                add_branch!("ForCondition");
                if let Some(condition) = &statement.condition {
                    display_expr(&condition.node, &condition.span);
                } else {
                    add_leaf!("Empty");
                }
            }
            {
                add_branch!("ForStepExpression");
                if let Some(step) = &statement.step {
                    display_expr(&step.node, &step.span);
                } else {
                    add_leaf!("Empty");
                }
            }
            {
                add_branch!("ForBlock");
                display_statement(&statement.statement.node, &statement.statement.span);
            }
        }
        Statement::BreakStatement => add_leaf!("BreakStatement {}", span),
        Statement::ContinueStatement => add_leaf!("ContinueStatement {}", span),
        Statement::GotoStatement(identifier) => {
            add_branch!("GotoStatement {}", span);
            add_leaf!("Identifier -> \"{}\" {}", identifier.node, identifier.span);
        }
    }
}

/// Display Declaration Specifiers
pub fn display_declspec(specifiers: &Vec<Node<DeclarationSpecifier>>) {
    add_branch!("DeclarationSpecifiers");
    for declspec in specifiers {
        add_leaf!("{}", declspec);
    }
}

pub fn display_funcdeclarator(declarator: &FunctionDeclarator, span: Span) {
    add_branch!("FunctionDeclarator");
    add_leaf!(
        "Identifier -> \"{}\" {}",
        declarator.identifier,
        Span::new(
            span.start,
            span.start + FileLocation::new(declarator.identifier.len(), 0)
        )
    );

    // Add Parameters
    add_branch!("FunctionParameters");
    if declarator.parameters.is_empty() {
        add_leaf!("Empty");
    } else {
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
}

pub fn display_declaration(declaration: &Declaration, span: &Span) {
    add_branch!("Declaration {}", span);
    // Add declaration specifiers
    display_declspec(&declaration.specifiers);

    add_branch!("InitDeclaratorList");
    for init_decl in &declaration.init_declarators {
        // Add declarator
        add_branch!("InitDeclarator");
        match &init_decl.node.declarator.node {
            Declarator::FunctionDeclarator(funcdecl) => {
                display_funcdeclarator(funcdecl, init_decl.node.declarator.span);
            }
            Declarator::DirectDeclarator(identifier) => {
                add_leaf!(
                    "DirectDeclarator -> \"{}\" {}",
                    identifier,
                    init_decl.node.declarator.span
                );
            }
        }
        match &init_decl.node.initializer {
            Some(initializer) => {
                add_branch!("Initializer");
                match &initializer.node {
                    Initializer::AssignmentExpression(expr) => {
                        display_expr(&expr, &initializer.span)
                    }
                }
            }
            None => {}
        }
    }
}

// TODO: Have this be debug mode only
pub fn display_translationunit(tunit: &TranslationUnit) {
    defer_print!();
    add_branch!("TranslationUnit");
    for extdecl in &tunit.external_declarations {
        match &extdecl.node {
            ExternalDeclaration::Declaration(decl) => display_declaration(decl, &extdecl.span),
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
