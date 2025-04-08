#include <stdio.h>
#include <unistd.h>

void main()
{
    long long i = 0;
    while (1)
    {
        printf("%lld\n", ++i);
        sleep(1);
    }
}