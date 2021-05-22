int main() {
    int a = 0;
    int b = 30;
    int c = 10;

    do {
        c = b - c;
    } while (c >= 0);

    while (a < b) {
        a += 1;
        b -= 1;

        if (b == 3)
            break;
    }

    return 0;
}