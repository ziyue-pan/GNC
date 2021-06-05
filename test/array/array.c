int main() {
    int* a;

    for(int i =0 ;i < 10; i+= 1) {
        *(a + i) = i;
    }

    return 0;
}
