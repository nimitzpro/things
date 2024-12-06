#include <iostream>
#include <malloc/_malloc.h>
#include <ostream>


void func(int* s) {
    int x = 5;

    std::cout << &x << ", " << x << std::endl;
    s = &x;

    std::cout << s << ", " << *s << std::endl;
}

int main (int argc, char *argv[]) {

  int* r;
  std::cout << r << std::endl;

  func(r);
  
  /* { */
  /*   int x = 5; */
  /*   r = &x; */
  /* } */

  /* std::cout << x << std::endl; */


  int y = *r;
  std::cout << r << ", "<< *r << std::endl;


  std::cout << &y << ", " << y << std::endl;

  return 0;
}
