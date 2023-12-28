#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include <parser.h>

#define todo() do { printf("%s todo: line %i\n", __FUNCTION__, __LINE__); exit(1); } while (0)

typedef struct {
  int dst;
  int src;
  int size;
} MapRange;

typedef struct {
  int num_ranges;
  MapRange *ranges;
} Mapping;

typedef struct {
  int num_seeds;
  int *seeds;
  int num_mappings;
  Mapping *mappings;
} Almanac;

Almanac *
almanac_build(ParserOut *out) {
  Almanac *a = malloc(sizeof(Almanac));

  ParserOut *seed_line = parser_out_data_list_get(out->data, 0);
  seed_line = parser_out_data_list_get(seed_line->data, 1);
  a->num_seeds = parser_out_data_list_size(seed_line->data);
  a->seeds = malloc(sizeof(int) * a->num_seeds);

  for (int i = 0; i < a->num_seeds; i++) {
    a->seeds[i] = parser_out_data_get_uint(parser_out_data_list_get(seed_line->data, i)->data);
  }

  ParserOut *map_chunks = parser_out_data_list_get(out->data, 1);
  a->num_mappings = parser_out_data_list_size(map_chunks->data);
  a->mappings = malloc(sizeof(Mapping) * a->num_mappings);

  for (int i = 0; i < a->num_mappings; i++) {
    ParserOut *chunk = parser_out_data_list_get(map_chunks->data, i);
    chunk = parser_out_data_list_get(chunk->data, 1);

    a->mappings[i].num_ranges = parser_out_data_list_size(chunk->data);
    a->mappings[i].ranges = malloc(sizeof(MapRange) * a->mappings[i].num_ranges);
    for (int j = 0; j < a->mappings[i].num_ranges; j++) {
      ParserOut *mapping_data = parser_out_data_list_get(chunk->data, j);
      a->mappings[i].ranges[j].dst = parser_out_data_get_uint(parser_out_data_list_get(mapping_data->data, 0)->data);
      a->mappings[i].ranges[j].src = parser_out_data_get_uint(parser_out_data_list_get(mapping_data->data, 1)->data);
      a->mappings[i].ranges[j].size = parser_out_data_get_uint(parser_out_data_list_get(mapping_data->data, 2)->data);
    }
  }

  return a;
}

Parser *
seed_line() {
  return parser_sequence(2,
    parser_string("seeds:"),
    parser_take_many_1(parser_adjacent(parser_opt_ws(), parser_uint(), parser_opt_ws()))
  );
}

Parser *
map() {
  return parser_sequence(2,
    parser_adjacent(NULL, parser_drop_til(parser_char(':')), parser_opt_ws()),
    parser_take_many_1(
      parser_adjacent(NULL, parser_take_N(3, parser_adjacent(parser_opt_ws(), parser_uint(), NULL)), parser_opt_ws())
    )
  );
}

