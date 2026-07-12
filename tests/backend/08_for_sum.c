// expect: 120
// Case 8: for loop. Sum 1..15 = 120.
// Exercises init/cond/step lowering and a loop-scoped induction variable.

int main() {
	int s = 0;
	for (int i = 1; i <= 15; i = i + 1)
		s = s + i;
	return s;
}
