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
  if (_g_AssumeABoolean + 3 < sizeof(const float))
  {
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
  sizeof(sizeof(2));
  sizeof(((5 * 3)));
  sizeof((((i))))++;
  while (i < 10)
  {
    while (1)
      return 2;
    do
    {
      char character = 'Z' + 'D';
    } while (i == sizeof(5 - 3));

    int j = i + 1;
  }

  for (int i = 0; i < 12; i - 1)
  {
    for (;;)
      continue;

    int counter = 45;
    return 20 * counter;
  }

  2 * 2;
  ;
  goto Checkpoint;

Checkpoint: return 34;

  switch (i * i + i)
  {
    case 2 * 32: return --g_GlobalVariable * g_NiceVar;
    default: break;
  }
  // This is a comment to try and mess with the tokenizer :D
  { return _g_AssumeABoolean ? 4 & 12 ^ 13 | 1 && 5 || (++g_GlobalVariable + 2u << 2 & 31) * (((34l + _g_AssumeABoolean == ++g_NiceVar >> 3) * 34) * 4) * 23.6f - 45 >= 25 != 0 : 77; }
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
├╼ FunctionDefinition <117..1328>
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
│   └╼ CompoundStatement <133..1324>
│     ├╼ IfStatement <197..565>
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
│     │ │ └╼ CompoundStatement <249..320>
│     │ │   ├╼ Declaration <254..268>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <254..258>
│     │ │   │ └╼ InitDeclaratorList
│     │ │   │   └╼ InitDeclarator
│     │ │   │     ├╼ DirectDeclarator -> "ch" <259..261>
│     │ │   │     └╼ Initializer
│     │ │   │       └╼ Constant
│     │ │   │         └╼ Character -> 'A'
│     │ │   └╼ ReturnStatement <273..320>
│     │ │     └╼ TernaryOperatorExpression <273..320>
│     │ │       ├╼ Condition
│     │ │       │ └╼ Identifier -> "g_NiceVar" <280..289>
│     │ │       ├╼ IfExpression
│     │ │       │ └╼ TernaryOperatorExpression <292..314>
│     │ │       │   ├╼ Condition
│     │ │       │   │ └╼ BinaryOperatorExpression <292..305>
│     │ │       │   │   ├╼ Operator -> Greater <302..303>
│     │ │       │   │   ├╼ LHS
│     │ │       │   │   │ └╼ SizeofValExpression <292..301>
│     │ │       │   │   │   └╼ Identifier -> "ch" <299..301>
│     │ │       │   │   └╼ RHS
│     │ │       │   │     └╼ Constant
│     │ │       │   │       └╼ Integer -> Generic(1), <304..305>
│     │ │       │   ├╼ IfExpression
│     │ │       │   │ └╼ Identifier -> "ch" <308..310>
│     │ │       │   └╼ ElseExpression
│     │ │       │     └╼ Constant
│     │ │       │       └╼ Integer -> Generic(0), <313..314>
│     │ │       └╼ ElseExpression
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(11), <317..319>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <332..565>
│     │     ├╼ IfExpression
│     │     │ └╼ BinaryOperatorExpression <336..351>
│     │     │   ├╼ Operator -> LessOrEqual <346..348>
│     │     │   ├╼ LHS
│     │     │   │ └╼ Identifier -> "g_NiceVar" <336..345>
│     │     │   └╼ RHS
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(44), <349..351>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <357..368>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <357..368>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <376..565>
│     │         ├╼ IfExpression
│     │         │ └╼ BinaryOperatorExpression <380..400>
│     │         │   ├╼ Operator -> Greater <397..398>
│     │         │   ├╼ LHS
│     │         │   │ └╼ Identifier -> "g_GlobalVariable" <380..396>
│     │         │   └╼ RHS
│     │         │     └╼ Constant
│     │         │       └╼ Integer -> Generic(3), <399..400>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <469..481>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <469..481>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <556..565>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <556..565>
│     ├╼ Declaration <570..580>
│     │ ├╼ DeclarationSpecifiers
│     │ │ └╼ TypeSpecifier -> Int <570..573>
│     │ └╼ InitDeclaratorList
│     │   └╼ InitDeclarator
│     │     ├╼ DirectDeclarator -> "i" <574..575>
│     │     └╼ Initializer
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(0), <578..579>
│     ├╼ ExpressionStatement <583..601>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <583..598>
│     │     └╼ SizeofValExpression <590..598>
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <597..598>
│     ├╼ ExpressionStatement <604..622>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <604..618>
│     │     └╼ BinaryOperatorExpression <613..618>
│     │       ├╼ Operator -> Multiply <615..616>
│     │       ├╼ LHS
│     │       │ └╼ Constant
│     │       │   └╼ Integer -> Generic(5), <613..614>
│     │       └╼ RHS
│     │         └╼ Constant
│     │           └╼ Integer -> Generic(3), <617..618>
│     ├╼ ExpressionStatement <625..643>
│     │ └╼ Expression
│     │   └╼ SizeofValExpression <625..642>
│     │     └╼ UnaryOperatorExpression <631..642>
│     │       ├╼ Operator -> PostIncrement <640..642>
│     │       └╼ Expression
│     │         └╼ Identifier -> "i" <635..636>
│     ├╼ WhileStatement <646..795>
│     │ ├╼ WhileExpression
│     │ │ └╼ BinaryOperatorExpression <653..659>
│     │ │   ├╼ Operator -> Less <655..656>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <653..654>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(10), <657..659>
│     │ └╼ WhileBlock
│     │   └╼ CompoundStatement <664..795>
│     │     ├╼ WhileStatement <669..694>
│     │     │ ├╼ WhileExpression
│     │     │ │ └╼ Constant
│     │     │ │   └╼ Integer -> Generic(1), <676..677>
│     │     │ └╼ WhileBlock
│     │     │   └╼ ReturnStatement <685..694>
│     │     │     └╼ Constant
│     │     │       └╼ Integer -> Generic(2), <685..694>
│     │     ├╼ DoWhileStatement <699..775>
│     │     │ ├╼ DoBlock
│     │     │ │ └╼ CompoundStatement <707..741>
│     │     │ │   └╼ Declaration <714..741>
│     │     │ │     ├╼ DeclarationSpecifiers
│     │     │ │     │ └╼ TypeSpecifier -> Char <714..718>
│     │     │ │     └╼ InitDeclaratorList
│     │     │ │       └╼ InitDeclarator
│     │     │ │         ├╼ DirectDeclarator -> "character" <719..728>
│     │     │ │         └╼ Initializer
│     │     │ │           └╼ BinaryOperatorExpression <731..740>
│     │     │ │             ├╼ Operator -> Plus <735..736>
│     │     │ │             ├╼ LHS
│     │     │ │             │ └╼ Constant
│     │     │ │             │   └╼ Character -> 'Z'
│     │     │ │             └╼ RHS
│     │     │ │               └╼ Constant
│     │     │ │                 └╼ Character -> 'D'
│     │     │ └╼ DoWhileExpression
│     │     │   └╼ BinaryOperatorExpression <755..772>
│     │     │     ├╼ Operator -> Equals <757..759>
│     │     │     ├╼ LHS
│     │     │     │ └╼ Identifier -> "i" <755..756>
│     │     │     └╼ RHS
│     │     │       └╼ SizeofValExpression <760..772>
│     │     │         └╼ BinaryOperatorExpression <767..772>
│     │     │           ├╼ Operator -> Minus <769..770>
│     │     │           ├╼ LHS
│     │     │           │ └╼ Constant
│     │     │           │   └╼ Integer -> Generic(5), <767..768>
│     │     │           └╼ RHS
│     │     │             └╼ Constant
│     │     │               └╼ Integer -> Generic(3), <771..772>
│     │     └╼ Declaration <781..795>
│     │       ├╼ DeclarationSpecifiers
│     │       │ └╼ TypeSpecifier -> Int <781..784>
│     │       └╼ InitDeclaratorList
│     │         └╼ InitDeclarator
│     │           ├╼ DirectDeclarator -> "j" <785..786>
│     │           └╼ Initializer
│     │             └╼ BinaryOperatorExpression <789..794>
│     │               ├╼ Operator -> Plus <791..792>
│     │               ├╼ LHS
│     │               │ └╼ Identifier -> "i" <789..790>
│     │               └╼ RHS
│     │                 └╼ Constant
│     │                   └╼ Integer -> Generic(1), <793..794>
│     ├╼ ForStatement <803..914>
│     │ ├╼ ForInitializer
│     │ │ └╼ Declaration <808..818>
│     │ │   ├╼ DeclarationSpecifiers
│     │ │   │ └╼ TypeSpecifier -> Int <808..811>
│     │ │   └╼ InitDeclaratorList
│     │ │     └╼ InitDeclarator
│     │ │       ├╼ DirectDeclarator -> "i" <812..813>
│     │ │       └╼ Initializer
│     │ │         └╼ Constant
│     │ │           └╼ Integer -> Generic(0), <816..817>
│     │ ├╼ ForCondition
│     │ │ └╼ BinaryOperatorExpression <819..825>
│     │ │   ├╼ Operator -> Less <821..822>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <819..820>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(12), <823..825>
│     │ ├╼ ForStepExpression
│     │ │ └╼ BinaryOperatorExpression <827..832>
│     │ │   ├╼ Operator -> Minus <829..830>
│     │ │   ├╼ LHS
│     │ │   │ └╼ Identifier -> "i" <827..828>
│     │ │   └╼ RHS
│     │ │     └╼ Constant
│     │ │       └╼ Integer -> Generic(1), <831..832>
│     │ └╼ ForBlock
│     │   └╼ CompoundStatement <837..914>
│     │     ├╼ ForStatement <842..866>
│     │     │ ├╼ ForInitializer
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForCondition
│     │     │ │ └╼ Empty
│     │     │ ├╼ ForStepExpression
│     │     │ │ └╼ Empty
│     │     │ └╼ ForBlock
│     │     │   └╼ ContinueStatement <857..866>
│     │     ├╼ Declaration <872..889>
│     │     │ ├╼ DeclarationSpecifiers
│     │     │ │ └╼ TypeSpecifier -> Int <872..875>
│     │     │ └╼ InitDeclaratorList
│     │     │   └╼ InitDeclarator
│     │     │     ├╼ DirectDeclarator -> "counter" <876..883>
│     │     │     └╼ Initializer
│     │     │       └╼ Constant
│     │     │         └╼ Integer -> Generic(45), <886..888>
│     │     └╼ ReturnStatement <894..914>
│     │       └╼ BinaryOperatorExpression <894..914>
│     │         ├╼ Operator -> Multiply <904..905>
│     │         ├╼ LHS
│     │         │ └╼ Constant
│     │         │   └╼ Integer -> Generic(20), <901..903>
│     │         └╼ RHS
│     │           └╼ Identifier -> "counter" <906..913>
│     ├╼ ExpressionStatement <922..928>
│     │ └╼ Expression
│     │   └╼ BinaryOperatorExpression <922..927>
│     │     ├╼ Operator -> Multiply <924..925>
│     │     ├╼ LHS
│     │     │ └╼ Constant
│     │     │   └╼ Integer -> Generic(2), <922..923>
│     │     └╼ RHS
│     │       └╼ Constant
│     │         └╼ Integer -> Generic(2), <926..927>
│     ├╼ ExpressionStatement <931..932>
│     │ └╼ Expression
│     │   └╼ Empty
│     ├╼ GotoStatement <935..951>
│     │ └╼ Identifier -> "Checkpoint" <940..950>
│     ├╼ LabeledStatement <953..975>
│     │ ├╼ Identifier -> "Checkpoint" <953..963>
│     │ └╼ LabeledBlock
│     │   └╼ ReturnStatement <965..975>
│     │     └╼ Constant
│     │       └╼ Integer -> Generic(34), <965..975>
│     ├╼ SwitchStatement <979..1077>
│     │ ├╼ SwitchExpression
│     │ │ └╼ BinaryOperatorExpression <987..996>
│     │ │   ├╼ Operator -> Plus <993..994>
│     │ │   ├╼ LHS
│     │ │   │ └╼ BinaryOperatorExpression <987..992>
│     │ │   │   ├╼ Operator -> Multiply <989..990>
│     │ │   │   ├╼ LHS
│     │ │   │   │ └╼ Identifier -> "i" <987..988>
│     │ │   │   └╼ RHS
│     │ │   │     └╼ Identifier -> "i" <991..992>
│     │ │   └╼ RHS
│     │ │     └╼ Identifier -> "i" <995..996>
│     │ └╼ SwitchBlock
│     │   └╼ CompoundStatement <1001..1077>
│     │     ├╼ CaseStatement <1006..1057>
│     │     │ ├╼ CaseExpression
│     │     │ │ └╼ BinaryOperatorExpression <1011..1017>
│     │     │ │   ├╼ Operator -> Multiply <1013..1014>
│     │     │ │   ├╼ LHS
│     │     │ │   │ └╼ Constant
│     │     │ │   │   └╼ Integer -> Generic(2), <1011..1012>
│     │     │ │   └╼ RHS
│     │     │ │     └╼ Constant
│     │     │ │       └╼ Integer -> Generic(32), <1015..1017>
│     │     │ └╼ CaseBlock
│     │     │   └╼ ReturnStatement <1019..1057>
│     │     │     └╼ BinaryOperatorExpression <1019..1057>
│     │     │       ├╼ Operator -> Multiply <1045..1046>
│     │     │       ├╼ LHS
│     │     │       │ └╼ UnaryOperatorExpression <1026..1044>
│     │     │       │   ├╼ Operator -> PreDecrement <1026..1028>
│     │     │       │   └╼ Expression
│     │     │       │     └╼ Identifier -> "g_GlobalVariable" <1028..1044>
│     │     │       └╼ RHS
│     │     │         └╼ Identifier -> "g_NiceVar" <1047..1056>
│     │     └╼ DefaultStatement <1062..1077>
│     │       └╼ DefaultBlock
│     │         └╼ BreakStatement <1071..1077>
│     └╼ CompoundStatement <1146..1324>
│       └╼ ReturnStatement <1147..1324>
│         └╼ TernaryOperatorExpression <1147..1324>
│           ├╼ Condition
│           │ └╼ Identifier -> "_g_AssumeABoolean" <1154..1171>
│           ├╼ IfExpression
│           │ └╼ BinaryOperatorExpression <1174..1318>
│           │   ├╼ Operator -> LogicalOr <1195..1197>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <1174..1194>
│           │   │   ├╼ Operator -> LogicalAnd <1190..1192>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <1174..1189>
│           │   │   │   ├╼ Operator -> BitwiseOr <1186..1187>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ BinaryOperatorExpression <1174..1185>
│           │   │   │   │   ├╼ Operator -> BitwiseXor <1181..1182>
│           │   │   │   │   ├╼ LHS
│           │   │   │   │   │ └╼ BinaryOperatorExpression <1174..1180>
│           │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1176..1177>
│           │   │   │   │   │   ├╼ LHS
│           │   │   │   │   │   │ └╼ Constant
│           │   │   │   │   │   │   └╼ Integer -> Generic(4), <1174..1175>
│           │   │   │   │   │   └╼ RHS
│           │   │   │   │   │     └╼ Constant
│           │   │   │   │   │       └╼ Integer -> Generic(12), <1178..1180>
│           │   │   │   │   └╼ RHS
│           │   │   │   │     └╼ Constant
│           │   │   │   │       └╼ Integer -> Generic(13), <1183..1185>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Generic(1), <1188..1189>
│           │   │   └╼ RHS
│           │   │     └╼ Constant
│           │   │       └╼ Integer -> Generic(5), <1193..1194>
│           │   └╼ RHS
│           │     └╼ BinaryOperatorExpression <1199..1318>
│           │       ├╼ Operator -> NotEquals <1314..1316>
│           │       ├╼ LHS
│           │       │ └╼ BinaryOperatorExpression <1199..1313>
│           │       │   ├╼ Operator -> GreaterOrEqual <1308..1310>
│           │       │   ├╼ LHS
│           │       │   │ └╼ BinaryOperatorExpression <1199..1307>
│           │       │   │   ├╼ Operator -> Minus <1303..1304>
│           │       │   │   ├╼ LHS
│           │       │   │   │ └╼ BinaryOperatorExpression <1199..1302>
│           │       │   │   │   ├╼ Operator -> Multiply <1295..1296>
│           │       │   │   │   ├╼ LHS
│           │       │   │   │   │ └╼ BinaryOperatorExpression <1199..1293>
│           │       │   │   │   │   ├╼ Operator -> Multiply <1234..1235>
│           │       │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │ └╼ BinaryOperatorExpression <1199..1232>
│           │       │   │   │   │   │   ├╼ Operator -> BitwiseAnd <1228..1229>
│           │       │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │ └╼ BinaryOperatorExpression <1199..1227>
│           │       │   │   │   │   │   │   ├╼ Operator -> ShiftLeft <1223..1225>
│           │       │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │ └╼ BinaryOperatorExpression <1199..1222>
│           │       │   │   │   │   │   │   │   ├╼ Operator -> Plus <1218..1219>
│           │       │   │   │   │   │   │   │   ├╼ LHS
│           │       │   │   │   │   │   │   │   │ └╼ UnaryOperatorExpression <1199..1217>
│           │       │   │   │   │   │   │   │   │   ├╼ Operator -> PreIncrement <1199..1201>
│           │       │   │   │   │   │   │   │   │   └╼ Expression
│           │       │   │   │   │   │   │   │   │     └╼ Identifier -> "g_GlobalVariable" <1201..1217>
│           │       │   │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │   │       └╼ Integer -> Unsigned(2), <1220..1222>
│           │       │   │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │   │       └╼ Integer -> Generic(2), <1226..1227>
│           │       │   │   │   │   │   └╼ RHS
│           │       │   │   │   │   │     └╼ Constant
│           │       │   │   │   │   │       └╼ Integer -> Generic(31), <1230..1232>
│           │       │   │   │   │   └╼ RHS
│           │       │   │   │   │     └╼ BinaryOperatorExpression <1239..1293>
│           │       │   │   │   │       ├╼ Operator -> Multiply <1290..1291>
│           │       │   │   │   │       ├╼ LHS
│           │       │   │   │   │       │ └╼ BinaryOperatorExpression <1239..1288>
│           │       │   │   │   │       │   ├╼ Operator -> Multiply <1284..1285>
│           │       │   │   │   │       │   ├╼ LHS
│           │       │   │   │   │       │   │ └╼ BinaryOperatorExpression <1239..1282>
│           │       │   │   │   │       │   │   ├╼ Operator -> Equals <1263..1265>
│           │       │   │   │   │       │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │ └╼ BinaryOperatorExpression <1239..1262>
│           │       │   │   │   │       │   │   │   ├╼ Operator -> Plus <1243..1244>
│           │       │   │   │   │       │   │   │   ├╼ LHS
│           │       │   │   │   │       │   │   │   │ └╼ Constant
│           │       │   │   │   │       │   │   │   │   └╼ Integer -> SignedLong(34), <1239..1242>
│           │       │   │   │   │       │   │   │   └╼ RHS
│           │       │   │   │   │       │   │   │     └╼ Identifier -> "_g_AssumeABoolean" <1245..1262>
│           │       │   │   │   │       │   │   └╼ RHS
│           │       │   │   │   │       │   │     └╼ BinaryOperatorExpression <1266..1282>
│           │       │   │   │   │       │   │       ├╼ Operator -> ShiftRight <1278..1280>
│           │       │   │   │   │       │   │       ├╼ LHS
│           │       │   │   │   │       │   │       │ └╼ UnaryOperatorExpression <1266..1277>
│           │       │   │   │   │       │   │       │   ├╼ Operator -> PreIncrement <1266..1268>
│           │       │   │   │   │       │   │       │   └╼ Expression
│           │       │   │   │   │       │   │       │     └╼ Identifier -> "g_NiceVar" <1268..1277>
│           │       │   │   │   │       │   │       └╼ RHS
│           │       │   │   │   │       │   │         └╼ Constant
│           │       │   │   │   │       │   │           └╼ Integer -> Generic(3), <1281..1282>
│           │       │   │   │   │       │   └╼ RHS
│           │       │   │   │   │       │     └╼ Constant
│           │       │   │   │   │       │       └╼ Integer -> Generic(34), <1286..1288>
│           │       │   │   │   │       └╼ RHS
│           │       │   │   │   │         └╼ Constant
│           │       │   │   │   │           └╼ Integer -> Generic(4), <1292..1293>
│           │       │   │   │   └╼ RHS
│           │       │   │   │     └╼ Constant
│           │       │   │   │       └╼ Float -> Float(23.6) <1297..1302>
│           │       │   │   └╼ RHS
│           │       │   │     └╼ Constant
│           │       │   │       └╼ Integer -> Generic(45), <1305..1307>
│           │       │   └╼ RHS
│           │       │     └╼ Constant
│           │       │       └╼ Integer -> Generic(25), <1311..1313>
│           │       └╼ RHS
│           │         └╼ Constant
│           │           └╼ Integer -> Generic(0), <1317..1318>
│           └╼ ElseExpression
│             └╼ Constant
│               └╼ Integer -> Generic(77), <1321..1323>
├╼ FunctionDefinition <1330..1355>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <1330..1336>
│ │ │ └╼ TypeSpecifier -> Float <1337..1342>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <1343..1350>
│ │   └╼ FunctionParameters
│ │     └╼ Empty
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <1356..1411>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <1356..1361>
│ │ └╼ TypeSpecifier -> Int <1362..1365>
│ └╼ InitDeclaratorList
│   └╼ InitDeclarator
│     └╼ FunctionDeclarator
│       ├╼ Identifier -> "NiceFunction" <1366..1378>
│       └╼ FunctionParameters
│         ├╼ FunctionParameter <1379..1395>
│         │ ├╼ DeclarationSpecifiers
│         │ │ ├╼ TypeQualifier -> Const <1379..1384>
│         │ │ └╼ TypeSpecifier -> Int <1385..1388>
│         │ └╼ DirectDeclarator -> "param1" <1389..1395>
│         └╼ FunctionParameter <1397..1409>
│           ├╼ DeclarationSpecifiers
│           │ └╼ TypeSpecifier -> Float <1397..1402>
│           └╼ DirectDeclarator -> "param2" <1403..1409>
└╼ Declaration <1412..1460>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <1412..1420>
  │ └╼ TypeSpecifier -> Int <1421..1424>
  └╼ InitDeclaratorList
    └╼ InitDeclarator
      └╼ FunctionDeclarator
        ├╼ Identifier -> "GetSum" <1425..1431>
        └╼ FunctionParameters
          ├╼ FunctionParameter <1432..1445>
          │ ├╼ DeclarationSpecifiers
          │ │ ├╼ TypeSpecifier -> Unsigned <1432..1440>
          │ │ └╼ TypeSpecifier -> Int <1441..1444>
          │ └╼ DirectDeclarator -> None
          └╼ FunctionParameter <1446..1459>
            ├╼ DeclarationSpecifiers
            │ ├╼ TypeSpecifier -> Unsigned <1446..1454>
            │ └╼ TypeSpecifier -> Int <1455..1458>
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
