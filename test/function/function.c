

int _foo(int a, int b) {
    return a + b;
}


int main() {
    return _foo(0, 1);
}



int bar(int a, int b, int c) {
    int rst = 0;
    while (a < b) {
        rst <<= c;
        a += 1;
    }
    return rst;
}

