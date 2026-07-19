// expect: 33
// Case 15: compound assignment operators. Runs all ten through one variable
// in sequence, so a wrong result in any single step changes the exit code.
// Also checks the expression value: `y = (x += 1)` must see the new x.

int main() {
	int x = 10;

	x += 5;  // 15
	x -= 3;  // 12
	x *= 4;  // 48
	x /= 2;  // 24
	x %= 17; // 7
	x <<= 2; // 28
	x >>= 1; // 14
	x &= 30; // 14
	x |= 3;  // 15
	x ^= 5;  // 10

	int y = (x += 1); // x = 11, y = 11

	return x * 2 + y; // 33
}
