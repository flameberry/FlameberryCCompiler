static int g_GlobalVariable;
static int g_NiceVar;
static int g_AssumeABoolean;

int main(void) {
    if (g_AssumeABoolean)
    {
        return 23 + 44;
    }
    else if (g_NiceVar)
        return 69l;
    else if (g_GlobalVariable)
        return 12ul;
    else
        return 0;

    break;
    continue;

    {}
    { return (g_GlobalVariable + 2u) * (((34l + g_NiceVar) * 34) * 4) * 23.6f - 45; }
}

inline float AdityaG() {}
const int NiceFunction(const int param1, float param2);
unsigned int GetSum(unsigned int, unsigned int);