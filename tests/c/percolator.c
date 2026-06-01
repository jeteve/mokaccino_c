#include <stdio.h>
#include <stdlib.h>
#include "mokaccino.h"


int main(void) {
    printf("Mokaccino percolator test with version: %s\n", mokaccino_version());

    Percolator* p = NULL;
    if ( mokaccino_p_new(&p) == MOKACCINO_ERROR ){
        printf("ERROR cannot create correct percolator\n");
        return 1;
    }

    mokaccino_p_free(&p);

    // All good.

    return 0;

}