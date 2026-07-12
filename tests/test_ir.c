// Exercises the full backend scope: functions + params, recursion, if/else,
// while, for, local variables + reassignment, comparisons, and arithmetic
// (+ - * % ). Everything is `int` so it fits the initial codegen scope.
//
// Expected process exit code: 59

int add(int a, int b) {
	return a + b;
}

int factorial(int n) {
	if (n <= 1)
		return 1;
	return n * factorial(n - 1);
}

int sum_to(int n) {
	int total = 0;
	for (int i = 1; i <= n; i = i + 1)
		total = total + i;
	return total;
}

int max(int a, int b) {
	if (a > b)
		return a;
	else
		return b;
}

int main() {
	int a = 6;
	int b = 4;

	int s = add(a, b);	  // 10
	int f = factorial(4); // 24
	int t = sum_to(5);	  // 1+2+3+4+5 = 15
	int m = max(s, t);	  // 15

	int result = 0;
	int i = 0;
	while (i < 3) {
		result = result + m; // 15 * 3 = 45
		i = i + 1;
	}

	result = result + f - s; // 45 + 24 - 10 = 59
	return result % 256;	 // 59
}
