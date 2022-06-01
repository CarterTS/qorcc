long factorial(long i)
{
    if (i < 2)
    {
        return 1;
    }
    else
    {
        return i * factorial(i - 1);
    }
}