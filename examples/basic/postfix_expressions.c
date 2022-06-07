int test0(int a)
{
    return a++;
}

int test1(int a)
{
    return a--;
}

int test2(int* a)
{
    return a[5];
}

int test3(int a)
{
    return test0(5);
}

int test4(int* a)
{
    return a[5]++;
}