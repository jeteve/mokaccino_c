#include <stdio.h>
#include "mokaccino.h"

typedef int32_t (*query_builder_fn)(Query **q, const char *field, int64_t value);

query_builder_fn builders[] = {
    mokaccino_q_klt,
    mokaccino_q_kle,
    mokaccino_q_kgt,
    mokaccino_q_kge,
    mokaccino_q_keq,
};

int main(void) {
    printf("Mokaccino queries test with version: %s\n", mokaccino_version());

    Query* q = NULL;
    char* display = NULL;

    for(int fi = 0; fi < 5 ; fi++){

        if( builders[fi](&q, "field" , 42) != 0 ){
            printf("ERROR cannot build a field/num query for function %i\n", fi);
            return 1;
        }

        display = mokaccino_q_tostring(q);
        printf("\nQuery built: %s\n", display);
        mokaccino_string_free(&display);
        mokaccino_q_free(&q);
    }

    // All good.
    return 0;

}