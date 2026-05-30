#include <stdio.h>
#include "mokaccino.h"

int main(void) {
    printf("Mokaccino queries test with version: %s\n", mokaccino_version());

    // Pathological:
    if( mokaccino_q_term(NULL, "field", "value") != -1 ){
        printf("mokaccino_q_term should return -1\n");
        return 1;
    }

    Query* q = NULL;
    if( mokaccino_q_term(&q, NULL, "value") != -1 ){
        printf("mokaccino_q_term should return -1\n");
        return 1;
    }

    if( mokaccino_q_term(&q, "field", NULL) != -1 ){
        printf("mokaccino_q_term should return -1\n");
        return 1;
    }

    // Invalid UTF8 bytes
    // "\xe2\x28\xa1"
    if( mokaccino_q_term(&q, "field", "\xe2\x28\xa1") != -1 ){
        printf("mokaccino_q_term should return -1\n");
        return 1;
    }


    // Invalid UTF8 bytes
    // "\xe2\x28\xa1"
    if( mokaccino_q_term(&q, "\xe2\x28\xa1", "value") != -1 ){
        printf("mokaccino_q_term should return -1\n");
        return 1;
    }



    mokaccino_q_term(&q, "field", "value");
    char* debug = mokaccino_q_debug(q);

    printf("Query: %s\n", debug);
    mokaccino_string_free(&debug);


    // Negate it.
    mokaccino_q_negation(&q);

    debug = mokaccino_q_debug(q);
    printf("Query: %s\n", debug);
    mokaccino_string_free(&debug);

    mokaccino_q_free(&q);

    if( q != NULL ){
        printf("ERROR: Q is not NULL");
        return 1;
    }


    // Test prefix query
    if( mokaccino_q_prefix(&q, "field", "value") != 0 ){
        printf("mokaccino_q_prefix should return 0\n");
        return 1;
    }

    if( q == NULL ){
        printf("ERROR: Q is NULL");
        return 1;
    }
    mokaccino_q_free(&q);


    // All good.
    return 0;

}