// expect: 21
// Case 9: nested function calls. triple(7) = add(7, add(7, 7)) = 21.
// Exercises argument passing (w0..), `bl`, and a call as an argument to a call.

int add(int a, int b) {
	return a + b;
}

int triple(int x) {
	return add(x, add(x, x));
}

int main() {
	return triple(7);
}
