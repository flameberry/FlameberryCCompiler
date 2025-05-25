int a = 2, b = 5;
char short_int_var;
double wow;
int static nicer;
int static const nice;
long int long c;
int long long const static weird;

int shadowed;

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
		} while (1);
	}

	for (int i = 0; i < 12; i - 1) {
		int counter = 45;
		return 20 * counter;
	}

	int x = 2;
	int y = 4;
	int result = x * y;
	float a = 20.23f;
	char c = 'b';
	int m = 10 + a, n = 10.0f - c;
	int b = a;
	return x + y + a + c;
}
