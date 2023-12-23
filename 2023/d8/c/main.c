#include <errno.h>
#include <inttypes.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <parser.h>

#define todo() do { printf("%s todo: line %i\n", __FUNCTION__, __LINE__); exit(1); } while (0)

typedef struct {
  char *name;
  char *left;
  char *right;
} Node;

typedef struct {
  int num_directions;
  char *directions;
  int num_nodes;
  Node *nodes; // must be sorted
} Map;

char *
build_node_name(ParserOut *raw) {
  char *name = malloc(sizeof(char) * 4);

  for (int i = 0; i < 3; i++) {
    name[i] = *(char *)parser_out_data_list_get(raw->data, i)->data;
  }
  name[3] = '\0';

  return name;
}

int
node_cmp(const void *n1, const void *n2) {
  return strcmp( ((Node *)n1)->name, ((Node *)n2)->name);
}

int
build_nodes(ParserOut *raw, Node **nodes) {
  int num_nodes = parser_out_data_list_size(raw->data);
  *nodes = malloc(sizeof(Node) * num_nodes);

  for (int i = 0; i < num_nodes; i++) {
    ParserOut *raw_node = parser_out_data_list_get(raw->data, i);

    (*nodes)[i].name = build_node_name(parser_out_data_list_get(raw_node->data, 0));
    (*nodes)[i].left = build_node_name(parser_out_data_list_get(raw_node->data, 1));
    (*nodes)[i].right = build_node_name(parser_out_data_list_get(raw_node->data, 2));
  }

  qsort(*nodes, num_nodes, sizeof(Node), node_cmp);

  return num_nodes;
}

Map *
build_map(ParserOut *out) {
  Map *m = malloc(sizeof(Map));

  ParserOut *raw_direction_list = parser_out_data_list_get(out->data, 0);
  ParserOut *raw_nodes_list = parser_out_data_list_get(out->data, 1);

  m->num_directions = parser_out_data_list_size(raw_direction_list->data);
  m->directions = malloc(sizeof(char) * m->num_directions);

  for (int i = 0; i < m->num_directions; i++) {
    ParserOut *raw_direction = parser_out_data_list_get(raw_direction_list->data, i);
    m->directions[i] = *(char *)raw_direction->data;
  }

  m->num_nodes = build_nodes(raw_nodes_list, &m->nodes);

  return m;
}

Parser *
parse_direction_line(void) {
  return parser_adjacent(NULL, parser_take_many_1(parser_first_of(2, parser_char('L'), parser_char('R'))), parser_ws());
}

Parser *
parse_nodes(void) {
  Parser *node_id = parser_take_N(3, parser_any_char());
  return parser_take_many_1(parser_sequence(3,
    node_id,
    parser_adjacent(parser_string(" = ("), parser_ref(node_id), NULL),
    parser_adjacent(parser_string(", "), parser_ref(node_id), parser_sequence(2, parser_string(")"), parser_opt_ws()))
  ));
}

Map *
parse(char *buffer) {
  Parser *p;

  p = parser_sequence(2, parse_direction_line(), parse_nodes());

  ParserIn *in = parser_in_new(buffer);
  ParserOut *out = NULL;

  Map *m;
  if (parser_run(in, p, &out)) {
    m = build_map(out);
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(in);
  parser_free(p);
  parser_out_free(out);

  return m;
}

int
wander(Map *m, char *start) {
  int steps = 0;
  Node dummy;
  char *curr = start;

  for (int i = 0; curr[2] != 'Z'; i = (i + 1) % m->num_directions) {
    dummy.name = curr;
    Node *n = bsearch(&dummy, m->nodes, m->num_nodes, sizeof(Node), node_cmp);
    switch (m->directions[i]) {
      case 'L':
        curr = n->left;
        break;
      case 'R':
        curr = n->right;
        break;
      default:
        printf("unrecognized direction: %s\n", m->directions[i]);
        exit(1);
    }
    steps++;
  }

  return steps;
}

int
part1(Map *m) {
  return wander(m, "AAA");
}

uint64_t
gcd(uint64_t a, uint64_t b) {
  if (a == 0) return b;
  return gcd(b % a, a);
}

uint64_t
part2(Map *m) {
  uint64_t lcm = 1;

  for (int i = 0; i < m->num_nodes; i++) {
    if (m->nodes[i].name[2] == 'A') {
      uint64_t steps = (uint64_t)wander(m, m->nodes[i].name);
      lcm = (lcm * steps) / gcd(lcm, steps);
    }
  }

  return lcm;
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
  Map *m = parse(buffer);

  free(buffer);

  // TODO: proper parameter(s)
  printf("part1: %i\n", part1(m));
  printf("part2: %" PRIu64 "\n", part2(m));
}
