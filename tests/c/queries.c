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


    // Negate it.
    mokaccino_q_negation(&q);

    mokaccino_q_free(&q);

    

    if( q != NULL ){
        printf("Q is not NULL");
        return 1;
    }

    // All good.
    return 0;

}