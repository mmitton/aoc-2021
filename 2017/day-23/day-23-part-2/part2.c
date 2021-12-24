#include <stdio.h>
int main(int argc, char** argv) {
    long long a = 1;
    long long b = 0;
    long long c = 0;
    long long d = 0;
    long long e = 0;
    long long f = 0;
    long long g = 0;
    long long h = 0;
    b = 93;
    c = b;
    b = b * 100;
    b = b - -100000;
    c = b;
    c = c - -17000;

    while (1) {
        f = 1;
        d = 2;
    
        do {
            e = 2;
            do {
                g = d;
                g = g * e;
                g = g - b;
    
                if (g == 0) {
                    f = 0;
                }
    
                e = e - -1;
                g = e;
                g = g - b;
            } while (g != 0);
    
            d = d - -1;
            g = d;
            g = g - b;
        } while (g != 0);
        if (f == 0) {
            h = h - -1;
            printf("h: %lld\n", h);
        }
        g = b;
        g = g - c;
        if (g == 0) {
            printf("h: %lld\n", h);
            return 1;
        }
    
        b = b - -17;
    }
}
