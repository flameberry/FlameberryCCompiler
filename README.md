# Flameberry C Compiler (Written in Rust)
Flameberry C compiler is primitive C (ISO 17 standard compliant) compiler which is in early stages and the future plan is to make it a fully featured compiler with decent performance.

**Currently it supports:**
1. Preprocessing comments
2. Lexing (almost) every kind of C token
3. Parsing of:
    1. External Declarations
    2. Function Definitions
    3. Return Statements
    3. Compound Statements
    3. If Statements
    3. Break and Continue Statements
    4. Additive Expressions
    5. Multiplicative Expressions
    5. Primary Expressions
4. Generation of basic Abstract Syntax Tree for the C Translation Unit

## Demo
For the following Sandbox/test.c program, the generated AST is:
```C
static int g_GlobalVariable;
static int g_NiceVar;
static _Bool _g_AssumeABoolean;

int main(void) {
    if (_g_AssumeABoolean)
    {
        char character;
        return character;
    }
    else if (g_NiceVar)
        return 69l;
    else if (g_GlobalVariable)
        return 12ul;
    else
        return 0;

    break;
    continue;

    {}
    { return (g_GlobalVariable + 2u) * (((34l + g_NiceVar) * 34) * 4) * 23.6f - 45; }
}

inline float AdityaG() {}
const int NiceFunction(const int param1, float param2);
unsigned int GetSum(unsigned int, unsigned int);
```

