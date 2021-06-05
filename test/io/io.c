//#include<stdio.h>

int main() {
    int a;
    int b;

    scanf("%d %d", &a, &b);
    
    if (a > b) {
        printf("a is bigger than b");
    } else if(a < b) {
        printf("a is smaller than b");
    } else {
        printf("a is eqaul to b");
    }

    return 0;
}
