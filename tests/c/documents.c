#include <stdio.h>
#include <stdlib.h>
#include "mokaccino.h"
#include "macros.h"


int main(void) {
    printf("Mokaccino queries test with version: %s\n", mokaccino_version());

    // Test error case: passing NULL pointer
    ASSERT(mokaccino_d_new(NULL) == MOKACCINO_ERROR, "ERROR expected MOKACCINO_ERROR when passing NULL to mokaccino_d_new");

    Document* d = NULL;

    ASSERT(mokaccino_d_add_value(&d, "field", "value") == MOKACCINO_ERROR, "mokaccino_d_add_value should return MOKACCINO_ERROR when passed a **Document pointing to a NULL *Document");

    ASSERT(mokaccino_d_new(&d) != MOKACCINO_ERROR, "ERROR cannot create correct document");

    ASSERT(mokaccino_d_add_value(NULL, "field", "value") == MOKACCINO_ERROR, "ERROR add_value with NULL double pointer did not return error");

    ASSERT(mokaccino_d_add_value(&d, "field", "value") != MOKACCINO_ERROR, "ERROR cannot add value to document");

    // Test null arguments
    ASSERT(mokaccino_d_add_value(&d, NULL, "value") == MOKACCINO_ERROR, "ERROR expected MOKACCINO_ERROR for NULL field");

    ASSERT(mokaccino_d_add_value(&d, "field", NULL) == MOKACCINO_ERROR, "ERROR expected MOKACCINO_ERROR for NULL value");

    ASSERT(mokaccino_d_add_value(&d, NULL, NULL) == MOKACCINO_ERROR, "ERROR expected MOKACCINO_ERROR for NULL field and value");

    char* buffer_f = calloc(32, sizeof(char));
    char* buffer_v = calloc(32, sizeof(char));

    snprintf(buffer_f, 32, "field2");
    snprintf(buffer_v, 32, "value2");

    ASSERT(mokaccino_d_add_value(&d, buffer_f, buffer_v) != MOKACCINO_ERROR, "ERROR cannot add value to document");

    // It's safe to free those buffers. Copy of the strings are now owned
    // by the document.
    free(buffer_f);
    free(buffer_v);

    // Invalid UTF8 bytes for value
    // "\xe2\x28\xa1"
    ASSERT(mokaccino_d_add_value(&d, "field", "\xe2\x28\xa1") == MOKACCINO_ERROR, "ERROR added invalid UTF8 value to document");

    // Invalid UTF8 bytes for field
    ASSERT(mokaccino_d_add_value(&d, "\xe2\x28\xa1", "value") == MOKACCINO_ERROR, "ERROR added invalid UTF8 field to document");

    char* debug = mokaccino_d_debug(d);
    printf("Document: %s\n", debug);

    mokaccino_d_free(&d);
    mokaccino_string_free(&debug);

    // All good.
    return 0;

}
