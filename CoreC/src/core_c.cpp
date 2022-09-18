#include "core_c.h"
#include <iostream>

using namespace std;

int add(int a, int b) {
    cerr << "C -- a: " << a << " b: " << b << endl;
    return a + b;
}
