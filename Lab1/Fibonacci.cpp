// Brian and Aaron
#include <iostream>

int fib(int n, int i, int x, int y, int t) {
    if (n <= 0) {
        return 0;
    }

    if (i >= n || n <= 1) {
        return x;
    }
    t = x;
    x = x + y;
    y = t;

    return fib(n, i + 1, x, y, t);
}
// For testing purposes
int main() {
    int t;

    // fibonacci sequence at 10 is 55
    std::cout << fib(10, 2, 1, 1, t) << std::endl;

    return 0;
}
