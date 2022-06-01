#define UPPER_BOUND 1000

int for_loop_test()
{
    int total = 0;

    for (int i = 0; i < UPPER_BOUND; i++)
    {
        total += 1;
    }

    return total;
}

int while_loop_test()
{
    int total = 0;

    int i = 0;

    while (i < UPPER_BOUND)
    {
        total += i;
        i++;
    }
}
