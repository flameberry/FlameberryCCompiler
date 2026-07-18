// expect: 42
// Case 13: unary operators. Covers -, ~, ! and + on variables and constants,
// nested/double application, and unary results feeding conditions.

int main() {
	int x = 5;

	int neg = -x;        // -5
	int comp = ~x;       // -6
	int not_true = !x;   // 0
	int not_zero = !0;   // 1

	int double_neg = -(-x); // 5
	int not_not = !!x;      // 1
	int plus = +x;          // 5

	int cond = 0;
	if (!cond)
		cond = ~(-9); // 8

	// -5 + -6 + 0 + 1 + 5 + 1 + 5 + 8 = 9
	int sum = neg + comp + not_true + not_zero + double_neg + not_not + plus + cond;
	return sum + 33; // 42
}
