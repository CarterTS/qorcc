#define TEST

#ifdef TEST
int a;
#else
int b;
#endif

#undef TEST

#ifdef TEST
int c;
#else
int d;
#endif