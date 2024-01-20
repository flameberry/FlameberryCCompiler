int main() {
    // Arithmetic operators
    int a = 5, b = 3;
    int sum = a + b;
    int difference = a - b;
    int product = a * b;
    int quotient = a / b;
    int remainder = a % b;

    printf("Sum: %d\n", sum);
    printf("Difference: %d\n", difference);
    printf("Product: %d\n", product);
    printf("Quotient: %d\n", quotient);
    printf("Remainder: %d\n", remainder);

    // Relational operators
    int x = 10, y = 20;
    printf("Is x equal to y? %d\n", x == y);
    printf("Is x not equal to y? %d\n", x != y);
    printf("Is x less than y? %d\n", x < y);
    printf("Is x greater than y? %d\n", x > y);
    printf("Is x less than or equal to y? %d\n", x <= y);
    printf("Is x greater than or equal to y? %d\n", x >= y);

    // Logical operators
    int condition1 = 1, condition2 = 0;
    printf("Logical AND: %d\n", condition1 && condition2);
    printf("Logical OR: %d\n", condition1 || condition2);
    printf("Logical NOT: %d\n", !condition1);

    // Bitwise operators
    int m = 5, n = 3;
    printf("Bitwise AND: %d\n", m & n);
    printf("Bitwise OR: %d\n", m | n);
    printf("Bitwise XOR: %d\n", m ^ n);
    printf("Bitwise NOT: %d\n", ~m);

    // Assignment operators
    int variable = 10;
    variable += 5;
    printf("Updated variable: %d\n", variable);

    // Constants
    const double PI = 3.14159;
    printf("Value of PI: %f\n", PI);

    return 0;
}