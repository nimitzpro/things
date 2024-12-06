#include <iostream>
#include <malloc/_malloc.h>
#include <ostream>


struct Monke {
  int a;
  float b;
};

static Monke* r;

void func() {
    struct Monke x = { 5, 3.14f };

    std::cout << "Struct variable in func(): " << &x << ", " << x.a << std::endl;
    r = &x;

    std::cout << "static pointer r in func(): " << r << ", " << r->a << std::endl;
}

int main (int argc, char *argv[]) {

  std::cout << "r on init: " << r << std::endl;

  func();
  
  /* { */
  /*   int x = 5; */
  /*   r = &x; */
  /* } */

  /* std::cout << x << std::endl; */


  /* int x = *r; */
  std::cout << "r after exiting func(): " << r  << ", " << r->a << std::endl;

  struct Monke y = *r;

  std::cout << "new struct y = *r: " << &y << ", " << y.a << std::endl;


  /* std::cout << x << ", " << &x << std::endl; */

  return 0;
}