Almanac *
parse(char *buffer) {
  Parser *p;

  p = parser_sequence(2, seed_line(), parser_take_many_1(map()));

  ParserIn *in = parser_in_new(buffer);
  ParserOut *out;

  Almanac *a;
  if (parser_run(in, p, &out)) {
    a = almanac_build(out);
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(in);
  parser_free(p);
  parser_out_free(out);

  return a;
}

uint64_t
part1(Almanac *almanac) {
  uint64_t min = INT_MAX;

  for (int i = 0; i < almanac->num_seeds; i++) {
    uint64_t seed = almanac->seeds[i];

    for (int j = 0; j < almanac->num_mappings; j++) {
      Mapping *m = &almanac->mappings[j];

      for (int k = 0; k < m->num_ranges; k++) {
        MapRange *r = &m->ranges[k];
        if (seed >= r->src && seed < (r->src + r->size)) {
          seed = r->dst + (seed - r->src);
          break;
        }
      }
    }

    if (seed < min) {
      min = seed;
    }
  }

  return min;
}

typedef struct RangeList {
  uint64_t start;
  uint64_t size;
  struct RangeList *next;
} RangeList;

RangeList *
range_list_build(uint64_t start, uint64_t size) {
  RangeList *new = malloc(sizeof(RangeList));
  new->start = start;
  new->size = size;
  new->next = NULL;
  return new;
}

RangeList *
range_list_shift(RangeList **rl) {
  RangeList *tmp = *rl;
  *rl = (*rl)->next;
  tmp->next = NULL;
  return tmp;
}

void
range_list_unshift(RangeList **rl, RangeList *new) {
  if (new->next != NULL) {
    range_list_unshift(rl, new->next);
  } else {
    new->next = *rl;
    *rl = new;
  }
}

bool
range_map(RangeList **rl, MapRange *m, RangeList **new_out) {
  RangeList *r = *rl;

  if (r->start >= m->src + m->size || m->src >= r->start + r->size) {
    // no overlap, do nothing
    return false;

  } else if (r->start >= m->src && r->start + r->size <= m->src + m->size) {
    // MapRange contains RangeList,
    // update its start value (size does not change)
    r->start = m->dst + (r->start - m->src);
    // remove it from the input list (since it's fully mapped) and return it
    return true;

  } else if (r->start < m->src && r->start + r->size <= m->src + m->size) {
    // RangeList overlaps MapRange start
    // create the mapped portion of the range
    *new_out = range_list_build(m->dst, (r->start + r->size) - m->src);
    // update the size of the original (start stays the same)
    r->size = m->src - r->start;
    // return the newly mapped range
    return false;

  } else if (r->start >= m->src && r->start + r->size > m->src + m->size) {
    // RangeList overlaps MapRange end
    // create the mapped portion of the range
    int overlap = m->src + m->size - r->start;
    *new_out = range_list_build(m->dst + (r->start - m->src), overlap);
    // update the original range
    r->start = r->start + overlap;
    r->size = r->size - overlap;
    // return the newly mapped range
    return false;

  } else {
    // RangeList fully contains MapRange
    // create the mapped portion of the range
    *new_out = range_list_build(m->dst, m->size);
    // create the hanging off portion of the original range, and push it on the active ranges
    range_list_unshift(rl, range_list_build(m->src + m->size, (r->start + r->size) - (m->src + m->size)));
    // update the size of the original range (start stays the same)
    r->size = m->src - r->start;
    // return the newly mapped range
    return false;
  }
}

int
part2(Almanac *almanac) {
  RangeList *ranges = NULL;

  for (int i = 0; i < almanac->num_seeds; i += 2) {
    RangeList *new = range_list_build(almanac->seeds[i], almanac->seeds[i + 1]);
    range_list_unshift(&ranges, new);
  }

  for (int i = 0; i < almanac->num_mappings; i++) {
    Mapping *m = &almanac->mappings[i];

    RangeList *remaining = ranges;
    ranges = NULL;

    while (remaining != NULL) {
      for (int j = 0; j < m->num_ranges; j++) {
        RangeList *new = NULL;

        if (range_map(&remaining, &m->ranges[j], &new)) {
          // range_map returns true if the range is fully used up. since we
          // always move the first range from `remaining` to `ranges`, we can
          // (hackily) update it in place instead of using `new` and rely on
          // the next section to do the right thing.
          break;
        }

        // if there was an overlap, add the newly mapped ranges to the `ranges` list
        if (new != NULL) range_list_unshift(&ranges, new);
      }

      // pull off any remainder of the original range (or the hacky case where
      // we moved the entire range and it's already updated) and put it into
      // `ranges` for the next round.
      RangeList *tmp = range_list_shift(&remaining);
      range_list_unshift(&ranges, tmp);
    }
  }

  int min = INT_MAX;

  while (ranges != NULL) {
    RangeList *current = range_list_shift(&ranges);

    if (current->start < min) min = current->start;

    free(current);
  }

  return min;
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

  Almanac *almanac = parse(buffer);

  free(buffer);

  printf("part1: %i\n", part1(almanac));
  printf("part2: %i\n", part2(almanac));
}
