int fibb(int index)
{
    if (index < 2)
    {
        return 1;
    }
    else
    {
        return fibb(index - 1) + fibb(index - 2);
    }
}

int main()
{
    return fibb(5);
}