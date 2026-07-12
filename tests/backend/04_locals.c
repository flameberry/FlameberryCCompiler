// expect: 30
// Case 4: local variables. a*b - 5 = 35 - 5 = 30.
// Exercises slot allocation, initializers, and identifier reads.

int main() {
	int a = 5;
	int b = 7;
	int c = a * b;
	return c - 5;
}
