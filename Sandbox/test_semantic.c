int a = 2, b = 5;
char short_int_var;
double wow;
int static nicer;
int static const nice;
long int long c;
int long long const static weird;

int shadowed;

int factorial(int n) {
	if (n == 0)
		return 1;
	return n * factorial(n - 1);
}

float var;

void func(double x) {
	var += 10;
	func(var);
}

int main() {
	int shadowed;
	short_int_var += 1;
	{
		char shadowed;
		do {
			long shadowed;
			while (shadowed != '0') {
				for (int i = 0; i < 10; i += 1)
					shadowed += 1;
			}
			break;
		} while (1);
	}

	for (int i = 0; i < 12; i - 1) {
		continue;
		int counter = 45;
		return 20 * counter;
	}

	int x = 2;
	int y = 4;
	int result = x * y;
	float a = 20.23f;
	char c = 'b';
	int m = 10 + a, p = 10.0f - c;
	int b = a;
	return x + y + a + c;
}
