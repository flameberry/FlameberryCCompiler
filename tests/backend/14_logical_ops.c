// expect: 58
// Case 14: logical && and ||. The result must be exactly 1 or 0 (never the
// raw operand value — `t` is 5 here to catch that), and the right-hand side
// must not be evaluated when the left side already decides (observed via
// assignment side effects).

int main() {
	int t = 5;
	int f = 0;

	int a = t && 3; // 1, not 3 or 5
	int b = t && f; // 0
	int c = f && t; // 0
	int d = t || f; // 1, not 5
	int e = f || 7; // 1, not 7
	int g = f || f; // 0

	// short-circuit: rhs assignment must NOT run
	int x = 0;
	f && (x = 9); // lhs false, x stays 0
	int y = 0;
	t || (y = 9); // lhs true, y stays 0

	// rhs must run when the lhs does not decide
	int z = 0;
	t && (z = 4); // z = 4

	// 1 + 0 + 0 + 1 + 1 + 0 + 0 + 0 + 4 = 7
	return a + b + c + d + e + g + x + y + z + 51; // 58
}
