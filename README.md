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
        1. Primary except Generic Associations and String Literals
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
├╼ FunctionDefinition <117..1514>
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
│   └╼ CompoundStatement <133..1508>
│     ├╼ IfStatement <197..563>
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
│     │ │ └╼ CompoundStatement <247..318>
│     │ │   ├╼ Declaration <252..266>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <252..256>
│     │ │   │ └╼ InitDeclaratorList
│     │ │   │   └╼ InitDeclarator
│     │ │   │     ├╼ DirectDeclarator -> "ch" <257..259>
│     │ │   │     └╼ Initializer
│     │ │   │       └╼ Constant
│     │ │   │         └╼ Character -> 'A'
│     │ │   └╼ ReturnStatement <271..318>
│     │ │     └╼ TernaryOperatorExpression <271..318>
│     │ │       ├╼ Condition
│     │ │       │ └╼ Identifier -> "g_NiceVar" <278..287>
│     │ │       ├╼ IfExpression
│     │ │       │ └╼ TernaryOperatorExpression <290..312>
│     │ │       │   ├╼ Condition
│     │ │       │   │ └╼ BinaryOperatorExpression <290..303>
│     │ │       │   │   ├╼ Operator -> Greater <300..301>
│     │ │       │   │   ├╼ LHS
│     │ │       │   │   │ └╼ SizeofValExpression <290..299>
│     │ │       │   │   │   └╼ Identifier -> "ch" <297..299>
│     │ │       │   │   └╼ RHS
│     │ │       │   │     └╼ Constant
│     │ │       │   │       └╼ Integer -> Generic(1), <302..303>
│     │ │       │   ├╼ IfExpression
│     │ │       │   │ └╼ Identifier -> "ch" <306..308>
│     │ │       │   └╼ ElseExpression
│     │ │       │     └╼ Constant
│     │ │       │       └╼ Integer -> Generic(0), <311..312>
│     │ │       └╼ ElseExpression
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(11), <315..317>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <330..563>
│     │     ├╼ IfExpression
│     │     │ └╼ BinaryOperatorExpression <334..349>
│     │     │   ├╼ Operator -> LessOrEqual <344..346>
│     │     │   ├╼ LHS
│     │     │   │ └╼ Identifier -> "g_NiceVar" <334..343>
│     │     │   └╼ RHS
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(44), <347..349>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <355..366>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <355..366>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <374..563>
│     │         ├╼ IfExpression
│     │         │ └╼ BinaryOperatorExpression <378..398>
│     │         │   ├╼ Operator -> Greater <395..396>
│     │         │   ├╼ LHS
│     │         │   │ └╼ Identifier -> "g_GlobalVariable" <378..394>
│     │         │   └╼ RHS
│     │         │     └╼ Constant
│     │         │       └╼ Integer -> Generic(3), <397..398>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <467..479>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <467..479>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <554..563>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <554..563>
│     ├╼ Declaration <567..577>
│     │ ├╼ DeclarationSpecifiers
│     │ │ └╼ TypeSpecifier -> Int <567..570>
│     │ └╼ InitDeclaratorList
│     │   └╼ InitDeclarator
│     │     ├╼ DirectDeclarator -> "i" <571..572>
│     │     └╼ Initializer
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(0), <575..576>
│     ├╼ ExpressionStatement <580..609>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <580..606>
│     │     └╼ CastExpression <587..606>
│     │       ├╼ Typename
│     │       │ └╼ SpecifierQualifiers
│     │       │   ├╼ Const <588..593>
│     │       │   └╼ Int <594..597>
│     │       └╼ Expression
│     │         └╼ SizeofValExpression <598..606>
│     │           └╼ Constant
│     │             └╼ Integer -> Generic(2), <605..606>
│     ├╼ ExpressionStatement <612..630>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <612..626>
│     │     └╼ BinaryOperatorExpression <621..626>
│     │       ├╼ Operator -> Multiply <623..624>
│     │       ├╼ LHS
│     │       │ └╼ Constant
│     │       │   └╼ Integer -> Generic(5), <621..622>
│     │       └╼ RHS
│     │         └╼ Constant
│     │           └╼ Integer -> Generic(3), <625..626>
│     ├╼ ExpressionStatement <633..653>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <633..652>
│     │     └╼ UnaryOperatorExpression <639..652>
│     │       ├╼ Operator -> PostIncrement <650..652>
│     │       └╼ Expression
│     │         └╼ UnaryOperatorExpression <639..650>
│     │           ├╼ Operator -> PostIncrement <648..650>
│     │           └╼ Expression
│     │             └╼ Identifier -> "i" <643..644>
│     ├╼ WhileStatement <656..891>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <663..669>
│     │ │   ├╼ Operator -> Less <665..666>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <663..664>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <667..669>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <672..891>
│     │     ├╼ WhileStatement <677..702>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <684..685>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <693..702>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <693..702>
│     │     ├╼ DoWhileStatement <707..779>
│     │     │ ├╼ DoBlock
│     │     │ │ └╼ CompoundStatement <711..745>
│     │     │ │   └╼ Declaration <718..745>
│     │     │ │     ├╼ DeclarationSpecifiers
│     │     │ │     │ └╼ TypeSpecifier -> Char <718..722>
│     │     │ │     └╼ InitDeclaratorList
│     │     │ │       └╼ InitDeclarator
│     │     │ │         ├╼ DirectDeclarator -> "character" <723..732>
│     │     │ │         └╼ Initializer
│     │     │ │           └╼ BinaryOperatorExpression <735..744>
│     │     │ │             ├╼ Operator -> Plus <739..740>
│     │     │ │             ├╼ LHS
│     │     │ │             │ └╼ Constant
│     │     │ │             │   └╼ Character -> 'Z'
│     │     │ │             └╼ RHS
│     │     │ │               └╼ Constant
│     │     │ │                 └╼ Character -> 'D'
│     │     │ └╼ DoWhileExpression
│     │     │   └╼ BinaryOperatorExpression <759..776>
│     │     │     ├╼ Operator -> Equals <761..763>
│     │     │     ├╼ LHS
│     │     │     │ └╼ Identifier -> "i" <759..760>
│     │     │     └╼ RHS
│     │     │       └╼ SizeofValExpression <764..776>
│     │     │         └╼ BinaryOperatorExpression <771..776>
│     │     │           ├╼ Operator -> Minus <773..774>
│     │     │           ├╼ LHS
│     │     │           │ └╼ Constant
│     │     │           │   └╼ Integer -> Generic(5), <771..772>
│     │     │           └╼ RHS
│     │     │             └╼ Constant
│     │     │               └╼ Integer -> Generic(3), <775..776>
│     │     ├╼ Declaration <785..799>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <785..788>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "j" <789..790>
│     │     │     └╼ Initializer
│     │     │       └╼ BinaryOperatorExpression <793..798>
│     │     │         ├╼ Operator -> Plus <795..796>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Identifier -> "i" <793..794>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(1), <797..798>
│     │     ├╼ Declaration <804..842>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <804..807>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "result" <808..814>
│     │     │     └╼ Initializer
│     │     │       └╼ CastExpression <817..841>
│     │     │         ├╼ Typename
│     │     │         │ └╼ SpecifierQualifiers
│     │     │         │   └╼ Float <818..823>
│     │     │         └╼ Expression
│     │     │           └╼ CallExpression <824..841>
│     │     │             ├╼ CalleeExpression
│     │     │             │ └╼ CallExpression <824..834>
│     │     │             │   ├╼ CalleeExpression
│     │     │             │   │ └╼ Identifier -> "wow" <824..827>
│     │     │             │   └╼ ArgumentExpressionList
│     │     │             │     ├╼ ArgumentExpression
│     │     │             │     │ └╼ UnaryOperatorExpression <828..830>
│     │     │             │     │   ├╼ Operator -> Address <828..829>
│     │     │             │     │   └╼ Expression
│     │     │             │     │     └╼ Identifier -> "i" <829..830>
│     │     │             │     └╼ ArgumentExpression
│     │     │             │       └╼ Identifier -> "j" <832..833>
│     │     │             └╼ ArgumentExpressionList
│     │     │               ├╼ ArgumentExpression
│     │     │               │ └╼ UnaryOperatorExpression <835..837>
│     │     │               │   ├╼ Operator -> Address <835..836>
│     │     │               │   └╼ Expression
│     │     │               │     └╼ Identifier -> "j" <836..837>
│     │     │               └╼ ArgumentExpression
│     │     │                 └╼ Identifier -> "i" <839..840>
│     │     ├╼ ExpressionStatement <847..857>
│     │     │ └╼ Expression
│     │     │   └╼ CallExpression <847..856>
│     │     │     ├╼ CalleeExpression
│     │     │     │ └╼ Identifier -> "AdityaG" <847..854>
│     │     │     └╼ ArgumentExpressionList
│     │     │       └╼ Empty
│     │     ├╼ ExpressionStatement <863..874>
│     │     │ └╼ Expression
│     │     │   └╼ BinaryOperatorExpression <863..873>
│     │     │     ├╼ Operator -> Assign <866..867>
│     │     │     ├╼ LHS
│     │     │     │ └╼ UnaryOperatorExpression <863..865>
│     │     │     │   ├╼ Operator -> Indirection <863..864>
│     │     │     │   └╼ Expression
│     │     │     │     └╼ Identifier -> "i" <864..865>
│     │     │     └╼ RHS
│     │     │       └╼ BinaryOperatorExpression <868..873>
│     │     │         ├╼ Operator -> Plus <870..871>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Constant
│     │     │         │   └╼ Integer -> Generic(2), <868..869>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(2), <872..873>
│     │     └╼ ExpressionStatement <879..891>
│     │       └╼ Expression
│     │         └╼ BinaryOperatorExpression <879..890>
│     │           ├╼ Operator -> AssignPlus <881..883>
│     │           ├╼ LHS
│     │           │ └╼ Identifier -> "i" <879..880>
│     │           └╼ RHS
│     │             └╼ BinaryOperatorExpression <884..890>
│     │               ├╼ Operator -> Modulo <887..888>
│     │               ├╼ LHS
│     │               │ └╼ Constant
│     │               │   └╼ Integer -> Generic(23), <884..886>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(4), <889..890>
│     ├╼ ForStatement <899..1008>
│     │ ├╼ ForInitializer
│     │ │ └╼ Declaration <904..914>
│     │ │   ├╼ DeclarationSpecifiers
│     │ │   │ └╼ TypeSpecifier -> Int <904..907>
│     │ │   └╼ InitDeclaratorList
│     │ │     └╼ InitDeclarator
│     │ │       ├╼ DirectDeclarator -> "i" <908..909>
│     │ │       └╼ Initializer
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(0), <912..913>
│     │ ├╼ ForCondition
│     │ │ └╼ BinaryOperatorExpression <915..921>
│     │ │   ├╼ Operator -> Less <917..918>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <915..916>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(12), <919..921>
│     │ ├╼ ForStepExpression
│     │ │ └╼ BinaryOperatorExpression <923..928>
│     │ │   ├╼ Operator -> Minus <925..926>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <923..924>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(1), <927..928>
│     │ └╼ ForBlock
│     │   └╼ CompoundStatement <931..1008>
│     │     ├╼ ForStatement <936..960>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ Empty
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <951..960>
│     │     ├╼ Declaration <966..983>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <966..969>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <970..977>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <980..982>
│     │     └╼ ReturnStatement <988..1008>
│     │       └╼ BinaryOperatorExpression <988..1008>
│     │         ├╼ Operator -> Multiply <998..999>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <995..997>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <1000..1007>
│     ├╼ ExpressionStatement <1016..1022>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <1016..1021>
│     │     ├╼ Operator -> Multiply <1018..1019>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <1016..1017>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <1020..1021>
│     ├╼ ExpressionStatement <1025..1026>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <1029..1045>
│     │ └╼ Identifier -> "Checkpoint" <1034..1044>
│     ├╼ LabeledStatement <1047..1071>
│     │ ├╼ Identifier -> "Checkpoint" <1047..1057>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <1061..1071>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <1061..1071>
│     ├╼ SwitchStatement <1075..1183>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <1083..1092>
│     │ │   ├╼ Operator -> Plus <1089..1090>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <1083..1088>
│     │ │   │   ├╼ Operator -> Multiply <1085..1086>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <1083..1084>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <1087..1088>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <1091..1092>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <1095..1183>
│     │     ├╼ CaseStatement <1100..1157>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <1105..1111>
│     │     │ │   ├╼ Operator -> Multiply <1107..1108>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <1105..1106>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <1109..1111>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <1119..1157>
│     │     │     └╼ BinaryOperatorExpression <1119..1157>
│     │     │       ├╼ Operator -> Multiply <1145..1146>
│     │     │       ├╼ LHS
│     │     │       │ └╼ UnaryOperatorExpression <1126..1144>
│     │     │       │   ├╼ Operator -> PreDecrement <1126..1128>
│     │     │       │   └╼ Expression
│     │     │       │     └╼ Identifier -> "g_GlobalVariable" <1128..1144>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <1147..1156>
│     │     └╼ DefaultStatement <1162..1183>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <1177..1183>
│     └╼ CompoundStatement <1252..1508>
│       └╼ ReturnStatement <1257..1508>
│         └╼ TernaryOperatorExpression <1257..1508>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1264..1281>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1284..1496>
│           │   ├╼ Operator -> LogicalOr <1305..1307>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1284..1304>
│           │   │   ├╼ Operator -> LogicalAnd <1300..1302>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1284..1299>
│           │   │   │   ├╼ Operator -> BitwiseOr <1296..1297>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1284..1295>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1291..1292>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1284..1290>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1286..1287>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1284..1285>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1288..1290>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1293..1295>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1298..1299>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1303..1304>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1315..1496>
│           │       ├╼ Operator -> NotEquals <1486..1488>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1315..1485>
│           │       │   ├╼ Operator -> GreaterOrEqual <1474..1476>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1315..1473>
│           │       │   │   ├╼ Operator -> Minus <1463..1464>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1315..1462>
│           │       │   │   │   ├╼ Operator -> Multiply <1449..1450>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1315..1447>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1350..1351>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1315..1348>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1344..1345>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1315..1343>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1339..1341>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1315..1338>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1334..1335>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ UnaryOperatorExpression <1315..1333>
│           │       │   │   │   │   │   │   │   │   ├╼ Operator -> PostIncrement <1331..1333>
│           │       │   │   │   │   │   │   │   │   └╼ Expression
│           │       │   │   │   │   │   │   │   │     └╼ Identifier -> "g_GlobalVariable" <1315..1331>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1336..1338>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1342..1343>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1346..1348>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1361..1447>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1436..1437>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1361..1434>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1422..1423>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1361..1420>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1393..1395>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1361..1392>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1365..1366>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1361..1364>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ CastExpression <1367..1392>
│           │       │   │   │   │       │   │   │       ├╼ Typename
│           │       │   │   │   │       │   │   │       │ └╼ SpecifierQualifiers
│           │       │   │   │   │       │   │   │       │   └╼ Double <1368..1374>
│           │       │   │   │   │       │   │   │       └╼ Expression
│           │       │   │   │   │       │   │   │         └╼ Identifier -> "_g_AssumeABoolean" <1375..1392>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1404..1420>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1416..1418>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ UnaryOperatorExpression <1404..1415>
│           │       │   │   │   │       │   │       │   ├╼ Operator -> PreIncrement <1404..1406>
│           │       │   │   │   │       │   │       │   └╼ Expression
│           │       │   │   │   │       │   │       │     └╼ Identifier -> "g_NiceVar" <1406..1415>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1419..1420>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1432..1434>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1446..1447>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1457..1462>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1471..1473>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1483..1485>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1495..1496>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1505..1507>
├╼ FunctionDefinition <1516..1541>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1516..1522>
│ │ │ └╼ TypeSpecifier -> Float <1523..1528>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1529..1536>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1542..1597>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1542..1547>
│ │ └╼ TypeSpecifier -> Int <1548..1551>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1552..1564>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1565..1581>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1565..1570>
│         │ │ └╼ TypeSpecifier -> Int <1571..1574>
│         │ └╼ DirectDeclarator -> "param1" <1575..1581>
│         └╼ FunctionParameter <1583..1595>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1583..1588>
│           └╼ DirectDeclarator -> "param2" <1589..1595>
└╼ Declaration <1598..1646>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1598..1606>
  │ └╼ TypeSpecifier -> Int <1607..1610>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1611..1617>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1618..1631>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1618..1626>
          │ │ └╼ TypeSpecifier -> Int <1627..1630>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1632..1645>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1632..1640>
            │ └╼ TypeSpecifier -> Int <1641..1644>
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
