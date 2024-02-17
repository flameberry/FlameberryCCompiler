# Flameberry C Compiler (Written in Rust)
Flameberry C compiler is primitive C (ISO 17 standard compliant) compiler which is in early stages and the future plan is to make it a fully featured compiler with decent performance.

**Currently it supports:**
1. Preprocessing comments
2. Lexing (almost) every kind of C token
3. Parsing of:
    1. External Declarations
    2. Function Definitions
    3. Statements:
        1. Labeled
        2. Compound
        3. Expression
        4. Selection
        5. Iteration
        6. Jump
    4. Expressions:
        1. Primary except Generic Associations
        2. Postfix
        3. Unary
        4. Cast
        5. Multiplicative
        6. Additive
        7. Shift
        8. Relational
        9. Equality
        10. AND
        11. Exclusive OR
        12. Inclusive OR
        13. Logical AND
        14. Logical OR
        15. Conditional
        16. Assignment
        17. Generic Expression (which contains combinations of all the above expressions)
        18. Constant Expression
4. Generation of basic Abstract Syntax Tree for the C Translation Unit

## Demo
For the following Sandbox/test.c program, the generated AST is:
```C
static int g_GlobalVariable;
static int g_NiceVar;
static _Bool _g_AssumeABoolean;

int nice = 69, wow(), good = 3;

int main(void) {
  // This is a comment to try and mess with the tokenizer :D
  if (_g_AssumeABoolean + 3 < sizeof(const float)) {
    "wow" + "I am a string literal";
    char ch = 'A';
    return g_NiceVar ? sizeof ch > 1 ? ch : 0 : 11;
  }
  else if (g_NiceVar <= 44)
    return 69l;
  else if (g_GlobalVariable > 3)
    // This is a comment to try and mess with the tokenizer :D
    return 12ul;
  else
    // This is a comment to try and mess with the tokenizer :D
    return 0;

  int i = 0;
  sizeof((const int)sizeof(2));
  sizeof(((5 * 3)));
  sizeof((((i))))++++;
  while (i < 10) {
    while (1)
      return 2;
    do {
      char character = 'Z' + 'D';
    } while (i == sizeof(5 - 3));

    int j = i + 1;
    int result = (float)wow(&i, j)(&j, i);
    AdityaG();

    *i = 2 + 2;
    i += 23 % 4;
  }

  for (int i = 0; i < 12; i - 1) {
    for (;;)
      continue;

    int counter = 45;
    return 20 * counter;
  }

  2 * 2;
  ;
  goto Checkpoint;

Checkpoint:
  return 34;

  switch (i * i + i) {
    case 2 * 32:
      return --g_GlobalVariable * g_NiceVar;
    default:
      break;
  }
  // This is a comment to try and mess with the tokenizer :D
  {
    return _g_AssumeABoolean ? 4 & 12 ^ 13 | 1 && 5 ||
      (g_GlobalVariable++ + 2u << 2 & 31) *
      (((34l + (double)_g_AssumeABoolean ==
        ++g_NiceVar >> 3) *
        34) *
        4) *
      23.6f -
      45 >=
      25 !=
      0
      : 77;
  }
}

inline float AdityaG() {}
const int NiceFunction(const int param1, float param2);
unsigned int GetSum(unsigned int, unsigned int);
// This is a comment to try and mess with the tokenizer :D
```

