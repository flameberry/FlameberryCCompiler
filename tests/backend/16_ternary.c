// expect: 23
// Case 16: ternary conditional operator ?:. Exercises a taken then-branch, a
// taken else-branch, a nested ternary, and — via assignment side effects — the
// requirement that ONLY the selected branch is evaluated.

int main() {
	int x = 5;

	int a = x > 3 ? 10 : 20;              // cond true  -> 10
	int b = x > 3 ? (x < 2 ? 1 : 2) : 3;  // outer true -> inner false -> 2
	int c = x == 0 ? 100 : 7;             // cond false -> 7

	// only the taken branch may run
	int y = 0;
	int z = 0;
	x > 0 ? (y = 4) : (z = 9);            // y = 4, z stays 0

	return a + b + c + y + z; // 10 + 2 + 7 + 4 + 0 = 23
}
