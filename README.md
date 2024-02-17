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
    AdityaG(i, (i = 5, i * 3 - 2));

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
│   └╼ InitDeclarator
│     ├╼ DirectDeclarator -> "nice" <88..92>
│     └╼ Initializer
│       └╼ CommaExpression <95..114>
│         ├╼ Constant
│         │ └╼ Integer -> Generic(69), <95..97>
│         ├╼ CallExpression <99..104>
│         │ ├╼ CalleeExpression
│         │ │ └╼ Identifier -> "wow" <99..102>
│         │ └╼ ArgumentExpressionList
│         │   └╼ Empty
│         └╼ BinaryOperatorExpression <106..114>
│           ├╼ Operator -> Assign <111..112>
│           ├╼ LHS
│           │ └╼ Identifier -> "good" <106..110>
│           └╼ RHS
│             └╼ Constant
│               └╼ Integer -> Generic(3), <113..114>
├╼ FunctionDefinition <117..1572>
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
│   └╼ CompoundStatement <133..1566>
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
│     ├╼ WhileStatement <693..949>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <700..706>
│     │ │   ├╼ Operator -> Less <702..703>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <700..701>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <704..706>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <709..949>
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
│     │     ├╼ ExpressionStatement <884..915>
│     │     │ └╼ Expression
│     │     │   └╼ CallExpression <884..914>
│     │     │     ├╼ CalleeExpression
│     │     │     │ └╼ Identifier -> "AdityaG" <884..891>
│     │     │     └╼ ArgumentExpressionList
│     │     │       ├╼ ArgumentExpression
│     │     │       │ └╼ Identifier -> "i" <892..893>
│     │     │       └╼ ArgumentExpression
│     │     │         └╼ CommaExpression <896..912>
│     │     │           ├╼ BinaryOperatorExpression <896..901>
│     │     │           │ ├╼ Operator -> Assign <898..899>
│     │     │           │ ├╼ LHS
│     │     │           │ │ └╼ Identifier -> "i" <896..897>
│     │     │           │ └╼ RHS
│     │     │           │   └╼ Constant
│     │     │           │     └╼ Integer -> Generic(5), <900..901>
│     │     │           └╼ BinaryOperatorExpression <903..912>
│     │     │             ├╼ Operator -> Minus <909..910>
│     │     │             ├╼ LHS
│     │     │             │ └╼ BinaryOperatorExpression <903..908>
│     │     │             │   ├╼ Operator -> Multiply <905..906>
│     │     │             │   ├╼ LHS
│     │     │             │   │ └╼ Identifier -> "i" <903..904>
│     │     │             │   └╼ RHS
│     │     │             │     └╼ Constant
│     │     │             │       └╼ Integer -> Generic(3), <907..908>
│     │     │             └╼ RHS
│     │     │               └╼ Constant
│     │     │                 └╼ Integer -> Generic(2), <911..912>
│     │     ├╼ ExpressionStatement <921..932>
│     │     │ └╼ Expression
│     │     │   └╼ BinaryOperatorExpression <921..931>
│     │     │     ├╼ Operator -> Assign <924..925>
│     │     │     ├╼ LHS
│     │     │     │ └╼ UnaryOperatorExpression <921..923>
│     │     │     │   ├╼ Operator -> Indirection <921..922>
│     │     │     │   └╼ Expression
│     │     │     │     └╼ Identifier -> "i" <922..923>
│     │     │     └╼ RHS
│     │     │       └╼ BinaryOperatorExpression <926..931>
│     │     │         ├╼ Operator -> Plus <928..929>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Constant
│     │     │         │   └╼ Integer -> Generic(2), <926..927>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(2), <930..931>
│     │     └╼ ExpressionStatement <937..949>
│     │       └╼ Expression
│     │         └╼ BinaryOperatorExpression <937..948>
│     │           ├╼ Operator -> AssignPlus <939..941>
│     │           ├╼ LHS
│     │           │ └╼ Identifier -> "i" <937..938>
│     │           └╼ RHS
│     │             └╼ BinaryOperatorExpression <942..948>
│     │               ├╼ Operator -> Modulo <945..946>
│     │               ├╼ LHS
│     │               │ └╼ Constant
│     │               │   └╼ Integer -> Generic(23), <942..944>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(4), <947..948>
│     ├╼ ForStatement <957..1066>
│     │ ├╼ ForInitializer
│     │ │ └╼ Declaration <962..972>
│     │ │   ├╼ DeclarationSpecifiers
│     │ │   │ └╼ TypeSpecifier -> Int <962..965>
│     │ │   └╼ InitDeclaratorList
│     │ │     └╼ InitDeclarator
│     │ │       ├╼ DirectDeclarator -> "i" <966..967>
│     │ │       └╼ Initializer
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(0), <970..971>
│     │ ├╼ ForCondition
│     │ │ └╼ BinaryOperatorExpression <973..979>
│     │ │   ├╼ Operator -> Less <975..976>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <973..974>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(12), <977..979>
│     │ ├╼ ForStepExpression
│     │ │ └╼ BinaryOperatorExpression <981..986>
│     │ │   ├╼ Operator -> Minus <983..984>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <981..982>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(1), <985..986>
│     │ └╼ ForBlock
│     │   └╼ CompoundStatement <989..1066>
│     │     ├╼ ForStatement <994..1018>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ Empty
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <1009..1018>
│     │     ├╼ Declaration <1024..1041>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <1024..1027>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <1028..1035>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <1038..1040>
│     │     └╼ ReturnStatement <1046..1066>
│     │       └╼ BinaryOperatorExpression <1046..1066>
│     │         ├╼ Operator -> Multiply <1056..1057>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <1053..1055>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <1058..1065>
│     ├╼ ExpressionStatement <1074..1080>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <1074..1079>
│     │     ├╼ Operator -> Multiply <1076..1077>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <1074..1075>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <1078..1079>
│     ├╼ ExpressionStatement <1083..1084>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <1087..1103>
│     │ └╼ Identifier -> "Checkpoint" <1092..1102>
│     ├╼ LabeledStatement <1105..1129>
│     │ ├╼ Identifier -> "Checkpoint" <1105..1115>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <1119..1129>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <1119..1129>
│     ├╼ SwitchStatement <1133..1241>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <1141..1150>
│     │ │   ├╼ Operator -> Plus <1147..1148>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <1141..1146>
│     │ │   │   ├╼ Operator -> Multiply <1143..1144>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <1141..1142>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <1145..1146>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <1149..1150>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <1153..1241>
│     │     ├╼ CaseStatement <1158..1215>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <1163..1169>
│     │     │ │   ├╼ Operator -> Multiply <1165..1166>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <1163..1164>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <1167..1169>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <1177..1215>
│     │     │     └╼ BinaryOperatorExpression <1177..1215>
│     │     │       ├╼ Operator -> Multiply <1203..1204>
│     │     │       ├╼ LHS
│     │     │       │ └╼ UnaryOperatorExpression <1184..1202>
│     │     │       │   ├╼ Operator -> PreDecrement <1184..1186>
│     │     │       │   └╼ Expression
│     │     │       │     └╼ Identifier -> "g_GlobalVariable" <1186..1202>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <1205..1214>
│     │     └╼ DefaultStatement <1220..1241>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <1235..1241>
│     └╼ CompoundStatement <1310..1566>
│       └╼ ReturnStatement <1315..1566>
│         └╼ TernaryOperatorExpression <1315..1566>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1322..1339>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1342..1554>
│           │   ├╼ Operator -> LogicalOr <1363..1365>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1342..1362>
│           │   │   ├╼ Operator -> LogicalAnd <1358..1360>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1342..1357>
│           │   │   │   ├╼ Operator -> BitwiseOr <1354..1355>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1342..1353>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1349..1350>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1342..1348>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1344..1345>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1342..1343>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1346..1348>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1351..1353>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1356..1357>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1361..1362>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1373..1554>
│           │       ├╼ Operator -> NotEquals <1544..1546>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1373..1543>
│           │       │   ├╼ Operator -> GreaterOrEqual <1532..1534>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1373..1531>
│           │       │   │   ├╼ Operator -> Minus <1521..1522>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1373..1520>
│           │       │   │   │   ├╼ Operator -> Multiply <1507..1508>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1373..1505>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1408..1409>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1373..1406>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1402..1403>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1373..1401>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1397..1399>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1373..1396>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1392..1393>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ UnaryOperatorExpression <1373..1391>
│           │       │   │   │   │   │   │   │   │   ├╼ Operator -> PostIncrement <1389..1391>
│           │       │   │   │   │   │   │   │   │   └╼ Expression
│           │       │   │   │   │   │   │   │   │     └╼ Identifier -> "g_GlobalVariable" <1373..1389>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1394..1396>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1400..1401>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1404..1406>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1419..1505>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1494..1495>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1419..1492>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1480..1481>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1419..1478>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1451..1453>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1419..1450>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1423..1424>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1419..1422>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ CastExpression <1425..1450>
│           │       │   │   │   │       │   │   │       ├╼ Typename
│           │       │   │   │   │       │   │   │       │ └╼ SpecifierQualifiers
│           │       │   │   │   │       │   │   │       │   └╼ Double <1426..1432>
│           │       │   │   │   │       │   │   │       └╼ Expression
│           │       │   │   │   │       │   │   │         └╼ Identifier -> "_g_AssumeABoolean" <1433..1450>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1462..1478>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1474..1476>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ UnaryOperatorExpression <1462..1473>
│           │       │   │   │   │       │   │       │   ├╼ Operator -> PreIncrement <1462..1464>
│           │       │   │   │   │       │   │       │   └╼ Expression
│           │       │   │   │   │       │   │       │     └╼ Identifier -> "g_NiceVar" <1464..1473>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1477..1478>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1490..1492>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1504..1505>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1515..1520>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1529..1531>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1541..1543>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1553..1554>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1563..1565>
├╼ FunctionDefinition <1574..1599>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1574..1580>
│ │ │ └╼ TypeSpecifier -> Float <1581..1586>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1587..1594>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1600..1655>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1600..1605>
│ │ └╼ TypeSpecifier -> Int <1606..1609>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1610..1622>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1623..1639>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1623..1628>
│         │ │ └╼ TypeSpecifier -> Int <1629..1632>
│         │ └╼ DirectDeclarator -> "param1" <1633..1639>
│         └╼ FunctionParameter <1641..1653>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1641..1646>
│           └╼ DirectDeclarator -> "param2" <1647..1653>
└╼ Declaration <1656..1704>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1656..1664>
  │ └╼ TypeSpecifier -> Int <1665..1668>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1669..1675>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1676..1689>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1676..1684>
          │ │ └╼ TypeSpecifier -> Int <1685..1688>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1690..1703>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1690..1698>
            │ └╼ TypeSpecifier -> Int <1699..1702>
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
