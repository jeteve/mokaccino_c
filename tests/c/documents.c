#include <stdio.h>
#include "mokaccino.h"


int main(void) {
    printf("Mokaccino queries test with version: %s\n", mokaccino_version());

    Document* d = NULL;
    
    if ( mokaccino_d_new(&d) == MOKACCINO_ERROR ){
        printf("ERROR cannot create correct document\n");
        return 1;
    }

    if ( mokaccino_d_add_value(&d, "field", "value") == MOKACCINO_ERROR ){
        printf("ERROR cannot add value to document\n");
        return 1;
    }

    if ( mokaccino_d_add_value(&d, "field2", "value2") == MOKACCINO_ERROR ){
        printf("ERROR cannot add value to document\n");
        return 1;
    }

    char* debug = mokaccino_d_debug(d);
    printf("Document: %s\n", debug);


    mokaccino_d_free(&d);
    mokaccino_string_free(&debug);


    // All good.
    return 0;

}