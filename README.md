# Flameberry C Compiler (Written in Rust)

Flameberry C compiler is primitive C (somewhat ISO 17 standard compliant) compiler which is in early stages and the future plan is to make it a fully featured compiler with decent performance.

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
5. Semantic Analysis:
    1. Type Checking
    2. Implicit Casting
    3. Symbol Table
    4. Variable Shadowing

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
├╼ Declaration <1:1..1:29>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <1:1..1:7>
│ │ └╼ TypeSpecifier -> Int <1:8..1:11>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "g_GlobalVariable" <1:12..1:28>
├╼ Declaration <2:1..2:22>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <2:1..2:7>
│ │ └╼ TypeSpecifier -> Int <2:8..2:11>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "g_NiceVar" <2:12..2:21>
├╼ Declaration <3:1..3:32>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <3:1..3:7>
│ │ └╼ TypeSpecifier -> Bool <3:8..3:13>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ DirectDeclarator -> "_g_AssumeABoolean" <3:14..3:31>
├╼ Declaration <5:1..5:32>
│ ├╼ DeclarationSpecifiers
│ │ └╼ TypeSpecifier -> Int <5:1..5:4>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     ├╼ DirectDeclarator -> "nice" <5:5..5:9>
│     └╼ Initializer
│       └╼ CommaExpression <5:12..5:31>
│         ├╼ Constant
│         │ └╼ Integer -> Generic(69), <5:12..5:14>
│         ├╼ CallExpression <5:16..5:21>
│         │ ├╼ CalleeExpression
│         │ │ └╼ Identifier -> "wow" <5:16..5:19>
│         │ └╼ ArgumentExpressionList
│         │   └╼ Empty
│         └╼ BinaryOperatorExpression <5:23..5:31>
│           ├╼ Operator -> Assign <5:28..5:29>
│           ├╼ LHS
│           │ └╼ Identifier -> "good" <5:23..5:27>
│           └╼ RHS
│             └╼ Constant
│               └╼ Integer -> Generic(3), <5:30..5:31>
├╼ FunctionDefinition <7:1..77:2>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ └╼ TypeSpecifier -> Int <7:1..7:4>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "main" <7:5..11:5>
│ │   └╼ FunctionParameters
│ │     └╼ FunctionParameter <7:10..7:15>
│ │       ├╼ DeclarationSpecifiers
│ │       │ └╼ TypeSpecifier -> Void <7:10..7:14>
│ │       └╼ DirectDeclarator -> None
│ └╼ FunctionBody
│   └╼ CompoundStatement <7:17..75:12>
│     ├╼ IfStatement <9:3..21:14>
│     │ ├╼ IfExpression
│     │ │ └╼ BinaryOperatorExpression <9:7..9:50>
│     │ │   ├╼ Operator -> Less <9:29..9:30>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <9:7..9:28>
│     │ │   │   ├╼ Operator -> Plus <9:25..9:26>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "_g_AssumeABoolean" <9:7..9:24>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Constant
│     │ │   │       └╼ Integer -> Generic(3), <9:27..9:28>
│     │ │   └╼ RHS
│     │ │     └╼ SizeofTypeExpression <9:31..9:50>
│     │ │       └╼ TypeName
│     │ │         └╼ SpecifierQualifiers
│     │ │           ├╼ Const <9:38..9:43>
│     │ │           └╼ Float <9:44..9:49>
│     │ ├╼ ThenStatement
│     │ │ └╼ CompoundStatement <9:53..12:52>
│     │ │   ├╼ ExpressionStatement <10:5..10:37>
│     │ │   │ └╼ Expression
│     │ │   │   └╼ BinaryOperatorExpression <10:5..10:36>
│     │ │   │     ├╼ Operator -> Plus <10:11..10:12>
│     │ │   │     ├╼ LHS
│     │ │   │     │ └╼ StringLiteral -> "wow" <10:5..10:10>
│     │ │   │     └╼ RHS
│     │ │   │       └╼ StringLiteral -> "I am a string literal" <10:13..10:36>
│     │ │   ├╼ Declaration <11:5..11:19>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <11:5..11:9>
│     │ │   │ └╼ InitDeclaratorList
│     │ │   │   └╼ InitDeclarator
│     │ │   │     ├╼ DirectDeclarator -> "ch" <11:10..11:12>
│     │ │   │     └╼ Initializer
│     │ │   │       └╼ Constant
│     │ │   │         └╼ Character -> 'A'
│     │ │   └╼ ReturnStatement <12:5..12:52>
│     │ │     └╼ TernaryOperatorExpression <12:5..12:52>
│     │ │       ├╼ Condition
│     │ │       │ └╼ Identifier -> "g_NiceVar" <12:12..12:21>
│     │ │       ├╼ IfExpression
│     │ │       │ └╼ TernaryOperatorExpression <12:24..12:46>
│     │ │       │   ├╼ Condition
│     │ │       │   │ └╼ BinaryOperatorExpression <12:24..12:37>
│     │ │       │   │   ├╼ Operator -> Greater <12:34..12:35>
│     │ │       │   │   ├╼ LHS
│     │ │       │   │   │ └╼ SizeofValExpression <12:24..12:33>
│     │ │       │   │   │   └╼ Identifier -> "ch" <12:31..12:33>
│     │ │       │   │   └╼ RHS
│     │ │       │   │     └╼ Constant
│     │ │       │   │       └╼ Integer -> Generic(1), <12:36..12:37>
│     │ │       │   ├╼ IfExpression
│     │ │       │   │ └╼ Identifier -> "ch" <12:40..12:42>
│     │ │       │   └╼ ElseExpression
│     │ │       │     └╼ Constant
│     │ │       │       └╼ Integer -> Generic(0), <12:45..12:46>
│     │ │       └╼ ElseExpression
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(11), <12:49..12:51>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <14:8..21:14>
│     │     ├╼ IfExpression
│     │     │ └╼ BinaryOperatorExpression <14:12..14:27>
│     │     │   ├╼ Operator -> LessOrEqual <14:22..14:24>
│     │     │   ├╼ LHS
│     │     │   │ └╼ Identifier -> "g_NiceVar" <14:12..14:21>
│     │     │   └╼ RHS
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(44), <14:25..14:27>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <15:5..15:16>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <15:5..15:16>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <16:8..21:14>
│     │         ├╼ IfExpression
│     │         │ └╼ BinaryOperatorExpression <16:12..16:32>
│     │         │   ├╼ Operator -> Greater <16:29..16:30>
│     │         │   ├╼ LHS
│     │         │   │ └╼ Identifier -> "g_GlobalVariable" <16:12..16:28>
│     │         │   └╼ RHS
│     │         │     └╼ Constant
│     │         │       └╼ Integer -> Generic(3), <16:31..16:32>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <18:5..18:17>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <18:5..18:17>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <21:5..21:14>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <21:5..21:14>
│     ├╼ Declaration <23:3..23:13>
│     │ ├╼ DeclarationSpecifiers
│     │ │ └╼ TypeSpecifier -> Int <23:3..23:6>
│     │ └╼ InitDeclaratorList
│     │   └╼ InitDeclarator
│     │     ├╼ DirectDeclarator -> "i" <23:7..23:8>
│     │     └╼ Initializer
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(0), <23:11..23:12>
│     ├╼ ExpressionStatement <24:3..24:32>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <24:3..24:29>
│     │     └╼ CastExpression <24:10..24:29>
│     │       ├╼ Typename
│     │       │ └╼ SpecifierQualifiers
│     │       │   ├╼ Const <24:11..24:16>
│     │       │   └╼ Int <24:17..24:20>
│     │       └╼ Expression
│     │         └╼ SizeofValExpression <24:21..24:29>
│     │           └╼ Constant
│     │             └╼ Integer -> Generic(2), <24:28..24:29>
│     ├╼ ExpressionStatement <25:3..25:21>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <25:3..25:17>
│     │     └╼ BinaryOperatorExpression <25:12..25:17>
│     │       ├╼ Operator -> Multiply <25:14..25:15>
│     │       ├╼ LHS
│     │       │ └╼ Constant
│     │       │   └╼ Integer -> Generic(5), <25:12..25:13>
│     │       └╼ RHS
│     │         └╼ Constant
│     │           └╼ Integer -> Generic(3), <25:16..25:17>
│     ├╼ ExpressionStatement <26:3..26:23>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <26:3..26:22>
│     │     └╼ UnaryOperatorExpression <26:9..26:22>
│     │       ├╼ Operator -> PostIncrement <26:20..26:22>
│     │       └╼ Expression
│     │         └╼ UnaryOperatorExpression <26:9..26:20>
│     │           ├╼ Operator -> PostIncrement <26:18..26:20>
│     │           └╼ Expression
│     │             └╼ Identifier -> "i" <26:13..26:14>
│     ├╼ WhileStatement <27:3..39:17>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <27:10..27:16>
│     │ │   ├╼ Operator -> Less <27:12..27:13>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <27:10..27:11>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <27:14..27:16>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <27:19..39:17>
│     │     ├╼ WhileStatement <28:5..29:16>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <28:12..28:13>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <29:7..29:16>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <29:7..29:16>
│     │     ├╼ DoWhileStatement <30:5..32:34>
│     │     │ ├╼ DoBlock
│     │     │ │ └╼ CompoundStatement <30:9..31:34>
│     │     │ │   └╼ Declaration <31:7..31:34>
│     │     │ │     ├╼ DeclarationSpecifiers
│     │     │ │     │ └╼ TypeSpecifier -> Char <31:7..31:11>
│     │     │ │     └╼ InitDeclaratorList
│     │     │ │       └╼ InitDeclarator
│     │     │ │         ├╼ DirectDeclarator -> "character" <31:12..31:21>
│     │     │ │         └╼ Initializer
│     │     │ │           └╼ BinaryOperatorExpression <31:24..31:33>
│     │     │ │             ├╼ Operator -> Plus <31:28..31:29>
│     │     │ │             ├╼ LHS
│     │     │ │             │ └╼ Constant
│     │     │ │             │   └╼ Character -> 'Z'
│     │     │ │             └╼ RHS
│     │     │ │               └╼ Constant
│     │     │ │                 └╼ Character -> 'D'
│     │     │ └╼ DoWhileExpression
│     │     │   └╼ BinaryOperatorExpression <32:14..32:31>
│     │     │     ├╼ Operator -> Equals <32:16..32:18>
│     │     │     ├╼ LHS
│     │     │     │ └╼ Identifier -> "i" <32:14..32:15>
│     │     │     └╼ RHS
│     │     │       └╼ SizeofValExpression <32:19..32:31>
│     │     │         └╼ BinaryOperatorExpression <32:26..32:31>
│     │     │           ├╼ Operator -> Minus <32:28..32:29>
│     │     │           ├╼ LHS
│     │     │           │ └╼ Constant
│     │     │           │   └╼ Integer -> Generic(5), <32:26..32:27>
│     │     │           └╼ RHS
│     │     │             └╼ Constant
│     │     │               └╼ Integer -> Generic(3), <32:30..32:31>
│     │     ├╼ Declaration <34:5..34:19>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <34:5..34:8>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "j" <34:9..34:10>
│     │     │     └╼ Initializer
│     │     │       └╼ BinaryOperatorExpression <34:13..34:18>
│     │     │         ├╼ Operator -> Plus <34:15..34:16>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Identifier -> "i" <34:13..34:14>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(1), <34:17..34:18>
│     │     ├╼ Declaration <35:5..35:43>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <35:5..35:8>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "result" <35:9..35:15>
│     │     │     └╼ Initializer
│     │     │       └╼ CastExpression <35:18..35:42>
│     │     │         ├╼ Typename
│     │     │         │ └╼ SpecifierQualifiers
│     │     │         │   └╼ Float <35:19..35:24>
│     │     │         └╼ Expression
│     │     │           └╼ CallExpression <35:25..35:42>
│     │     │             ├╼ CalleeExpression
│     │     │             │ └╼ CallExpression <35:25..35:35>
│     │     │             │   ├╼ CalleeExpression
│     │     │             │   │ └╼ Identifier -> "wow" <35:25..35:28>
│     │     │             │   └╼ ArgumentExpressionList
│     │     │             │     ├╼ ArgumentExpression
│     │     │             │     │ └╼ UnaryOperatorExpression <35:29..35:31>
│     │     │             │     │   ├╼ Operator -> Address <35:29..35:30>
│     │     │             │     │   └╼ Expression
│     │     │             │     │     └╼ Identifier -> "i" <35:30..35:31>
│     │     │             │     └╼ ArgumentExpression
│     │     │             │       └╼ Identifier -> "j" <35:33..35:34>
│     │     │             └╼ ArgumentExpressionList
│     │     │               ├╼ ArgumentExpression
│     │     │               │ └╼ UnaryOperatorExpression <35:36..35:38>
│     │     │               │   ├╼ Operator -> Address <35:36..35:37>
│     │     │               │   └╼ Expression
│     │     │               │     └╼ Identifier -> "j" <35:37..35:38>
│     │     │               └╼ ArgumentExpression
│     │     │                 └╼ Identifier -> "i" <35:40..35:41>
│     │     ├╼ ExpressionStatement <36:5..36:36>
│     │     │ └╼ Expression
│     │     │   └╼ CallExpression <36:5..36:35>
│     │     │     ├╼ CalleeExpression
│     │     │     │ └╼ Identifier -> "AdityaG" <36:5..36:12>
│     │     │     └╼ ArgumentExpressionList
│     │     │       ├╼ ArgumentExpression
│     │     │       │ └╼ Identifier -> "i" <36:13..36:14>
│     │     │       └╼ ArgumentExpression
│     │     │         └╼ CommaExpression <36:17..36:33>
│     │     │           ├╼ BinaryOperatorExpression <36:17..36:22>
│     │     │           │ ├╼ Operator -> Assign <36:19..36:20>
│     │     │           │ ├╼ LHS
│     │     │           │ │ └╼ Identifier -> "i" <36:17..36:18>
│     │     │           │ └╼ RHS
│     │     │           │   └╼ Constant
│     │     │           │     └╼ Integer -> Generic(5), <36:21..36:22>
│     │     │           └╼ BinaryOperatorExpression <36:24..36:33>
│     │     │             ├╼ Operator -> Minus <36:30..36:31>
│     │     │             ├╼ LHS
│     │     │             │ └╼ BinaryOperatorExpression <36:24..36:29>
│     │     │             │   ├╼ Operator -> Multiply <36:26..36:27>
│     │     │             │   ├╼ LHS
│     │     │             │   │ └╼ Identifier -> "i" <36:24..36:25>
│     │     │             │   └╼ RHS
│     │     │             │     └╼ Constant
│     │     │             │       └╼ Integer -> Generic(3), <36:28..36:29>
│     │     │             └╼ RHS
│     │     │               └╼ Constant
│     │     │                 └╼ Integer -> Generic(2), <36:32..36:33>
│     │     ├╼ ExpressionStatement <38:5..38:16>
│     │     │ └╼ Expression
│     │     │   └╼ BinaryOperatorExpression <38:5..38:15>
│     │     │     ├╼ Operator -> Assign <38:8..38:9>
│     │     │     ├╼ LHS
│     │     │     │ └╼ UnaryOperatorExpression <38:5..38:7>
│     │     │     │   ├╼ Operator -> Indirection <38:5..38:6>
│     │     │     │   └╼ Expression
│     │     │     │     └╼ Identifier -> "i" <38:6..38:7>
│     │     │     └╼ RHS
│     │     │       └╼ BinaryOperatorExpression <38:10..38:15>
│     │     │         ├╼ Operator -> Plus <38:12..38:13>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Constant
│     │     │         │   └╼ Integer -> Generic(2), <38:10..38:11>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(2), <38:14..38:15>
│     │     └╼ ExpressionStatement <39:5..39:17>
│     │       └╼ Expression
│     │         └╼ BinaryOperatorExpression <39:5..39:16>
│     │           ├╼ Operator -> AssignPlus <39:7..39:9>
│     │           ├╼ LHS
│     │           │ └╼ Identifier -> "i" <39:5..39:6>
│     │           └╼ RHS
│     │             └╼ BinaryOperatorExpression <39:10..39:16>
│     │               ├╼ Operator -> Modulo <39:13..39:14>
│     │               ├╼ LHS
│     │               │ └╼ Constant
│     │               │   └╼ Integer -> Generic(23), <39:10..39:12>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(4), <39:15..39:16>
│     ├╼ ForStatement <42:3..47:25>
│     │ ├╼ ForInitializer
│     │ │ └╼ Declaration <42:8..42:18>
│     │ │   ├╼ DeclarationSpecifiers
│     │ │   │ └╼ TypeSpecifier -> Int <42:8..42:11>
│     │ │   └╼ InitDeclaratorList
│     │ │     └╼ InitDeclarator
│     │ │       ├╼ DirectDeclarator -> "i" <42:12..42:13>
│     │ │       └╼ Initializer
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(0), <42:16..42:17>
│     │ ├╼ ForCondition
│     │ │ └╼ BinaryOperatorExpression <42:19..42:25>
│     │ │   ├╼ Operator -> Less <42:21..42:22>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <42:19..42:20>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(12), <42:23..42:25>
│     │ ├╼ ForStepExpression
│     │ │ └╼ BinaryOperatorExpression <42:27..42:32>
│     │ │   ├╼ Operator -> Minus <42:29..42:30>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <42:27..42:28>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(1), <42:31..42:32>
│     │ └╼ ForBlock
│     │   └╼ CompoundStatement <42:35..47:25>
│     │     ├╼ ForStatement <43:5..44:16>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ Empty
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <44:7..44:16>
│     │     ├╼ Declaration <46:5..46:22>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <46:5..46:8>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <46:9..46:16>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <46:19..46:21>
│     │     └╼ ReturnStatement <47:5..47:25>
│     │       └╼ BinaryOperatorExpression <47:5..47:25>
│     │         ├╼ Operator -> Multiply <47:15..47:16>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <47:12..47:14>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <47:17..47:24>
│     ├╼ ExpressionStatement <50:3..50:9>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <50:3..50:8>
│     │     ├╼ Operator -> Multiply <50:5..50:6>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <50:3..50:4>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <50:7..50:8>
│     ├╼ ExpressionStatement <51:3..51:4>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <52:3..52:19>
│     │ └╼ Identifier -> "Checkpoint" <52:8..52:18>
│     ├╼ LabeledStatement <54:1..55:13>
│     │ ├╼ Identifier -> "Checkpoint" <54:1..54:11>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <55:3..55:13>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <55:3..55:13>
│     ├╼ SwitchStatement <57:3..61:13>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <57:11..57:20>
│     │ │   ├╼ Operator -> Plus <57:17..57:18>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <57:11..57:16>
│     │ │   │   ├╼ Operator -> Multiply <57:13..57:14>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <57:11..57:12>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <57:15..57:16>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <57:19..57:20>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <57:23..61:13>
│     │     ├╼ CaseStatement <58:5..59:45>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <58:10..58:16>
│     │     │ │   ├╼ Operator -> Multiply <58:12..58:13>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <58:10..58:11>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <58:14..58:16>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <59:7..59:45>
│     │     │     └╼ BinaryOperatorExpression <59:7..59:45>
│     │     │       ├╼ Operator -> Multiply <59:33..59:34>
│     │     │       ├╼ LHS
│     │     │       │ └╼ UnaryOperatorExpression <59:14..59:32>
│     │     │       │   ├╼ Operator -> PreDecrement <59:14..59:16>
│     │     │       │   └╼ Expression
│     │     │       │     └╼ Identifier -> "g_GlobalVariable" <59:16..59:32>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <59:35..59:44>
│     │     └╼ DefaultStatement <60:5..61:13>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <61:7..61:13>
│     └╼ CompoundStatement <64:4..75:12>
│       └╼ ReturnStatement <65:5..75:12>
│         └╼ TernaryOperatorExpression <65:5..75:12>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <65:12..65:29>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <65:32..74:8>
│           │   ├╼ Operator -> LogicalOr <65:53..65:55>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <65:32..65:52>
│           │   │   ├╼ Operator -> LogicalAnd <65:48..65:50>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <65:32..65:47>
│           │   │   │   ├╼ Operator -> BitwiseOr <65:44..65:45>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <65:32..65:43>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <65:39..65:40>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <65:32..65:38>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <65:34..65:35>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <65:32..65:33>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <65:36..65:38>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <65:41..65:43>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <65:46..65:47>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <65:51..65:52>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <66:8..74:8>
│           │       ├╼ Operator -> NotEquals <73:10..73:12>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <66:8..73:9>
│           │       │   ├╼ Operator -> GreaterOrEqual <72:10..72:12>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <66:8..72:9>
│           │       │   │   ├╼ Operator -> Minus <71:13..71:14>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <66:8..71:12>
│           │       │   │   │   ├╼ Operator -> Multiply <70:12..70:13>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <66:8..70:10>
│           │       │   │   │   │   ├╼ Operator -> Multiply <66:43..66:44>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <66:8..66:41>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <66:37..66:38>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <66:8..66:36>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <66:32..66:34>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <66:8..66:31>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <66:27..66:28>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ UnaryOperatorExpression <66:8..66:26>
│           │       │   │   │   │   │   │   │   │   ├╼ Operator -> PostIncrement <66:24..66:26>
│           │       │   │   │   │   │   │   │   │   └╼ Expression
│           │       │   │   │   │   │   │   │   │     └╼ Identifier -> "g_GlobalVariable" <66:8..66:24>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <66:29..66:31>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <66:35..66:36>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <66:39..66:41>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <67:10..70:10>
│           │       │   │   │   │       ├╼ Operator -> Multiply <69:13..69:14>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <67:10..69:11>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <68:27..68:28>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <67:10..68:25>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <67:42..67:44>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <67:10..67:41>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <67:14..67:15>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <67:10..67:13>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ CastExpression <67:16..67:41>
│           │       │   │   │   │       │   │   │       ├╼ Typename
│           │       │   │   │   │       │   │   │       │ └╼ SpecifierQualifiers
│           │       │   │   │   │       │   │   │       │   └╼ Double <67:17..67:23>
│           │       │   │   │   │       │   │   │       └╼ Expression
│           │       │   │   │   │       │   │   │         └╼ Identifier -> "_g_AssumeABoolean" <67:24..67:41>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <68:9..68:25>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <68:21..68:23>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ UnaryOperatorExpression <68:9..68:20>
│           │       │   │   │   │       │   │       │   ├╼ Operator -> PreIncrement <68:9..68:11>
│           │       │   │   │   │       │   │       │   └╼ Expression
│           │       │   │   │   │       │   │       │     └╼ Identifier -> "g_NiceVar" <68:11..68:20>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <68:24..68:25>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <69:9..69:11>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <70:9..70:10>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <71:7..71:12>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <72:7..72:9>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <73:7..73:9>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <74:7..74:8>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <75:9..75:11>
├╼ FunctionDefinition <79:1..79:26>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <79:1..79:7>
│ │ │ └╼ TypeSpecifier -> Float <79:8..79:13>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <79:14..86:14>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <80:1..80:56>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <80:1..80:6>
│ │ └╼ TypeSpecifier -> Int <80:7..80:10>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <80:11..92:11>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <80:24..80:40>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <80:24..80:29>
│         │ │ └╼ TypeSpecifier -> Int <80:30..80:33>
│         │ └╼ DirectDeclarator -> "param1" <80:34..80:40>
│         └╼ FunctionParameter <80:42..80:54>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <80:42..80:47>
│           └╼ DirectDeclarator -> "param2" <80:48..80:54>
└╼ Declaration <81:1..81:49>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <81:1..81:9>
  │ └╼ TypeSpecifier -> Int <81:10..81:13>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <81:14..87:14>
        └╼ FunctionParameters
          ├╼ FunctionParameter <81:21..81:34>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <81:21..81:29>
          │ │ └╼ TypeSpecifier -> Int <81:30..81:33>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <81:35..81:48>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <81:35..81:43>
            │ └╼ TypeSpecifier -> Int <81:44..81:47>
            └╼ DirectDeclarator -> None
```

## Getting Started

To run the command line tool:

```sh
cargo run -- --dump-ast <path/to/source/file>
```

Use the `--dump-ast` option to display the Abstract Syntax Tree of the given program.