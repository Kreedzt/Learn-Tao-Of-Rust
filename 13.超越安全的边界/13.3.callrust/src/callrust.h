#include <inttypes.h>
#include <stdio.h>

// 13-64
void print_hello_from_rust();


// 13-70
uint32_t hm_chars(const char *str);


// 13-74
char* batman_song(uint8_t length);
void free_song(char*);


// 13-78
// 使用了 size_t, 需要引入 <stdio.h>
uint32_t sum_of_even(const uint32_t *numbers_arr, size_t length);


// 13-81
typedef struct {
  uint32_t x;
  uint32_t y;
} tuple_t;
tuple_t flip_things_around(tuple_t);


// 13-85
// 不透明数据类型
typedef struct databse_S database_t;
database_t* database_new();
void database_free(database_t*);
void database_insert(database_t *);
uint32_t database_query(const database_t* p, const char* zip);
