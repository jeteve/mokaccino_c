
// Poorman's assert.
#define ASSERT(expr, msg) do { if (!(expr)) { fprintf(stderr, "%s\n", msg); return 1; } } while (0)
