// expect: 77
// Case 12: constants that don't fit a single 16-bit mov immediate — these
// need the movz+movk sequence. Covers a 17-bit value, a 20-bit value,
// INT_MAX, and arithmetic on all of them.

int main() {
	int big = 70000;
	int bigger = 1000000;
	int max = 2147483647;

	int r1 = big % 256;      // 70000 - 273*256 = 112
	int r2 = bigger / 65536; // 15
	int r3 = max % 100;      // 47

	return r1 + r2 + r3 - 97; // 112 + 15 + 47 - 97 = 77
}
