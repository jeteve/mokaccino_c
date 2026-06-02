#include <stdio.h>
#include <stdlib.h>
#include "mokaccino.h"


int main(void) {
    printf("Mokaccino queries test with version: %s\n", mokaccino_version());

    // Test error case: passing NULL pointer
    if ( mokaccino_d_new(NULL) != MOKACCINO_ERROR ) {
        printf("ERROR expected MOKACCINO_ERROR when passing NULL to mokaccino_d_new\n");
        return 1;
    }

    Document* d = NULL;
    
    if ( mokaccino_d_new(&d) == MOKACCINO_ERROR ){
        printf("ERROR cannot create correct document\n");
        return 1;
    }

    if ( mokaccino_d_add_value(&d, "field", "value") == MOKACCINO_ERROR ){
        printf("ERROR cannot add value to document\n");
        return 1;
    }

    char* buffer_f = calloc(32, sizeof(char));
    char* buffer_v = calloc(32, sizeof(char));
    
    snprintf(buffer_f, 32, "field2");
    snprintf(buffer_v, 32, "value2");

    if ( mokaccino_d_add_value(&d, buffer_f, buffer_v) == MOKACCINO_ERROR ){
        printf("ERROR cannot add value to document\n");
        return 1;
    }

    // It's safe to free those buffers. Copy of the strings are now owned
    // by the document.
    free(buffer_f);
    free(buffer_v);

    // Invalid UTF8 bytes for value
    // "\xe2\x28\xa1"
    if ( mokaccino_d_add_value(&d, "field", "\xe2\x28\xa1") != MOKACCINO_ERROR ){
        printf("ERROR added invalid UTF8 value to document\n");
        return 1;
    }

    // Invalid UTF8 bytes for field
    if ( mokaccino_d_add_value(&d, "\xe2\x28\xa1", "value") != MOKACCINO_ERROR ){
        printf("ERROR added invalid UTF8 field to document\n");
        return 1;
    }

    char* debug = mokaccino_d_debug(d);
    printf("Document: %s\n", debug);


    mokaccino_d_free(&d);
    mokaccino_string_free(&debug);


    // All good.
    return 0;

}