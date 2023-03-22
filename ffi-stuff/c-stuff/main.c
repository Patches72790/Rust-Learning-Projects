#include "c_stuff.h"
#include <stdio.h>
#include <stdlib.h>

typedef struct Thing {
  int *stuff;
} Thing;

int main() {
  Thing *t = malloc(sizeof(Thing *));
  int x = 42;
  t->stuff = &x;

  Thing t2;
  t2.stuff = t->stuff;

  call_in_c(*(*t).stuff);
  printf("Hello world: %p\n", t);

  printf("Hello world: %p\n", &t2);
}
