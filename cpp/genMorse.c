#include <assert.h>
#include <errno.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void genMorse(char l[], int i, int length) {
  if (i == length - 1) {
    /* printf("null terminator\n"); */
    l[i] = '\0';
    puts(l);
  } else {
    l[i] = '.';
    genMorse(l, i + 1, length);
    l[i] = '-';
    genMorse(l, i + 1, length);
    /* printf("exec number: %d\n", i); */
  }
}

int main(int argc, char *argv[]) {
  int length;
  printf("Enter length of combo: ");
  scanf("%d", &length);
  char l[length];
  printf("length: %d\n", length);
  genMorse(l, 0, length);
  /* printf("%s", morse); */
  return EXIT_SUCCESS;
}
