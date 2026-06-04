// A minimal C program that leaks memory.
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    void *leaked_memory = malloc(1024);
    return 0;
}