// A minimal C program that leaks memory.
#include <stdio.h>
#include <stdlib.h>
#include <strings.h>

int main(void) {
char *leaked = malloc(1024 * sizeof(char));

    strcpy(leaked, "This memory will be leaked");
    printf("%s\n", leaked);

    leaked = NULL;

    return 0;
}