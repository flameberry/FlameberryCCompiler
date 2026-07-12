// expect: 55
// Case 7: while loop. Sum 1..10 = 55.
// Exercises the `Label top; cond; JmpIfZero end; body; Jmp top` shape.

int main() {
	int i = 1;
	int s = 0;
	while (i <= 10) {
		s = s + i;
		i = i + 1;
	}
	return s;
}
