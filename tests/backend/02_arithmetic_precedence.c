// expect: 9
// Case 2: arithmetic precedence. 2 + 3*4 - 10/2 = 2 + 12 - 5 = 9.
// Exercises add/sub/mul/sdiv and operator-precedence lowering.

int main() {
	return 2 + 3 * 4 - 10 / 2;
}
