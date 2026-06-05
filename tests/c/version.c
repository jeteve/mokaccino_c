#include <stdio.h>
#include "mokaccino.h"

#include "macros.h"

int main(void) {
    printf("Mokaccino version: %s\n", mokaccino_version());

    ASSERT(true == true, "True wasnt true");

    return 0;
}