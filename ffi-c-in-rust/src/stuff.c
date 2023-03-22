#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct Thing {
  int stuff;
} Thing;

void *gimme_a_thing(int32_t size) {
  printf("Hello from C!\n");

  Thing *ptr = malloc(sizeof(Thing *));

  ptr->stuff = 69;

  return ptr;
}