```
TranslationUnit
├╼ Declaration <0..28>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <0..6>
│ │ └╼ TypeSpecifier -> Int <7..10>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "g_GlobalVariable" <11..27>
├╼ Declaration <29..50>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <29..35>
│ │ └╼ TypeSpecifier -> Int <36..39>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "g_NiceVar" <40..49>
├╼ Declaration <51..82>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <51..57>
│ │ └╼ TypeSpecifier -> Bool <58..63>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "_g_AssumeABoolean" <64..81>
├╼ Declaration <84..115>
│ ├╼ DeclarationSpecifiers
│ │ └╼ TypeSpecifier -> Int <84..87>
│ └╼ InitDeclaratorList
│   ├╼ InitDeclarator
│   │ ├╼ DirectDeclarator -> "nice" <88..92>
│   │ └╼ Initializer
│   │   └╼ Constant
│   │     └╼ Integer -> Generic(69), <95..97>
│   ├╼ InitDeclarator
│   │ └╼ FunctionDeclarator
│   │   ├╼ Identifier -> "wow" <99..102>
│   │   └╼ FunctionParameters
│   │     └╼ Empty
│   └╼ InitDeclarator
│     ├╼ DirectDeclarator -> "good" <106..110>
│     └╼ Initializer
│       └╼ Constant
│         └╼ Integer -> Generic(3), <113..114>
├╼ FunctionDefinition <117..1551>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ └╼ TypeSpecifier -> Int <117..120>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "main" <121..125>
│ │   └╼ FunctionParameters
│ │     └╼ FunctionParameter <126..131>
│ │       ├╼ DeclarationSpecifiers
│ │       │ └╼ TypeSpecifier -> Void <126..130>
│ │       └╼ DirectDeclarator -> None
│ └╼ FunctionBody
│   └╼ CompoundStatement <133..1545>
│     ├╼ IfStatement <197..600>
│     │ ├╼ IfExpression
│     │ │ └╼ BinaryOperatorExpression <201..244>
│     │ │   ├╼ Operator -> Less <223..224>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <201..222>
│     │ │   │   ├╼ Operator -> Plus <219..220>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "_g_AssumeABoolean" <201..218>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Constant
│     │ │   │       └╼ Integer -> Generic(3), <221..222>
│     │ │   └╼ RHS
│     │ │     └╼ SizeofTypeExpression <225..244>
│     │ │       └╼ TypeName
│     │ │         └╼ SpecifierQualifiers
│     │ │           ├╼ Const <232..237>
│     │ │           └╼ Float <238..243>
│     │ ├╼ ThenStatement
│     │ │ └╼ CompoundStatement <247..355>
│     │ │   ├╼ ExpressionStatement <252..284>
│     │ │   │ └╼ Expression
│     │ │   │   └╼ BinaryOperatorExpression <252..283>
│     │ │   │     ├╼ Operator -> Plus <258..259>
│     │ │   │     ├╼ LHS
│     │ │   │     │ └╼ StringLiteral -> "wow" <252..257>
│     │ │   │     └╼ RHS
│     │ │   │       └╼ StringLiteral -> "I am a string literal" <260..283>
│     │ │   ├╼ Declaration <289..303>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <289..293>
│     │ │   │ └╼ InitDeclaratorList
│     │ │   │   └╼ InitDeclarator
│     │ │   │     ├╼ DirectDeclarator -> "ch" <294..296>
│     │ │   │     └╼ Initializer
│     │ │   │       └╼ Constant
│     │ │   │         └╼ Character -> 'A'
│     │ │   └╼ ReturnStatement <308..355>
│     │ │     └╼ TernaryOperatorExpression <308..355>
│     │ │       ├╼ Condition
│     │ │       │ └╼ Identifier -> "g_NiceVar" <315..324>
│     │ │       ├╼ IfExpression
│     │ │       │ └╼ TernaryOperatorExpression <327..349>
│     │ │       │   ├╼ Condition
│     │ │       │   │ └╼ BinaryOperatorExpression <327..340>
│     │ │       │   │   ├╼ Operator -> Greater <337..338>
│     │ │       │   │   ├╼ LHS
│     │ │       │   │   │ └╼ SizeofValExpression <327..336>
│     │ │       │   │   │   └╼ Identifier -> "ch" <334..336>
│     │ │       │   │   └╼ RHS
│     │ │       │   │     └╼ Constant
│     │ │       │   │       └╼ Integer -> Generic(1), <339..340>
│     │ │       │   ├╼ IfExpression
│     │ │       │   │ └╼ Identifier -> "ch" <343..345>
│     │ │       │   └╼ ElseExpression
│     │ │       │     └╼ Constant
│     │ │       │       └╼ Integer -> Generic(0), <348..349>
│     │ │       └╼ ElseExpression
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(11), <352..354>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <367..600>
│     │     ├╼ IfExpression
│     │     │ └╼ BinaryOperatorExpression <371..386>
│     │     │   ├╼ Operator -> LessOrEqual <381..383>
│     │     │   ├╼ LHS
│     │     │   │ └╼ Identifier -> "g_NiceVar" <371..380>
│     │     │   └╼ RHS
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(44), <384..386>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <392..403>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <392..403>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <411..600>
│     │         ├╼ IfExpression
│     │         │ └╼ BinaryOperatorExpression <415..435>
│     │         │   ├╼ Operator -> Greater <432..433>
│     │         │   ├╼ LHS
│     │         │   │ └╼ Identifier -> "g_GlobalVariable" <415..431>
│     │         │   └╼ RHS
│     │         │     └╼ Constant
│     │         │       └╼ Integer -> Generic(3), <434..435>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <504..516>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <504..516>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <591..600>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <591..600>
│     ├╼ Declaration <604..614>
│     │ ├╼ DeclarationSpecifiers
│     │ │ └╼ TypeSpecifier -> Int <604..607>
│     │ └╼ InitDeclaratorList
│     │   └╼ InitDeclarator
│     │     ├╼ DirectDeclarator -> "i" <608..609>
│     │     └╼ Initializer
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(0), <612..613>
│     ├╼ ExpressionStatement <617..646>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <617..643>
│     │     └╼ CastExpression <624..643>
│     │       ├╼ Typename
│     │       │ └╼ SpecifierQualifiers
│     │       │   ├╼ Const <625..630>
│     │       │   └╼ Int <631..634>
│     │       └╼ Expression
│     │         └╼ SizeofValExpression <635..643>
│     │           └╼ Constant
│     │             └╼ Integer -> Generic(2), <642..643>
│     ├╼ ExpressionStatement <649..667>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <649..663>
│     │     └╼ BinaryOperatorExpression <658..663>
│     │       ├╼ Operator -> Multiply <660..661>
│     │       ├╼ LHS
│     │       │ └╼ Constant
│     │       │   └╼ Integer -> Generic(5), <658..659>
│     │       └╼ RHS
│     │         └╼ Constant
│     │           └╼ Integer -> Generic(3), <662..663>
│     ├╼ ExpressionStatement <670..690>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <670..689>
│     │     └╼ UnaryOperatorExpression <676..689>
│     │       ├╼ Operator -> PostIncrement <687..689>
│     │       └╼ Expression
│     │         └╼ UnaryOperatorExpression <676..687>
│     │           ├╼ Operator -> PostIncrement <685..687>
│     │           └╼ Expression
│     │             └╼ Identifier -> "i" <680..681>
│     ├╼ WhileStatement <693..928>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <700..706>
│     │ │   ├╼ Operator -> Less <702..703>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <700..701>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <704..706>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <709..928>
│     │     ├╼ WhileStatement <714..739>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <721..722>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <730..739>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <730..739>
│     │     ├╼ DoWhileStatement <744..816>
│     │     │ ├╼ DoBlock
│     │     │ │ └╼ CompoundStatement <748..782>
│     │     │ │   └╼ Declaration <755..782>
│     │     │ │     ├╼ DeclarationSpecifiers
│     │     │ │     │ └╼ TypeSpecifier -> Char <755..759>
│     │     │ │     └╼ InitDeclaratorList
│     │     │ │       └╼ InitDeclarator
│     │     │ │         ├╼ DirectDeclarator -> "character" <760..769>
│     │     │ │         └╼ Initializer
│     │     │ │           └╼ BinaryOperatorExpression <772..781>
│     │     │ │             ├╼ Operator -> Plus <776..777>
│     │     │ │             ├╼ LHS
│     │     │ │             │ └╼ Constant
│     │     │ │             │   └╼ Character -> 'Z'
│     │     │ │             └╼ RHS
│     │     │ │               └╼ Constant
│     │     │ │                 └╼ Character -> 'D'
│     │     │ └╼ DoWhileExpression
│     │     │   └╼ BinaryOperatorExpression <796..813>
│     │     │     ├╼ Operator -> Equals <798..800>
│     │     │     ├╼ LHS
│     │     │     │ └╼ Identifier -> "i" <796..797>
│     │     │     └╼ RHS
│     │     │       └╼ SizeofValExpression <801..813>
│     │     │         └╼ BinaryOperatorExpression <808..813>
│     │     │           ├╼ Operator -> Minus <810..811>
│     │     │           ├╼ LHS
│     │     │           │ └╼ Constant
│     │     │           │   └╼ Integer -> Generic(5), <808..809>
│     │     │           └╼ RHS
│     │     │             └╼ Constant
│     │     │               └╼ Integer -> Generic(3), <812..813>
│     │     ├╼ Declaration <822..836>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <822..825>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "j" <826..827>
│     │     │     └╼ Initializer
│     │     │       └╼ BinaryOperatorExpression <830..835>
│     │     │         ├╼ Operator -> Plus <832..833>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Identifier -> "i" <830..831>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(1), <834..835>
│     │     ├╼ Declaration <841..879>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <841..844>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "result" <845..851>
│     │     │     └╼ Initializer
│     │     │       └╼ CastExpression <854..878>
│     │     │         ├╼ Typename
│     │     │         │ └╼ SpecifierQualifiers
│     │     │         │   └╼ Float <855..860>
│     │     │         └╼ Expression
│     │     │           └╼ CallExpression <861..878>
│     │     │             ├╼ CalleeExpression
│     │     │             │ └╼ CallExpression <861..871>
│     │     │             │   ├╼ CalleeExpression
│     │     │             │   │ └╼ Identifier -> "wow" <861..864>
│     │     │             │   └╼ ArgumentExpressionList
│     │     │             │     ├╼ ArgumentExpression
│     │     │             │     │ └╼ UnaryOperatorExpression <865..867>
│     │     │             │     │   ├╼ Operator -> Address <865..866>
│     │     │             │     │   └╼ Expression
│     │     │             │     │     └╼ Identifier -> "i" <866..867>
│     │     │             │     └╼ ArgumentExpression
│     │     │             │       └╼ Identifier -> "j" <869..870>
│     │     │             └╼ ArgumentExpressionList
│     │     │               ├╼ ArgumentExpression
│     │     │               │ └╼ UnaryOperatorExpression <872..874>
│     │     │               │   ├╼ Operator -> Address <872..873>
│     │     │               │   └╼ Expression
│     │     │               │     └╼ Identifier -> "j" <873..874>
│     │     │               └╼ ArgumentExpression
│     │     │                 └╼ Identifier -> "i" <876..877>
│     │     ├╼ ExpressionStatement <884..894>
│     │     │ └╼ Expression
│     │     │   └╼ CallExpression <884..893>
│     │     │     ├╼ CalleeExpression
│     │     │     │ └╼ Identifier -> "AdityaG" <884..891>
│     │     │     └╼ ArgumentExpressionList
│     │     │       └╼ Empty
│     │     ├╼ ExpressionStatement <900..911>
│     │     │ └╼ Expression
│     │     │   └╼ BinaryOperatorExpression <900..910>
│     │     │     ├╼ Operator -> Assign <903..904>
│     │     │     ├╼ LHS
│     │     │     │ └╼ UnaryOperatorExpression <900..902>
│     │     │     │   ├╼ Operator -> Indirection <900..901>
│     │     │     │   └╼ Expression
│     │     │     │     └╼ Identifier -> "i" <901..902>
│     │     │     └╼ RHS
│     │     │       └╼ BinaryOperatorExpression <905..910>
│     │     │         ├╼ Operator -> Plus <907..908>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Constant
│     │     │         │   └╼ Integer -> Generic(2), <905..906>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(2), <909..910>
│     │     └╼ ExpressionStatement <916..928>
│     │       └╼ Expression
│     │         └╼ BinaryOperatorExpression <916..927>
│     │           ├╼ Operator -> AssignPlus <918..920>
│     │           ├╼ LHS
│     │           │ └╼ Identifier -> "i" <916..917>
│     │           └╼ RHS
│     │             └╼ BinaryOperatorExpression <921..927>
│     │               ├╼ Operator -> Modulo <924..925>
│     │               ├╼ LHS
│     │               │ └╼ Constant
│     │               │   └╼ Integer -> Generic(23), <921..923>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(4), <926..927>
│     ├╼ ForStatement <936..1045>
│     │ ├╼ ForInitializer
│     │ │ └╼ Declaration <941..951>
│     │ │   ├╼ DeclarationSpecifiers
│     │ │   │ └╼ TypeSpecifier -> Int <941..944>
│     │ │   └╼ InitDeclaratorList
│     │ │     └╼ InitDeclarator
│     │ │       ├╼ DirectDeclarator -> "i" <945..946>
│     │ │       └╼ Initializer
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(0), <949..950>
│     │ ├╼ ForCondition
│     │ │ └╼ BinaryOperatorExpression <952..958>
│     │ │   ├╼ Operator -> Less <954..955>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <952..953>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(12), <956..958>
│     │ ├╼ ForStepExpression
│     │ │ └╼ BinaryOperatorExpression <960..965>
│     │ │   ├╼ Operator -> Minus <962..963>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <960..961>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(1), <964..965>
│     │ └╼ ForBlock
│     │   └╼ CompoundStatement <968..1045>
│     │     ├╼ ForStatement <973..997>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ Empty
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <988..997>
│     │     ├╼ Declaration <1003..1020>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <1003..1006>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <1007..1014>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <1017..1019>
│     │     └╼ ReturnStatement <1025..1045>
│     │       └╼ BinaryOperatorExpression <1025..1045>
│     │         ├╼ Operator -> Multiply <1035..1036>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <1032..1034>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <1037..1044>
│     ├╼ ExpressionStatement <1053..1059>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <1053..1058>
│     │     ├╼ Operator -> Multiply <1055..1056>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <1053..1054>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <1057..1058>
│     ├╼ ExpressionStatement <1062..1063>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <1066..1082>
│     │ └╼ Identifier -> "Checkpoint" <1071..1081>
│     ├╼ LabeledStatement <1084..1108>
│     │ ├╼ Identifier -> "Checkpoint" <1084..1094>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <1098..1108>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <1098..1108>
│     ├╼ SwitchStatement <1112..1220>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <1120..1129>
│     │ │   ├╼ Operator -> Plus <1126..1127>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <1120..1125>
│     │ │   │   ├╼ Operator -> Multiply <1122..1123>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <1120..1121>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <1124..1125>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <1128..1129>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <1132..1220>
│     │     ├╼ CaseStatement <1137..1194>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <1142..1148>
│     │     │ │   ├╼ Operator -> Multiply <1144..1145>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <1142..1143>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <1146..1148>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <1156..1194>
│     │     │     └╼ BinaryOperatorExpression <1156..1194>
│     │     │       ├╼ Operator -> Multiply <1182..1183>
│     │     │       ├╼ LHS
│     │     │       │ └╼ UnaryOperatorExpression <1163..1181>
│     │     │       │   ├╼ Operator -> PreDecrement <1163..1165>
│     │     │       │   └╼ Expression
│     │     │       │     └╼ Identifier -> "g_GlobalVariable" <1165..1181>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <1184..1193>
│     │     └╼ DefaultStatement <1199..1220>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <1214..1220>
│     └╼ CompoundStatement <1289..1545>
│       └╼ ReturnStatement <1294..1545>
│         └╼ TernaryOperatorExpression <1294..1545>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1301..1318>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1321..1533>
│           │   ├╼ Operator -> LogicalOr <1342..1344>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1321..1341>
│           │   │   ├╼ Operator -> LogicalAnd <1337..1339>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1321..1336>
│           │   │   │   ├╼ Operator -> BitwiseOr <1333..1334>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1321..1332>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1328..1329>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1321..1327>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1323..1324>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1321..1322>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1325..1327>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1330..1332>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1335..1336>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1340..1341>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1352..1533>
│           │       ├╼ Operator -> NotEquals <1523..1525>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1352..1522>
│           │       │   ├╼ Operator -> GreaterOrEqual <1511..1513>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1352..1510>
│           │       │   │   ├╼ Operator -> Minus <1500..1501>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1352..1499>
│           │       │   │   │   ├╼ Operator -> Multiply <1486..1487>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1352..1484>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1387..1388>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1352..1385>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1381..1382>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1352..1380>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1376..1378>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1352..1375>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1371..1372>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ UnaryOperatorExpression <1352..1370>
│           │       │   │   │   │   │   │   │   │   ├╼ Operator -> PostIncrement <1368..1370>
│           │       │   │   │   │   │   │   │   │   └╼ Expression
│           │       │   │   │   │   │   │   │   │     └╼ Identifier -> "g_GlobalVariable" <1352..1368>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1373..1375>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1379..1380>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1383..1385>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1398..1484>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1473..1474>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1398..1471>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1459..1460>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1398..1457>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1430..1432>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1398..1429>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1402..1403>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1398..1401>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ CastExpression <1404..1429>
│           │       │   │   │   │       │   │   │       ├╼ Typename
│           │       │   │   │   │       │   │   │       │ └╼ SpecifierQualifiers
│           │       │   │   │   │       │   │   │       │   └╼ Double <1405..1411>
│           │       │   │   │   │       │   │   │       └╼ Expression
│           │       │   │   │   │       │   │   │         └╼ Identifier -> "_g_AssumeABoolean" <1412..1429>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1441..1457>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1453..1455>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ UnaryOperatorExpression <1441..1452>
│           │       │   │   │   │       │   │       │   ├╼ Operator -> PreIncrement <1441..1443>
│           │       │   │   │   │       │   │       │   └╼ Expression
│           │       │   │   │   │       │   │       │     └╼ Identifier -> "g_NiceVar" <1443..1452>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1456..1457>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1469..1471>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1483..1484>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1494..1499>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1508..1510>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1520..1522>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1532..1533>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1542..1544>
├╼ FunctionDefinition <1553..1578>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1553..1559>
│ │ │ └╼ TypeSpecifier -> Float <1560..1565>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1566..1573>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1579..1634>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1579..1584>
│ │ └╼ TypeSpecifier -> Int <1585..1588>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1589..1601>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1602..1618>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1602..1607>
│         │ │ └╼ TypeSpecifier -> Int <1608..1611>
│         │ └╼ DirectDeclarator -> "param1" <1612..1618>
│         └╼ FunctionParameter <1620..1632>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1620..1625>
│           └╼ DirectDeclarator -> "param2" <1626..1632>
└╼ Declaration <1635..1683>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1635..1643>
  │ └╼ TypeSpecifier -> Int <1644..1647>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1648..1654>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1655..1668>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1655..1663>
          │ │ └╼ TypeSpecifier -> Int <1664..1667>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1669..1682>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1669..1677>
            │ └╼ TypeSpecifier -> Int <1678..1681>
            └╼ DirectDeclarator -> None
```

## Getting Started

To build the rust project:

```sh
cargo build
```

Before running the main.rs, don't forget to set the path where your C programs are located (_This will be abstracted in the future and a script will be provided to do the same instead of needing to modify the rust program itself_):

```rust
fn main() {
    let testpath = "<path/to/parent/directory/containing/c/programs>";
    run_tests(testpath);
}
```

Or if you want to test using a single file:

```rust
fn main() {
    let test_path = "Sandbox/test.c";
    compile_file(test_path);
}
```

To run the project:

```sh
cargo run
```

_Note: The repository currently uses test programs present in the repository https://github.com/nlsandler/writing-a-c-compiler-tests to check the output._
