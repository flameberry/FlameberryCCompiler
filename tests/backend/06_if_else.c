// expect: 55
// Case 6: if / else-if / else. n == 5 takes the middle branch -> 55.
// Exercises comparisons (cmp+cset), JmpIfZero, and branch chaining.

int main() {
	int n = 5;
	if (n < 0)
		return 100;
	else if (n == 5)
		return 55;
	else
		return 0;
}
