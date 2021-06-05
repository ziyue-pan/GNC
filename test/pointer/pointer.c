int main() {

    int * a;
    int b = 0x256;

    a= (int*) b;

    int * addr = &b;

    *a = 3;


    return *addr;
}
