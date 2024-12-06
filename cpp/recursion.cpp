#include <array>
#include <cmath>
#include <iostream>
#include <string>

using namespace std;

unsigned long
associative(unsigned n) { // Calculates number of different ways a series of
                          // terminals x1...x_n can be multiplied associatively,
                          // e.g. x1*x2*x3 = (x1*x2)*x3 or x1*(x2*x3) -> 2 ways
  long S = 0;
  unsigned k;
  if (n <= 2)
    return 1;
  for (k = 1; k < n; k++)
    S += associative(k) * associative(n - k);

  return S;
}

unsigned long associative_iter(unsigned n) {
  if (n <= 2)
    return 1;
  return 0; // solve
}

unsigned long gcd(unsigned a, unsigned b) { return b == 0 ? a : gcd(b, a % b); }

unsigned long ackermann(unsigned x, unsigned y) {
  if (!x)
    return y + 1;
  if (!y)
    return ackermann(x - 1, 1);
  return ackermann(x - 1, ackermann(x, y - 1));
}

double rootPowersSum(double b, double c, unsigned long n) {
  // Calculate the sum of the power of the roots of x^2 + bx + c = 0
  // i.e. given n <- 2, x1^2 + x2^2 = rootPowersSum(), n <- 4, x1^4 + x2^4 =
  // rootPowersSum()
  if (!n)
    return 2;
  if (n == 1)
    return b;
  return (-b) * rootPowersSum(-b, c, n - 1) - c * rootPowersSum(-b, c, n - 2);
}

array<double, 2> quadratic(double a, double b, double c) {
  double x1 = (-b + pow((b * b - 4 * a * c), 0.5f)) / (2 * a);
  double x2 = (-b - pow((b * b - 4 * a * c), 0.5f)) / (2 * a);

  return array<double, 2>{x1, x2};
}

int anagram(char *x,
            char *y) { // Sus code, SEE anagram.c for correct C-style function
  char *p;
  if (!*x && !*y)
    return 1;
  p = strchr(y, *x);
  if (!p)
    return 0;
  *p = 0;
  strcat(y, p + 1);
  return anagram(x + 1, y);
}

bool anagram_modern(string x, string y) {
  if (x.length() != y.length())
    return false;

  for (int i = 0; i < x.length(); i++) {
    int p = y.find(x[i]);
    if (p == string::npos)
      return false;
    y[p] = '\0';
    /* y.erase(p, 1); */ // alt
    cout << "erased " << p << " from " << y << endl;
  }
  return true; // y should be empty since both are equal length
}

/*
char *genMorse(int i, int n) { // SEE genMorse.c for C-style function
  if (i == n) {
    char x = '\0';
    char *pX = &x;
    cout << "added null terminator" << endl;
    return pX;
  } else {
    char dot[n - i] = {'.'};
    char *pDot = &dot;
        strcat(dot, genMorse(i + 1, n));
        cout << "execution number: " << i << endl;
        return dot;
        strcat('_', genMorse(i + 1, n));
      }
    }
*/

void genMorseModern(int i, int n, string str = "") {
  if (i == n) {
    cout << str << endl;
  } else {
    string str1 = str + "-";
    string str2 = str + ".";
    genMorseModern(i + 1, n, str1);
    genMorseModern(i + 1, n, str2);
  }
}

void convert_simple(int x, int b) {
  if (x) {
    convert_simple(x / b, b);
    /* cout << "here, x=" << x << endl; */
    cout << x % b;
  }
}

int convert(int x, int b, int c = 0) {
  if (x) {
    return convert(x / b, b, c + 1) + ((x % b) * pow(10, c));
  }
  return 0;
}

int main(int argc, char *argv[]) {
  /* cout << associative(3) << endl; */
  /* cout << gcd(200, 30) << endl; */

  /* unsigned x, y; */
  /* cout << "x: ", cin >> x; */
  /* cout << "y: ", cin >> y; */
  /* cout << ackermann(x, y) << endl; */

  /* double b, c; */
  /* unsigned long n; */
  /* cout << "b: ", cin >> s; */
  /* cout << "c: ", cin >> p; */
  /* cout << "n: ", cin >> n; */
  /* cout << rootPowersSum(b, c, n) << endl; */

  /* double a, b, c; */
  /* cout << "a: ", cin >> a; */
  /* cout << "b: ", cin >> b; */
  /* cout << "c: ", cin >> c; */
  /* array<double, 2> x = quadratic(a, b, c); */
  /* cout << x[0] << ", " << x[1] << endl; */

  /* string x, y; */
  /* cin >> x; */ // get string input
  /* cin >> y; */
  /* char* char_arr = s.data(); // type conversion from string to character
   * array */

  /* char x[100], y[100]; */
  /* cin.getline(x, 100); */ // get char* input
  /* cin.getline(y, 100); */
  /* const char *s = "Hello, World!"; */ // type conversion from char* to string
  /* std::string str(s); */

  /* string x = "restful"; */
  /* string y = "fluster"; */
  /* if (anagram(x.data(), y.data())) { */
  /*   cout << "poggers" << endl; */
  /* } else */
  /*   cout << "cringe" << endl; */

  /* char x[] = "restful"; */
  /* char y[] = "esrftul"; */
  /* printf("%d\n", anagram(x, y)); */

  /* if (anagram_modern(x, y)) */
  /*   cout << "poggers" << endl; */
  /* else */
  /*   cout << "cring" << endl; */

  /* char *morse = genMorse(0, 20); */
  /* cout << *morse << morse[0] << morse[1] << morse[2] << endl; */
  /* cout << morse[0] << morse[1] << morse[2] << endl; */

  /* genMorseModern(0, 5); */

  convert_simple(10, 2);
  cout << endl;

  int a = 256;
  int b = convert(a, 2);
  cout << b << endl;

  return 0;
}
