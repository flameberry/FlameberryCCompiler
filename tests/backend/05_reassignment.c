// expect: 12
// Case 5: reassignment. x starts 10, +5 -> 15, -3 -> 12.
// Exercises `Assign` lowering (Copy into an existing slot, read-modify-write).

int main() {
	int x = 10;
	x = x + 5;
	x = x - 3;
	return x;
}
