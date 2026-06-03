#include <stdio.h>
#include <stdlib.h>
#include "mokaccino.h"


// 1. State to capture the matches. (max 128)
typedef struct {
    uint32_t matches[128];
    size_t count;
} MatchResults;

// a function to capture a matching query id
void on_match(uint32_t id, void* user_data) {
    MatchResults* results = (MatchResults*)user_data;
    if (results->count >= 128 ){
        // Result outbuffer is too small. Do something or just
        return;
    }

    results->matches[results->count++] = id;
}

int main(void) {
    printf("Mokaccino percolator test with version: %s\n", mokaccino_version());

    Percolator* p = NULL;
    if ( mokaccino_p_new(&p) == MOKACCINO_ERROR ){
        printf("ERROR cannot create correct percolator\n");
        return 1;
    }

    // Check we cannot overwrite an existing percolator
    if ( mokaccino_p_new(&p) != MOKACCINO_ERROR ){
        printf("ERROR mokaccino_p_new allowed overwriting an existing percolator\n");
        return 1;
    }

    // Now build a query
    Query* q = NULL;
    if( mokaccino_q_term(&q, "field", "value") == MOKACCINO_ERROR ){
        printf("ERROR cannot build query\n");
        return 1;
    }

    // Check we get an error if percolator is NULL
    if ( mokaccino_p_index_id(NULL, &q, 42) != MOKACCINO_ERROR ){
        printf("ERROR expected MOKACCINO_ERROR for NULL percolator\n");
        return 1;
    }

    // And index in the percolator under the number 42
    if( mokaccino_p_index_id(p, &q, 42) == MOKACCINO_ERROR ){
        printf("ERROR cannot index in percolator\n");
        return 1;
    }

    // Check *q is now NULL
    if( q != NULL ){
        printf("ERROR: Q is not NULL");
        return 1;
    }

    // Since *q is now NULL, indexing it again should fail with MOKACCINO_ERROR
    if( mokaccino_p_index_id(p, &q, 42) != MOKACCINO_ERROR ){
        printf("ERROR: indexing with null q should fail\n");
        return 1;
    }

    // Build a second query
    mokaccino_q_prefix(&q, "field", "val");
    // Index it
    mokaccino_p_index_id(p, &q, 43);

    // Test null document error
    MatchResults null_test_results = { .count = 0 };
    if (mokaccino_p_percolate(p, NULL, on_match, &null_test_results) != MOKACCINO_ERROR) {
        printf("ERROR: Expected MOKACCINO_ERROR when percolating with NULL document\n");
        return 1;
    }

    // Time to percolate a document.
    Document* d = NULL;
    mokaccino_d_new(&d);
    mokaccino_d_add_value(&d, "field", "value");

    MatchResults results = { .count = 0 };
    mokaccino_p_percolate(p, d, on_match, &results);
    // Do NOT forget to free the document:
    mokaccino_d_free(&d);

    // There should be Two matches.
    if ( results.count != 2 ){
        printf("ERROR: Missing some matches");
        return 1;
    }

    // Now another document with 'valuation'.
    // Will match the prefix query, but not the pure term query.
    mokaccino_d_new(&d);
    mokaccino_d_add_value(&d, "field", "valuation");
    // Test that passing a NULL percolator fails properly
    if (mokaccino_p_percolate(NULL, d, on_match, &results) != MOKACCINO_ERROR) {
        printf("ERROR: mokaccino_p_percolate should return MOKACCINO_ERROR when passed a NULL percolator\n");
        return 1;
    }

    // Reset the result:
    results.count = 0;
    mokaccino_p_percolate(p, d, on_match, &results);
    mokaccino_d_free(&d);

    // There should be One matches.
    if ( results.count != 1 ){
        printf("ERROR: Missing some matches");
        return 1;
    }

    mokaccino_p_free(&p);

    // All good.

    return 0;

}