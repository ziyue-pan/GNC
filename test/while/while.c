int main() {
    int a = 0;
    int b = 30;

    while (a < b) {
        a += 1;
        b -= 1;

        if (b == 3)
            break;
    }

    return 0;
}