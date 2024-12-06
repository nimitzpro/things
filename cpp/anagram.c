#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int anagram(const char *x, char *y) {
  char *p;
  if (!*x && !*y)
    return 1;
  p = strchr(y, *x);
  if (!p)
    return 0;
  memmove(p, p + 1, strlen(p));
  /* printf("%s\n", x); */
  /* printf("%s\n", y); */
  /* printf("\n"); */
  return anagram(x + 1, y);
}

int main(int argc, char *argv[]) {
  const char *x = "restful";
  char y[] = "esrftul";
  printf("%d\n", anagram(x, y));
  return EXIT_SUCCESS;
}
