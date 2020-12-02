#include "callrust.h"
#include <stdint.h>
#include <stdio.h>
#include <inttypes.h>

int main() {
  // 13-65
  print_hello_from_rust();

  
  // 13-70
  uint32_t count = hm_chars("The taö of Rust");
  printf("%d\n", count);


  // 13-75
  char* song = batman_song(5);
  printf("%s\n", song);
  free_song(song);


  // 13-79
  uint32_t numbers[6] = { 1, 2, 3, 4, 5, 6};
  uint32_t sum = sum_of_even(numbers, 6);
  printf("%d\n", sum);


  // 13-82
  tuple_t initial = { .x = 10, .y = 20 };
  tuple_t new = flip_things_around(initial);
  printf("(%d, %d)\n", new.x, new.y);


  // 13-86
  database_t* database = database_new();
  database_insert(database);
  uint32_t pop1 = database_query(database, "10186");
  uint32_t pop2 = database_query(database, "10852");
  database_free(database);
  printf("%d\n", pop2 - pop1);
}
