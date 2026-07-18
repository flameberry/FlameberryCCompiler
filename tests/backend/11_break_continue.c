// expect: 32
// Case 11: break and continue. Covers: break out of a while, continue in a
// for (the step expression must still run), break binding to the innermost
// loop only, continue in an outer loop with nesting, and break in a
// condition-less for (;;).

int main() {
	// break out of an infinite while
	int i = 0;
	while (1) {
		if (i == 5)
			break;
		i = i + 1;
	}
	// i = 5

	// continue must not skip the step expression, or this never terminates
	int evens = 0;
	for (int j = 0; j < 10; j = j + 1) {
		if (j % 2 == 1)
			continue;
		evens = evens + j;
	}
	// evens = 0 + 2 + 4 + 6 + 8 = 20

	// break binds to the inner while; continue binds to the outer for
	int nested = 0;
	for (int a = 0; a < 3; a = a + 1) {
		int b = 0;
		while (b < 10) {
			if (b == 2)
				break;
			b = b + 1;
		}
		if (a == 1)
			continue;
		nested = nested + b;
	}
	// nested = 2 + 2 = 4 (a == 1 skipped)

	// break out of a for with no condition and no step
	int k = 0;
	for (;;) {
		k = k + 1;
		if (k == 3)
			break;
	}
	// k = 3

	return i + evens + nested + k; // 5 + 20 + 4 + 3 = 32
}
