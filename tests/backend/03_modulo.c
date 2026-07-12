// expect: 5
// Case 3: modulo. 17 % 5 + 23 % 4 = 2 + 3 = 5.
// `%` needs the msub-after-sdiv idiom on AArch64 (no native mod).

int main() {
	return 17 % 5 + 23 % 4;
}
