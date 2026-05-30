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

    // Make another query and do an and.
    Query* q2 = NULL;
    mokaccino_q_term(&q2, "field2", "value2");

    // Save q for comparison.
    Query* pre_and_q = q;
    mokaccino_q_and(&q, &q2);

    // Check q2 is now NULL
    if( q2 != NULL ){
        printf("ERROR: Q2 is not NULL");
        return 1;
    }

    if( q == pre_and_q ){
        printf("ERROR: Q has not changed");
        return 1;
    }

    // Build another new q2
    mokaccino_q_term(&q2, "field3", "value3");

    // Now do an OR with q.
    mokaccino_q_or(&q, &q2);
    debug = mokaccino_q_tostring(q);
    printf("Query after OR: %s\n", debug);
    mokaccino_string_free(&debug);


    mokaccino_q_free(&q);
    mokaccino_q_free(&q2);


    // All good.
    return 0;

}