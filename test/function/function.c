
int _foo(int a, int b) {
    return a + b;
}


int main() {
    int a = _foo(0, 0);
    return a;
}


int bar(int a, int b, int c) {
    int rst = 0;
    while (a < b) {
        rst <<= c;
        a += 1;
    }
    return rst;
}

