export const ExampleOptions = [
    {
        id: 1,
        name: 'Basic',
        code: `int main() {
    return 0;
}`
    },
    {
        id: 2,
        name: 'Unary',
        code: `int neg_unary() {
    int a = -2;
    return -a;
}

int not_unary() {
    int a = 1;
    return ~a;
}

int complement_unary() {
    int b = !0;
    int a = !23;
    return !b;
}
`
    },
    {
        id: 3,
        name: 'Binary',
        code: `int add() {
    int a = 2 + 2;
    return a;
}

int mul() {
    int a = 1 + 3 * 4 - 2;
    return a + 1;
}

int div() {
    int b = 2;
    int a = 3 / b;
    return b;
}`
    },
    {
        id: 4,
        name: 'If Condition',
        code: `int main() {
    int a = 0;
    int b = 1;

    if (a > b) {
        return 1;
    } else if (a < b) {
        return 2;
    } else {
        return 3;
    }
}`
    },
    {
        id: 5,
        name: 'For Loop',
        code: `int main() {
    int b = 255;

    for (int i = 0; i < b; i += 1) {
        b -= 1;
    }
    return 0;
}
`
    },
    {
        id: 6,
        name: 'While Loop',
        code: `int main() {
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
}`
    },
    {
        id: 7,
        name: 'Functions',
        code: `
int foo(int a, int b) {
    return a + b;
}

int main() {
    return foo(0, 1);
}

int bar(int a, int b, int c) {
    int rst = 0;
    while (a < b) {
        rst <<= c;
        a += 1;
    }
    return rst;
}
`
    },
    {
        id: 8,
        name: 'Global Variables',
        code: `int main() {
    a = 3;
    return a;
}

int a  = 0;

int foo() {
    return 0;
}
`
    },
    {
        id: 9,
        name: 'Type System',
        code: `int main() {
    bool a = false;
    byte b = 0o3;
    short c = 0x3;
    int d = 0b1011;
    long f = 0xcafebabe;

    float g = 0.5;
    double h = 0.25;

    return a;
}`
    },
    {
        id: 10,
        name: 'Cast Expression',
        code: `int cast() {

    double a = 10.5;

    int b = (int) a;

    return b;
}`
    },
    {
        id: 11,
        name: 'String & Output',
        code: `int main() {
    char* s = "String Test";

    printf("%s", s);

    return 0;
}`
    }
]
