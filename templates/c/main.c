#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

#include <parser.h>

#define todo() do { printf("%s todo: line %i\n", __FUNCTION__, __LINE__); exit(1); } while (0)

/* TODO: return type, if not a count */
int
parse(char *buffer /* TODO: output param, if not returned */) {
  Parser *p;

  // TODO: build parser

  ParserIn *in = parser_in_new(buffer);
  ParserOut *out = NULL;

  if (parser_run(in, p, &out)) {
    // TODO: run parser
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(in);
  parser_free(p);
  parser_out_free(out);

  return 0; // TODO: output value
}

int
part1(int count) {
  todo();
}

int
part2(int count) {
  todo();
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
  int count = parse(buffer);

  free(buffer);

  // TODO: proper parameter(s)
  printf("part1: %i\n", part1(count));
  printf("part2: %i\n", part2(count));
}
