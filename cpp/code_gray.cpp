#include <fstream>
#include <iostream>

#define NMax 10
#define NrMaxS (1 << NMax)

int n, C[NrMaxS][NMax];

void display();
void genCode(int);

int main() {
  std::cout << "n = ";
  std::cin >> n;
  genCode(n);
  display();
  return 0;
}

void display() {
  std::ofstream f("Gray.out");
  for (int i = 0; i < 1 << n; i++) {
    for (int j = 0; j < n; j++) {
      f << C[i][j];
    }
    f << std::endl;
  }
  f.close();
}

void genCode(int x) { // x = 3
  if (x == 1) {
    C[1][0] = 1; // 1 0*n-1
  } else {
    genCode(x - 1); // x = 2
    for (int i = (1 << (x - 1)) - 1; i >= 0; i--) {
      // 2^(x-1) - 1 > i >= 0
      for (int j = 0; j < x - 1; j++) { // 0 < j < x - 1
        C[(1 << x) - (i + 1)][j] = C[i][j];
        // C[2^(x) - (i+1)][j] = C[i][j]
        // copy C[i][j] to C[2^x - i +1, since indices start from 0)][j]
        // copying down value, increment and continue for row
      }
      C[(1 << x) - (i + 1)][x - 1] = 1;
      // C[2^x - i][x-1] = 1, assign the last value to 1 for all i
    }
  }
}
