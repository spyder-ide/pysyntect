#include<stdio.h>
#include<stdlib.h>
#include<string.h>

const char* y[34];
typedef void (*message_handler)(char *, int *, unsigned long, my_struct *, int);

typedef struct {
    int* list;
    unsigned char** str_arr;
    long long int x;
} my_struct;

extern my_struct *str_const;

void main(void) {
    my_struct x;
    int res = func(2, "String", &x, *y);
    printf("%d %f %ull", x, u, y);
    free(x);
}
