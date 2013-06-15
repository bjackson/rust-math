#include <math.h>
#include <stdlib.h>
#include <stdio.h>

float sinApprox(float angle)
{
    return angle - pow(angle,3)/6 + pow(angle,5)/120 - pow(angle,7)/5040 + pow(angle,9)/362880 - \
pow(angle,11)/39916800 + pow(angle,13)/6227020800 - pow(angle,15)/1307674368000;
}

int main(int argc, char const *argv[])
{
    //printf("%f\n", sin(3.14159/4));
    printf("%f\n", sinApprox(3.14159/4));
    return 0;
}

