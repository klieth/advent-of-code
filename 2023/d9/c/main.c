#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

#include <parser.h>

#define todo() do { printf("%s todo: line %i\n", __FUNCTION__, __LINE__); exit(1); } while (0)

typedef struct {
  int length;
  int *data;
} List;

int
build_lists(ParserOut *out, List **lists) {
  int num_lists = parser_out_data_list_size(out->data);
  *lists = malloc(sizeof(List) * num_lists);

  for (int i = 0; i < num_lists; i++) {
    ParserOut *raw_list = parser_out_data_list_get(out->data, i);
    int list_length = parser_out_data_list_size(raw_list->data);
    (*lists)[i].length = list_length;
    (*lists)[i].data = malloc(sizeof(int) * list_length);

    for (int j = 0; j < list_length; j++) {
      ParserOut *raw_num = parser_out_data_list_get(raw_list->data, j);
      (*lists)[i].data[j] = parser_out_data_get_int(raw_num->data);
    }
  }

  return num_lists;
}

int
parse(char *buffer, List **lists) {
  Parser *p;

  // single line
  p = parser_take_many_1(parser_adjacent(NULL, parser_int(), parser_optional(parser_char(' '))));
  // many lines
  p = parser_take_many_1(parser_adjacent(NULL, p, parser_char('\n')));

  ParserIn *in = parser_in_new(buffer);
  ParserOut *out = NULL;

  int num_lists;
  if (parser_run(in, p, &out)) {
    num_lists = build_lists(out, lists);
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(in);
  parser_free(p);
  parser_out_free(out);

  return num_lists;
}

int
add_last(int length, int *list, int acc) {
  return acc + list[length - 1];
}

int
sub_first(int _length, int *list, int acc) {
  return list[0] - acc;
}

int
step(int length, int *list, int (*op)(int, int *, int)) {
  int *diff_list = malloc(sizeof(int) * (length - 1));

  int acc = list[0];
  bool all_zero = true;
  for (int i = 1; i < length; i++) {
    int new = list[i] - acc;
    if (new != 0) all_zero = false;
    diff_list[i - 1] = new;
    acc = list[i];
  }

  int ret = all_zero ? list[length - 1] : op(length, list, step(length - 1, diff_list, op));;

  free(diff_list);

  return ret;
}

int
run(int num_lists, List *lists, int (*op)(int, int *, int)) {
  int sum = 0;

  for (int i = 0; i < num_lists; i++) {
    sum += step(lists[i].length, lists[i].data, op);
  }

  return sum;
}

int
main(int argc, char *argv[]) {
  if (argc < 2) {
    printf("no filename specified\n");
    exit(1);
  }

  FILE *f = fopen(argv[1], "r");
  if (f == NULL) {
    if (errno == 2) {
      printf("specified file does not exist: %s\n", argv[1]);
    } else {
      printf("Error: %i\n", errno);
    }
    exit(1);
  }

  fseek(f, 0, SEEK_END);
  int size = ftell(f);
  rewind(f);

  char *buffer = malloc(size);
  if (!buffer) {
    printf("failed to create file read buffer\n");
    exit(1);
  }

  fread(buffer, 1, size, f);

  fclose(f);

  List *lists = NULL;
  int count = parse(buffer, &lists);

  free(buffer);

  printf("part1: %i\n", run(count, lists, add_last));
  printf("part2: %i\n", run(count, lists, sub_first));
}