```sh
TranslationUnit
├╼ Declaration <0..27>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <0..6>
│ │ └╼ TypeSpecifier -> Int <7..10>
│ └╼ DirectDeclarator -> "g_GlobalVariable" <11..27>
├╼ Declaration <29..49>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <29..35>
│ │ └╼ TypeSpecifier -> Int <36..39>
│ └╼ DirectDeclarator -> "g_NiceVar" <40..49>
├╼ Declaration <51..81>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ StorageClassSpecifier -> Static <51..57>
│ │ └╼ TypeSpecifier -> Bool <58..63>
│ └╼ DirectDeclarator -> "_g_AssumeABoolean" <64..81>
├╼ FunctionDefinition <84..434>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ └╼ TypeSpecifier -> Int <84..87>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "main" <88..92>
│ │   └╼ FunctionParameters
│ │     └╼ FunctionParameter <93..98>
│ │       ├╼ DeclarationSpecifiers
│ │       │ └╼ TypeSpecifier -> Void <93..97>
│ │       └╼ DirectDeclarator -> None
│ └╼ FunctionBody
│   └╼ CompoundStatement <105..430>
│     ├╼ IfStatement <105..312>
│     │ ├╼ IfExpression
│     │ │ └╼ Identifier -> "_g_AssumeABoolean" <109..126>
│     │ ├╼ ThenStatement
│     │ │ └╼ CompoundStatement <142..183>
│     │ │   ├╼ Declaration <142..157>
│     │ │   │ ├╼ DeclarationSpecifiers
│     │ │   │ │ └╼ TypeSpecifier -> Char <142..146>
│     │ │   │ └╼ DirectDeclarator -> "character" <147..156>
│     │ │   └╼ ReturnStatement <166..183>
│     │ │     └╼ Identifier -> "character" <173..182>
│     │ └╼ ElseStatement
│     │   └╼ IfStatement <199..312>
│     │     ├╼ IfExpression
│     │     │ └╼ Identifier -> "g_NiceVar" <203..212>
│     │     ├╼ ThenStatement
│     │     │ └╼ ReturnStatement <222..233>
│     │     │   └╼ Constant
│     │     │     └╼ Integer -> SignedLong(69), <229..232>
│     │     └╼ ElseStatement
│     │       └╼ IfStatement <243..312>
│     │         ├╼ IfExpression
│     │         │ └╼ Identifier -> "g_GlobalVariable" <247..263>
│     │         ├╼ ThenStatement
│     │         │ └╼ ReturnStatement <273..285>
│     │         │   └╼ Constant
│     │         │     └╼ Integer -> UnsignedLong(12), <280..284>
│     │         └╼ ElseStatement
│     │           └╼ ReturnStatement <303..312>
│     │             └╼ Constant
│     │               └╼ Integer -> Generic(0), <310..311>
│     ├╼ BreakStatement <318..324>
│     ├╼ ContinueStatement <329..338>
│     ├╼ CompoundStatement <345..345>
│     │ └╼ Empty
│     └╼ CompoundStatement <353..430>
│       └╼ ReturnStatement <353..430>
│         └╼ BinaryOperatorExpression <361..429>
│           ├╼ Operator -> Minus <425..426>
│           ├╼ LHS
│           │ └╼ BinaryOperatorExpression <361..424>
│           │   ├╼ Operator -> Multiply <417..418>
│           │   ├╼ LHS
│           │   │ └╼ BinaryOperatorExpression <361..415>
│           │   │   ├╼ Operator -> Multiply <384..385>
│           │   │   ├╼ LHS
│           │   │   │ └╼ BinaryOperatorExpression <361..382>
│           │   │   │   ├╼ Operator -> Plus <378..379>
│           │   │   │   ├╼ LHS
│           │   │   │   │ └╼ Identifier -> "g_GlobalVariable" <361..377>
│           │   │   │   └╼ RHS
│           │   │   │     └╼ Constant
│           │   │   │       └╼ Integer -> Unsigned(2), <380..382>
│           │   │   └╼ RHS
│           │   │     └╼ BinaryOperatorExpression <389..415>
│           │   │       ├╼ Operator -> Multiply <412..413>
│           │   │       ├╼ LHS
│           │   │       │ └╼ BinaryOperatorExpression <389..410>
│           │   │       │   ├╼ Operator -> Multiply <406..407>
│           │   │       │   ├╼ LHS
│           │   │       │   │ └╼ BinaryOperatorExpression <389..404>
│           │   │       │   │   ├╼ Operator -> Plus <393..394>
│           │   │       │   │   ├╼ LHS
│           │   │       │   │   │ └╼ Constant
│           │   │       │   │   │   └╼ Integer -> SignedLong(34), <389..392>
│           │   │       │   │   └╼ RHS
│           │   │       │   │     └╼ Identifier -> "g_NiceVar" <395..404>
│           │   │       │   └╼ RHS
│           │   │       │     └╼ Constant
│           │   │       │       └╼ Integer -> Generic(34), <408..410>
│           │   │       └╼ RHS
│           │   │         └╼ Constant
│           │   │           └╼ Integer -> Generic(4), <414..415>
│           │   └╼ RHS
│           │     └╼ Constant
│           │       └╼ Float -> Float(23.6) <419..424>
│           └╼ RHS
│             └╼ Constant
│               └╼ Integer -> Generic(45), <427..429>
├╼ FunctionDefinition <436..461>
│ ├╼ FunctionDeclaration
│ │ ├╼ DeclarationSpecifiers
│ │ │ ├╼ FunctionSpecifier -> Inline <436..442>
│ │ │ └╼ TypeSpecifier -> Float <443..448>
│ │ └╼ FunctionDeclarator
│ │   ├╼ Identifier -> "AdityaG" <449..456>
│ │   └╼ FunctionParameters
│ └╼ FunctionBody
│   └╼ Empty
├╼ Declaration <462..515>
│ ├╼ DeclarationSpecifiers
│ │ ├╼ TypeQualifier -> Const <462..467>
│ │ └╼ TypeSpecifier -> Int <468..471>
│ └╼ FunctionDeclarator
│   ├╼ Identifier -> "NiceFunction" <472..484>
│   └╼ FunctionParameters
│     ├╼ FunctionParameter <485..501>
│     │ ├╼ DeclarationSpecifiers
│     │ │ ├╼ TypeQualifier -> Const <485..490>
│     │ │ └╼ TypeSpecifier -> Int <491..494>
│     │ └╼ DirectDeclarator -> "param1" <495..501>
│     └╼ FunctionParameter <503..515>
│       ├╼ DeclarationSpecifiers
│       │ └╼ TypeSpecifier -> Float <503..508>
│       └╼ DirectDeclarator -> "param2" <509..515>
└╼ Declaration <518..565>
  ├╼ DeclarationSpecifiers
  │ ├╼ TypeSpecifier -> Unsigned <518..526>
  │ └╼ TypeSpecifier -> Int <527..530>
  └╼ FunctionDeclarator
    ├╼ Identifier -> "GetSum" <531..537>
    └╼ FunctionParameters
      ├╼ FunctionParameter <538..551>
      │ ├╼ DeclarationSpecifiers
      │ │ ├╼ TypeSpecifier -> Unsigned <538..546>
      │ │ └╼ TypeSpecifier -> Int <547..550>
      │ └╼ DirectDeclarator -> None
      └╼ FunctionParameter <552..565>
        ├╼ DeclarationSpecifiers
        │ ├╼ TypeSpecifier -> Unsigned <552..560>
        │ └╼ TypeSpecifier -> Int <561..564>
        └╼ DirectDeclarator -> None
Time taken: 3.388584ms
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
