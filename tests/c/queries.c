#include <stdio.h>
#include "mokaccino.h"
#include "macros.h"

int main(void) {
    printf("Mokaccino queries test with version: %s\n", mokaccino_version());

    // Pathological:
    ASSERT(mokaccino_q_term(NULL, "field", "value") == MOKACCINO_ERROR, "mokaccino_q_term should return MOKACCINO_ERROR");

    Query* q = NULL;
    ASSERT(mokaccino_q_term(&q, NULL, "value") == MOKACCINO_ERROR, "mokaccino_q_term should return MOKACCINO_ERROR");

    ASSERT(mokaccino_q_term(&q, "field", NULL) == MOKACCINO_ERROR, "mokaccino_q_term should return MOKACCINO_ERROR");

    // Invalid UTF8 bytes
    // "\xe2\x28\xa1"
    ASSERT(mokaccino_q_term(&q, "field", "\xe2\x28\xa1") == MOKACCINO_ERROR, "mokaccino_q_term should return MOKACCINO_ERROR");

    // Invalid UTF8 bytes
    // "\xe2\x28\xa1"
    ASSERT(mokaccino_q_term(&q, "\xe2\x28\xa1", "value") == MOKACCINO_ERROR, "mokaccino_q_term should return MOKACCINO_ERROR");

    mokaccino_q_term(&q, "field", "value");
    char* debug = mokaccino_q_debug(q);

    printf("Query: %s\n", debug);

    // Try building a new q before freeing it.
    ASSERT(mokaccino_q_term(&q, "field2", "value2") != 0, "mokaccino_q_term should return MOKACCINO_ERROR when building on top of an unfreed Query*");

    mokaccino_string_free(&debug);

    Query* null_q = NULL;
    ASSERT(mokaccino_q_negation(NULL) == MOKACCINO_ERROR, "mokaccino_q_negation with NULL should return MOKACCINO_ERROR");

    ASSERT(mokaccino_q_negation(&null_q) == MOKACCINO_ERROR, "mokaccino_q_negation with NULL *Query should return MOKACCINO_ERROR");

    // Negate it.
    mokaccino_q_negation(&q);

    debug = mokaccino_q_debug(q);
    printf("Query: %s\n", debug);
    mokaccino_string_free(&debug);

    mokaccino_q_free(&q);

    ASSERT(q == NULL, "ERROR: Q is not NULL");

    // Test H3IN query
    ASSERT(mokaccino_q_h3in(&q, "location", "81197ffffffffff") == 0, "Failed to build a h3in query");
    debug = mokaccino_q_debug(q);
    printf("Query: %s\n", debug);
    mokaccino_string_free(&debug);
    mokaccino_q_free(&q);

    // Test prefix query
    ASSERT(mokaccino_q_prefix(&q, "field", "value") == 0, "mokaccino_q_prefix should return 0");
    ASSERT(q != NULL, "ERROR: Q is NULL");

    // Test NULL double pointer errors
    ASSERT(mokaccino_q_and(NULL, &q) == MOKACCINO_ERROR, "mokaccino_q_and should return MOKACCINO_ERROR for NULL q1");
    ASSERT(mokaccino_q_and(&q, NULL) == MOKACCINO_ERROR, "mokaccino_q_and should return MOKACCINO_ERROR for NULL q2");
    ASSERT(mokaccino_q_or(NULL, &q) == MOKACCINO_ERROR, "mokaccino_q_or should return MOKACCINO_ERROR for NULL q1");
    ASSERT(mokaccino_q_or(&q, NULL) == MOKACCINO_ERROR, "mokaccino_q_or should return MOKACCINO_ERROR for NULL q2");
    ASSERT(q != NULL, "ERROR: Q is NULL");

    // Make another query and do an and.
    Query* q2 = NULL;
    mokaccino_q_term(&q2, "field2", "value2");

    // Save q for comparison.
    Query* pre_and_q = q;
    mokaccino_q_and(&q, &q2);

    // Check q2 is now NULL
    ASSERT(q2 == NULL, "ERROR: Q2 is not NULL");

    ASSERT(q != pre_and_q, "ERROR: Q has not changed");

    // Build another new q2
    mokaccino_q_term(&q2, "field3", "value3");

    // Now do an OR with q.
    mokaccino_q_or(&q, &q2);
    debug = mokaccino_q_tostring(q);
    printf("Query after OR: %s\n", debug);
    mokaccino_string_free(&debug);

    mokaccino_q_free(&q);
    mokaccino_q_free(&q2);

    // Test NULL dereference in build_two_qs (mokaccino_q_and / mokaccino_q_or)
    Query* q_null = NULL;
    Query* q_valid = NULL;
    mokaccino_q_term(&q_valid, "field", "value");

    ASSERT(mokaccino_q_and(&q_null, &q_valid) == MOKACCINO_ERROR, "mokaccino_q_and should return MOKACCINO_ERROR for pointer to NULL");

    ASSERT(mokaccino_q_or(&q_valid, &q_null) == MOKACCINO_ERROR, "mokaccino_q_or should return MOKACCINO_ERROR for pointer to NULL");

    ASSERT(mokaccino_q_and(NULL, &q_valid) == MOKACCINO_ERROR, "mokaccino_q_and should return MOKACCINO_ERROR for NULL");

    // Test Pointer aliasing / double free protection
    ASSERT(mokaccino_q_and(&q_valid, &q_valid) == MOKACCINO_ERROR, "mokaccino_q_and should return MOKACCINO_ERROR for aliased pointers to avoid double-free");

    mokaccino_q_free(&q_valid);

    // All good.
    return 0;

}
