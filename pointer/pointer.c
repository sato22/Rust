#include<stdio.h>

int main( void )
{
	int *p;
	int k = 12345;
	int t;

	p = &k;
	t = *p;

	printf( "p=%p, t=%d\n", p, t );

	return 0;
}