#include <errno.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include <parser.h>

#define todo() do { printf("%s todo: line %i\n", __FUNCTION__, __LINE__); exit(1); } while (0)

int
races_build(ParserOut *out, int **times, int **records) {
  ParserOut *raw_times = parser_out_data_list_get(out->data, 0);
  raw_times = parser_out_data_list_get(raw_times->data, 1);
  ParserOut *raw_records = parser_out_data_list_get(out->data, 1);
  raw_records = parser_out_data_list_get(raw_records->data, 1);

  int num_races = parser_out_data_list_size(raw_times->data);

  *times = malloc(sizeof(int) * num_races);
  *records = malloc(sizeof(int) * num_races);

  for (int i = 0; i < num_races; i++) {
    (*times)[i] = parser_out_data_get_uint(parser_out_data_list_get(raw_times->data, i)->data);
    (*records)[i] = parser_out_data_get_uint(parser_out_data_list_get(raw_records->data, i)->data);
  }

  return num_races;
}

Parser *
line(char *prefix) {
  return parser_sequence(2,
    parser_adjacent(NULL, parser_string(prefix), parser_ws()),
    parser_take_many_1(parser_adjacent(NULL, parser_uint(), parser_ws()))
  );
}

int
parse(char *buffer, int **times, int **records) {
  Parser *p;

  p = parser_sequence(2,
    line("Time:"),
    line("Distance:")
  );

  ParserIn *in = parser_in_new(buffer);
  ParserOut *out = NULL;

  int num_races;
  if (parser_run(in, p, &out)) {
    num_races = races_build(out, times, records);
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(in);
  parser_free(p);
  parser_out_free(out);

  return num_races;
}

double
quadratic_roots_diff(long a, long b, long c) {
  double discrim = sqrt((b * b) - (4 * a * c));
  double pos = ((-1 * b) + discrim) / (2 * a);
  double neg = ((-1 * b) - discrim) / (2 * a);
  // exactly matching the existing record doesn't count, so we nudge them to account for that case
  return ceil(neg - 1) - floor(pos + 1);
}

double
part1(int num_races, int *times, int *records) {
  double result = 1;

  for (int i = 0; i < num_races; i++) {
    result *= quadratic_roots_diff(-1, times[i], -1 * records[i]) + 1;
  }

  return result;
}

double
int_concat(double a, double b) {
  return (a * pow(10, ceil(log10(b)))) + b;
}

double
part2(int num_races, int *times, int *records) {
  long time = 0;
  long record = 0;

  for (int i = 0; i < num_races; i++) {
    time = (long) int_concat(time, times[i]);
    record = (long) int_concat(record, records[i]);
  }

  return quadratic_roots_diff(-1, time, -1 * record) + 1;
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

  // TODO: return type and/or output parameter
  int *times = NULL;
  int *records = NULL;
  int num_races = parse(buffer, &times, &records);

  free(buffer);

  // TODO: proper parameter(s)
  printf("part1: %.0f\n", part1(num_races, times, records));
  printf("part2: %.0f\n", part2(num_races, times, records));
}
