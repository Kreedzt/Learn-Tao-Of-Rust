#include "fn.h"

void c_fn(int n) {
  printf("This is c fn printf!, %d\n", n);
  fflush(stdout);
}

int c_add(int a, int b) {
  return a + b;
}
