#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define todo() do { printf("%s todo: line %i\n", __FUNCTION__, __LINE__); exit(1); } while (0)
#include <parser.h>

typedef struct {
  int value;
  int length;
  int x;
  int y;
} Number;

typedef struct {
  char ch;
  int x;
  int y;
} Symbol;

typedef struct {
  int num_numbers;
  Number *numbers;

  int num_symbols;
  Symbol *symbols;
} Schematic;

Schematic *
schematic_build(ParserOut *out) {
  int num_numbers = 0;
  int num_symbols = 0;

  int num_entries = parser_out_data_list_size(out->data);
  for (int i = 0; i < num_entries; i++) {
    ParserOut *entry = parser_out_data_list_get(out->data, i);

    switch (entry->type) {
      case PARSER_OUT_INT:
        num_numbers++;
        break;
      case PARSER_OUT_CHAR:
        num_symbols++;
        break;
      case PARSER_OUT_LIST:
      case PARSER_OUT_NO_DATA:
        break;
      default:
        todo();
    }
  }

  Schematic *schematic = malloc(sizeof(Schematic));
  schematic->num_numbers = num_numbers;
  schematic->numbers = malloc(sizeof(Number) * num_numbers);
  schematic->num_symbols = num_symbols;
  schematic->symbols = malloc(sizeof(Symbol) * num_symbols);

  num_numbers = 0;
  num_symbols = 0;

  for (int i = 0; i < num_entries; i++) {
    ParserOut *entry = parser_out_data_list_get(out->data, i);

    switch (entry->type) {
      case PARSER_OUT_INT:
        Number *n = &(schematic->numbers[num_numbers++]);
        n->value = parser_out_data_get_uint(entry->data);
        n->length = strlen(parser_out_data_get_int_raw(entry->data));
        n->x = entry->col;
        n->y = entry->line;
        break;
      case PARSER_OUT_CHAR:
        Symbol *s = &(schematic->symbols[num_symbols++]);
        s->ch = *(char *)entry->data;
        s->x = entry->col;
        s->y = entry->line;
        break;
      case PARSER_OUT_LIST:
      case PARSER_OUT_NO_DATA:
        break;
      default:
        todo();
    }
  }

  return schematic;
}

Schematic *
parse(char *buffer) {
  Parser *p;

  p = parser_take_many_til_1(
        parser_first_of(4,
          parser_uint(),
          parser_take_many_1(parser_char('.')),
          parser_whitespace(),
          parser_any_char()
        ),
        parser_end_of_input()
      );

  ParserIn *in = parser_in_new(buffer);
  ParserOut *out = NULL;

  Schematic *schematic = NULL;
  if (parser_run(in, p, &out)) {
    schematic = schematic_build(out);
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(in);
  parser_free(p);
  parser_out_free(out);

  return schematic;
}

bool
adjacent(Number *n, Symbol *s) {
  bool x_adj = (n->x - 1 <= s->x) && (s->x <= n->x + n->length);
  bool y_adj = (n->y - 1 <= s->y) && (s->y <= n->y + 1);
  return x_adj && y_adj;
}

int
part1(Schematic *s) {
  int sum = 0;

  for (int i = 0; i < s->num_numbers; i++) {
    for (int j = 0; j < s->num_symbols; j++) {
      if (adjacent(&s->numbers[i], &s->symbols[j])) {
        sum += s->numbers[i].value;
        break;
      }
    }
  }

  return sum;
}

int
gear_ratio(Schematic *schematic, Symbol *symbol) {
  int first = 0;
  int second = 0;

  for (int n = 0; n < schematic->num_numbers; n++) {
    Number *number = &(schematic->numbers[n]);
    if (!adjacent(number, symbol)) continue;

    if (first == 0) {
      first = number->value;
    } else if (second == 0) {
      second = number->value;
    } else {
      // Too many numbers, not a gear.
      return 0;
    }
  }

  // if we found 0 or only 1 adjacent part number(s), this will multiply to
  // zero and so we don't have to explicitly check for that case
  return first * second;
}

int
part2(Schematic *schematic) {
  int sum = 0;

  for (int s = 0; s < schematic->num_symbols; s++) {
    Symbol *symbol = &(schematic->symbols[s]);
    if (symbol->ch != '*') continue;

    sum += gear_ratio(schematic, symbol);
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

  Schematic *s = parse(buffer);

  free(buffer);

  // TODO: run part1 and part2
  printf("part1: %i\n", part1(s));
  printf("part2: %i\n", part2(s));
}
