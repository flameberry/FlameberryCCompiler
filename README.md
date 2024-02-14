# Flameberry C Compiler (Written in Rust)
Flameberry C compiler is primitive C (ISO 17 standard compliant) compiler which is in early stages and the future plan is to make it a fully featured compiler with decent performance.

**Currently it supports:**
1. Preprocessing comments
2. Lexing (almost) every kind of C token
3. Parsing of:
    1. External Declarations
    2. Function Definitions
    3. Return Statements
    4. Compound Statements
    5. If Statements
    6. Break and Continue Statements
    7. Expressions
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
  } else if (g_NiceVar <= 44)
    return 69l;
  else if (g_GlobalVariable > 3)
    // This is a comment to try and mess with the tokenizer :D
    return 12ul;
  else
    // This is a comment to try and mess with the tokenizer :D
    return 0;

  int i = 0;
  sizeof(sizeof(2));
  sizeof(((5 * 3)));
  sizeof((((i))))++ ++;
  while (i < 10) {
    while (1)
      return 2;
    do {
      char character = 'Z' + 'D';
    } while (i == sizeof(5 - 3));

    int j = i + 1;
    wow(&i, j)(&j, i);
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
    return _g_AssumeABoolean ? 4 & 12 ^ 13 | 1 && 5 || (++g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == ++g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0 : 77;
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
├╼ FunctionDefinition <117..1854>
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
│   └╼ CompoundStatement <133..1848>
│     ├╼ IfStatement <197..561>
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
│     │   └╼ IfStatement <328..561>
│     │     ├╼ IfExpression
│     │     │ └╼ BinaryOperatorExpression <332..347>
│     │     │   ├╼ Operator -> LessOrEqual <342..344>
│     │     │   ├╼ LHS
│     │     │   │ └╼ Identifier -> "g_NiceVar" <332..341>
│     │     │   └╼ RHS
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(44), <345..347>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <353..364>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <353..364>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <372..561>
│     │         ├╼ IfExpression
│     │         │ └╼ BinaryOperatorExpression <376..396>
│     │         │   ├╼ Operator -> Greater <393..394>
│     │         │   ├╼ LHS
│     │         │   │ └╼ Identifier -> "g_GlobalVariable" <376..392>
│     │         │   └╼ RHS
│     │         │     └╼ Constant
│     │         │       └╼ Integer -> Generic(3), <395..396>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <465..477>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <465..477>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <552..561>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <552..561>
│     ├╼ Declaration <565..575>
│     │ ├╼ DeclarationSpecifiers
│     │ │ └╼ TypeSpecifier -> Int <565..568>
│     │ └╼ InitDeclaratorList
│     │   └╼ InitDeclarator
│     │     ├╼ DirectDeclarator -> "i" <569..570>
│     │     └╼ Initializer
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(0), <573..574>
│     ├╼ ExpressionStatement <578..596>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <578..593>
│     │     └╼ SizeofValExpression <585..593>
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <592..593>
│     ├╼ ExpressionStatement <599..617>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <599..613>
│     │     └╼ BinaryOperatorExpression <608..613>
│     │       ├╼ Operator -> Multiply <610..611>
│     │       ├╼ LHS
│     │       │ └╼ Constant
│     │       │   └╼ Integer -> Generic(5), <608..609>
│     │       └╼ RHS
│     │         └╼ Constant
│     │           └╼ Integer -> Generic(3), <612..613>
│     ├╼ ExpressionStatement <620..641>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <620..640>
│     │     └╼ UnaryOperatorExpression <626..640>
│     │       ├╼ Operator -> PostIncrement <638..640>
│     │       └╼ Expression
│     │         └╼ UnaryOperatorExpression <626..637>
│     │           ├╼ Operator -> PostIncrement <635..637>
│     │           └╼ Expression
│     │             └╼ Identifier -> "i" <630..631>
│     ├╼ WhileStatement <644..859>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <651..657>
│     │ │   ├╼ Operator -> Less <653..654>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <651..652>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <655..657>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <660..859>
│     │     ├╼ WhileStatement <665..690>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <672..673>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <681..690>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <681..690>
│     │     ├╼ DoWhileStatement <695..767>
│     │     │ ├╼ DoBlock
│     │     │ │ └╼ CompoundStatement <699..733>
│     │     │ │   └╼ Declaration <706..733>
│     │     │ │     ├╼ DeclarationSpecifiers
│     │     │ │     │ └╼ TypeSpecifier -> Char <706..710>
│     │     │ │     └╼ InitDeclaratorList
│     │     │ │       └╼ InitDeclarator
│     │     │ │         ├╼ DirectDeclarator -> "character" <711..720>
│     │     │ │         └╼ Initializer
│     │     │ │           └╼ BinaryOperatorExpression <723..732>
│     │     │ │             ├╼ Operator -> Plus <727..728>
│     │     │ │             ├╼ LHS
│     │     │ │             │ └╼ Constant
│     │     │ │             │   └╼ Character -> 'Z'
│     │     │ │             └╼ RHS
│     │     │ │               └╼ Constant
│     │     │ │                 └╼ Character -> 'D'
│     │     │ └╼ DoWhileExpression
│     │     │   └╼ BinaryOperatorExpression <747..764>
│     │     │     ├╼ Operator -> Equals <749..751>
│     │     │     ├╼ LHS
│     │     │     │ └╼ Identifier -> "i" <747..748>
│     │     │     └╼ RHS
│     │     │       └╼ SizeofValExpression <752..764>
│     │     │         └╼ BinaryOperatorExpression <759..764>
│     │     │           ├╼ Operator -> Minus <761..762>
│     │     │           ├╼ LHS
│     │     │           │ └╼ Constant
│     │     │           │   └╼ Integer -> Generic(5), <759..760>
│     │     │           └╼ RHS
│     │     │             └╼ Constant
│     │     │               └╼ Integer -> Generic(3), <763..764>
│     │     ├╼ Declaration <773..787>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <773..776>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "j" <777..778>
│     │     │     └╼ Initializer
│     │     │       └╼ BinaryOperatorExpression <781..786>
│     │     │         ├╼ Operator -> Plus <783..784>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Identifier -> "i" <781..782>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(1), <785..786>
│     │     ├╼ ExpressionStatement <792..810>
│     │     │ └╼ Expression
│     │     │   └╼ CallExpression <792..809>
│     │     │     ├╼ CalleeExpression
│     │     │     │ └╼ CallExpression <792..802>
│     │     │     │   ├╼ CalleeExpression
│     │     │     │   │ └╼ Identifier -> "wow" <792..795>
│     │     │     │   └╼ ArgumentExpressionList
│     │     │     │     ├╼ ArgumentExpression
│     │     │     │     │ └╼ UnaryOperatorExpression <796..798>
│     │     │     │     │   ├╼ Operator -> Address <796..797>
│     │     │     │     │   └╼ Expression
│     │     │     │     │     └╼ Identifier -> "i" <797..798>
│     │     │     │     └╼ ArgumentExpression
│     │     │     │       └╼ Identifier -> "j" <800..801>
│     │     │     └╼ ArgumentExpressionList
│     │     │       ├╼ ArgumentExpression
│     │     │       │ └╼ UnaryOperatorExpression <803..805>
│     │     │       │   ├╼ Operator -> Address <803..804>
│     │     │       │   └╼ Expression
│     │     │       │     └╼ Identifier -> "j" <804..805>
│     │     │       └╼ ArgumentExpression
│     │     │         └╼ Identifier -> "i" <807..808>
│     │     ├╼ ExpressionStatement <815..825>
│     │     │ └╼ Expression
│     │     │   └╼ CallExpression <815..824>
│     │     │     ├╼ CalleeExpression
│     │     │     │ └╼ Identifier -> "AdityaG" <815..822>
│     │     │     └╼ ArgumentExpressionList
│     │     │       └╼ Empty
│     │     ├╼ ExpressionStatement <831..842>
│     │     │ └╼ Expression
│     │     │   └╼ BinaryOperatorExpression <831..841>
│     │     │     ├╼ Operator -> Assign <834..835>
│     │     │     ├╼ LHS
│     │     │     │ └╼ UnaryOperatorExpression <831..833>
│     │     │     │   ├╼ Operator -> Indirection <831..832>
│     │     │     │   └╼ Expression
│     │     │     │     └╼ Identifier -> "i" <832..833>
│     │     │     └╼ RHS
│     │     │       └╼ BinaryOperatorExpression <836..841>
│     │     │         ├╼ Operator -> Plus <838..839>
│     │     │         ├╼ LHS
│     │     │         │ └╼ Constant
│     │     │         │   └╼ Integer -> Generic(2), <836..837>
│     │     │         └╼ RHS
│     │     │           └╼ Constant
│     │     │             └╼ Integer -> Generic(2), <840..841>
│     │     └╼ ExpressionStatement <847..859>
│     │       └╼ Expression
│     │         └╼ BinaryOperatorExpression <847..858>
│     │           ├╼ Operator -> AssignPlus <849..851>
│     │           ├╼ LHS
│     │           │ └╼ Identifier -> "i" <847..848>
│     │           └╼ RHS
│     │             └╼ BinaryOperatorExpression <852..858>
│     │               ├╼ Operator -> Modulo <855..856>
│     │               ├╼ LHS
│     │               │ └╼ Constant
│     │               │   └╼ Integer -> Generic(23), <852..854>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(4), <857..858>
│     ├╼ ForStatement <867..976>
│     │ ├╼ ForInitializer
│     │ │ └╼ Declaration <872..882>
│     │ │   ├╼ DeclarationSpecifiers
│     │ │   │ └╼ TypeSpecifier -> Int <872..875>
│     │ │   └╼ InitDeclaratorList
│     │ │     └╼ InitDeclarator
│     │ │       ├╼ DirectDeclarator -> "i" <876..877>
│     │ │       └╼ Initializer
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(0), <880..881>
│     │ ├╼ ForCondition
│     │ │ └╼ BinaryOperatorExpression <883..889>
│     │ │   ├╼ Operator -> Less <885..886>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <883..884>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(12), <887..889>
│     │ ├╼ ForStepExpression
│     │ │ └╼ BinaryOperatorExpression <891..896>
│     │ │   ├╼ Operator -> Minus <893..894>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <891..892>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(1), <895..896>
│     │ └╼ ForBlock
│     │   └╼ CompoundStatement <899..976>
│     │     ├╼ ForStatement <904..928>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ Empty
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <919..928>
│     │     ├╼ Declaration <934..951>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <934..937>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <938..945>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <948..950>
│     │     └╼ ReturnStatement <956..976>
│     │       └╼ BinaryOperatorExpression <956..976>
│     │         ├╼ Operator -> Multiply <966..967>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <963..965>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <968..975>
│     ├╼ ExpressionStatement <984..990>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <984..989>
│     │     ├╼ Operator -> Multiply <986..987>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <984..985>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <988..989>
│     ├╼ ExpressionStatement <993..994>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <997..1013>
│     │ └╼ Identifier -> "Checkpoint" <1002..1012>
│     ├╼ LabeledStatement <1015..1039>
│     │ ├╼ Identifier -> "Checkpoint" <1015..1025>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <1029..1039>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <1029..1039>
│     ├╼ SwitchStatement <1043..1143>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <1051..1060>
│     │ │   ├╼ Operator -> Plus <1057..1058>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <1051..1056>
│     │ │   │   ├╼ Operator -> Multiply <1053..1054>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <1051..1052>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <1055..1056>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <1059..1060>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <1063..1143>
│     │     ├╼ CaseStatement <1066..1121>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <1071..1077>
│     │     │ │   ├╼ Operator -> Multiply <1073..1074>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <1071..1072>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <1075..1077>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <1083..1121>
│     │     │     └╼ BinaryOperatorExpression <1083..1121>
│     │     │       ├╼ Operator -> Multiply <1109..1110>
│     │     │       ├╼ LHS
│     │     │       │ └╼ UnaryOperatorExpression <1090..1108>
│     │     │       │   ├╼ Operator -> PreDecrement <1090..1092>
│     │     │       │   └╼ Expression
│     │     │       │     └╼ Identifier -> "g_GlobalVariable" <1092..1108>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <1111..1120>
│     │     └╼ DefaultStatement <1124..1143>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <1137..1143>
│     └╼ CompoundStatement <1212..1848>
│       └╼ ReturnStatement <1217..1848>
│         └╼ TernaryOperatorExpression <1217..1848>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1224..1241>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1244..1813>
│           │   ├╼ Operator -> LogicalOr <1265..1267>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1244..1264>
│           │   │   ├╼ Operator -> LogicalAnd <1260..1262>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1244..1259>
│           │   │   │   ├╼ Operator -> BitwiseOr <1256..1257>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1244..1255>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1251..1252>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1244..1250>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1246..1247>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1244..1245>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1248..1250>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1253..1255>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1258..1259>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1263..1264>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1304..1813>
│           │       ├╼ Operator -> NotEquals <1770..1772>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1304..1769>
│           │       │   ├╼ Operator -> GreaterOrEqual <1721..1723>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1304..1720>
│           │       │   │   ├╼ Operator -> Minus <1669..1670>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1304..1668>
│           │       │   │   │   ├╼ Operator -> Multiply <1610..1611>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1304..1608>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1339..1340>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1304..1337>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1333..1334>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1304..1332>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1328..1330>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1304..1327>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1323..1324>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ UnaryOperatorExpression <1304..1322>
│           │       │   │   │   │   │   │   │   │   ├╼ Operator -> PreIncrement <1304..1306>
│           │       │   │   │   │   │   │   │   │   └╼ Expression
│           │       │   │   │   │   │   │   │   │     └╼ Identifier -> "g_GlobalVariable" <1306..1322>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1325..1327>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1331..1332>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1335..1337>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1395..1608>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1553..1554>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1395..1551>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1494..1495>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1395..1492>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1419..1421>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1395..1418>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1399..1400>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1395..1398>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <1401..1418>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1476..1492>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1488..1490>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ UnaryOperatorExpression <1476..1487>
│           │       │   │   │   │       │   │       │   ├╼ Operator -> PreIncrement <1476..1478>
│           │       │   │   │   │       │   │       │   └╼ Expression
│           │       │   │   │   │       │   │       │     └╼ Identifier -> "g_NiceVar" <1478..1487>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1491..1492>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1549..1551>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1607..1608>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1663..1668>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1718..1720>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1767..1769>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1812..1813>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1845..1847>
├╼ FunctionDefinition <1856..1881>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1856..1862>
│ │ │ └╼ TypeSpecifier -> Float <1863..1868>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1869..1876>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1882..1937>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1882..1887>
│ │ └╼ TypeSpecifier -> Int <1888..1891>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1892..1904>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1905..1921>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1905..1910>
│         │ │ └╼ TypeSpecifier -> Int <1911..1914>
│         │ └╼ DirectDeclarator -> "param1" <1915..1921>
│         └╼ FunctionParameter <1923..1935>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1923..1928>
│           └╼ DirectDeclarator -> "param2" <1929..1935>
└╼ Declaration <1938..1986>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1938..1946>
  │ └╼ TypeSpecifier -> Int <1947..1950>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1951..1957>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1958..1971>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1958..1966>
          │ │ └╼ TypeSpecifier -> Int <1967..1970>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1972..1985>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1972..1980>
            │ └╼ TypeSpecifier -> Int <1981..1984>
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
