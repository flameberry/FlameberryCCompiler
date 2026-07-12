// expect: 120
// Case 10: recursion. factorial(5) = 120.
// Exercises self-calls, the base-case branch, and correct frame save/restore
// across recursive calls (x30/x29 must survive the `bl`).

int factorial(int n) {
	if (n <= 1)
		return 1;
	return n * factorial(n - 1);
}

int main() {
	return factorial(5);
}
