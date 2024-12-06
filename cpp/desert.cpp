#include <_stdio.h>
#include <cstdio>

#define InFile "desert.in"
#define OutFile "desert.out"

int n, k;
long double p;

long double dist(int n) {
  if (n <= 2)
    return (k * n) / p;
  return k / ((2 * n - 3) * p) + dist(n - 1);
}

int main() {
  FILE *fin, *fout;
  long double d;
  fin = fopen(InFile, "rt");
  fscanf(fin, "%d %d %Lf", &n, &k, &p);
  fclose(fin);
  p /= 100;
  d = dist(n);
  fout = fopen(OutFile, "wt");
  fprintf(fout, "%.3Lf\n", d);
  fclose(fout);
  return 0;
}
