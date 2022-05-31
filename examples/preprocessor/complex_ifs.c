#define TEST

#ifdef TEST
int a;
#else
int b;
#endif

#ifndef TEST
int c;
#define TEST0
#else
int d;
#define TEST1
#endif

#ifdef TEST1
    #ifdef TEST0
        int e;
    #endif

    #ifndef TEST0
        int f;
    #endif
#endif

