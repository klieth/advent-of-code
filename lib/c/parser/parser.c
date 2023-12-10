#include "parser.h"

#include <stdarg.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

Parser *
parser_new(ParserType type) {
  Parser *p = malloc(sizeof(Parser));
  p->type = type;
  return p;
}

typedef struct ParserDataAdjacent ParserDataAdjacent;
void parser_data_adjacent_free(ParserDataAdjacent *);

typedef struct ParserDataParserList ParserDataParserList;
void parser_data_parser_list_free(ParserDataParserList *);

typedef struct ParserDataTakeMany ParserDataTakeMany;
void parser_data_take_many_free(ParserDataTakeMany *);

typedef struct ParserDataSepBy ParserDataSepBy;
void parser_data_sep_by_free(ParserDataSepBy *);

void
parser_free(Parser *p) {
  if (!p) return;

  switch (p->type) {
    case PARSER_WHITESPACE:
    case PARSER_UINT:
      break;
    case PARSER_ADJACENT:
      parser_data_adjacent_free(p->data);
      break;
    case PARSER_CHARS:
    case PARSER_STRING:
      free(p->data);
      break;
    case PARSER_SEQUENCE:
    case PARSER_FIRST_OF:
      parser_data_parser_list_free(p->data);
      break;
    case PARSER_TAKE_MANY:
      parser_data_take_many_free(p->data);
      break;
    case PARSER_SEP_BY:
      parser_data_sep_by_free(p->data);
      break;
    default:
      printf("parser_free unrecognized type: %i\n", p->type);
      exit(1);
  }

  free(p);
}

// === PARSER IN ===

ParserIn *
parser_in_new(char *str) {
  ParserIn *i = malloc(sizeof(ParserIn));
  i->str = str;
  i->mark.str = str;
  return i;
}

void
parser_in_free(ParserIn *in) {
  // we don't take ownership of in->str so don't deallocate
  // in->mark.str is required to point at the same block of memory as in->str, so we don't touch it.
  free(in);
}

char
parser_in_peek(ParserIn *in) {
  return *(in->mark.str);
}

char
parser_in_next(ParserIn *in) {
  char c = parser_in_peek(in);
  in->mark.str++;
  return c;
}

void
parser_in_skip(ParserIn *in, int count) {
  in->mark.str += count;
}

void
parser_in_take(ParserIn *in, int count, char *out) {
  memcpy(out, in->mark.str, count);
  parser_in_skip(in, count);
}

ParserInMark
parser_in_mark(ParserIn *in) {
  return in->mark;
}

void
parser_in_rewind(ParserIn *in, ParserInMark *pim) {
  // TODO: assert that the specified mark was generated from this ParserIn.
  in->mark.str = pim->str;
}

// === PARSER OUT ===

struct ParserOutDataList {
  int size;
  int capacity;
  ParserOut **data;
};

ParserOutDataList *
parser_out_data_list_new() {
  ParserOutDataList *pod = malloc(sizeof(ParserOutDataList));

  pod->size = 0;
  pod->capacity = 5;
  pod->data = malloc(sizeof(ParserOut *) * pod->capacity);

  return pod;
}

void
parser_out_data_list_free(ParserOutDataList *podl) {
  if (!podl) return;

  for (int i = 0; i < podl->size; i++) {
    parser_out_free(podl->data[i]);
  }

  free(podl->data);
  free(podl);
}

void
parser_out_data_list_append(ParserOutDataList *list, ParserOut *new) {
  if (list->size + 1 > list->capacity) {
    int new_capacity = list->capacity + 5;
    // increase by a constant number of slots; we don't expect to be using
    // large lists and this should be plenty to have a fairly average number of
    // reallocations without using too much extra memory
    ParserOut **re = realloc(list->data, sizeof(ParserOut *) * new_capacity);

    if (!re) {
      printf("failed to increase size of ParserOutDataList\n");
      exit(1);
    }

    list->data = re;
    list->capacity = new_capacity;
  }

  list->data[list->size] = new;
  list->size++;
}

int
parser_out_data_list_size(ParserOutDataList *list) {
  return list->size;
}

ParserOut *
parser_out_data_list_get(ParserOutDataList *list, int index) {
  if (index >= list->size) {
    printf("parser_out_data_list_get attempted to read index '%i', beyond end of list '%i'\n", index, list->size);
    exit(1);
  }

  return list->data[index];
}

ParserOut *
parser_out_new_take_data(ParserOutType type, void *data) {
  ParserOut *out = malloc(sizeof(ParserOut));
  out->type = type;
  out->data = data;
  return out;
}

ParserOut *
parser_out_new_copy_data(ParserOutType type, int size, void *data) {
  void *copy = malloc(size);
  memcpy(copy, data, size);
  return parser_out_new_take_data(type, copy);
}

ParserOut *
parser_out_new(ParserOutType type) {
  return parser_out_new_take_data(type, NULL);
}

const int PARSER_OUT_ERRBUF_MAX = 256;

ParserOut *
parser_out_error(char *fmt, ...) {
  char buf[PARSER_OUT_ERRBUF_MAX];

  va_list args;
  va_start(args, fmt);
  vsnprintf(buf, PARSER_OUT_ERRBUF_MAX, fmt, args);
  va_end(args);

  return parser_out_new_copy_data(PARSER_OUT_ERROR, sizeof(char) * strlen(buf) + 1, buf);
}

void
parser_out_free(ParserOut *out) {
  switch (out->type) {
    case PARSER_OUT_WHITESPACE:
      // data must be set to NULL, so there's nothing to free.
      break;
    case PARSER_OUT_LIST:
      parser_out_data_list_free(out->data);
      break;
    case PARSER_OUT_INT:
    case PARSER_OUT_ERROR:
    case PARSER_OUT_STRING:
      free(out->data);
      break;
    default:
      printf("parser_out_free unrecognized type: %i\n", out->type);
      exit(1);
  }

  free(out);
}

// === BUILDERS & RUNNERS ===

Parser *
parser_debug(void (*dbg)(ParserIn *)) {
  Parser *p = parser_new(PARSER_DEBUG);
  p->data = dbg;
  return p;
}

Parser *
parser_whitespace(void) {
  return parser_new(PARSER_WHITESPACE);
}

bool
parser_run_whitespace(ParserIn *in, ParserOut **out) {
  char n;

  while (true) {
    n = parser_in_peek(in);

    // TODO what else counts for whitespace?
    if (n == ' ' || n == '\n') {
      parser_in_skip(in, 1);
    } else {
      break;
    }
  }

  *out = parser_out_new(PARSER_OUT_WHITESPACE);

  return true;
}

// TODO: this should be better than copying a character into a string;
Parser *
parser_char(char c) {
  char str[] = { c, '\0' };
  return parser_string(str);
}

Parser *
parser_string(char *str) {
  Parser *p = parser_new(PARSER_STRING);

  int len = strlen(str);
  p->data = malloc(sizeof(char) * len + 1);
  strcpy(p->data, str);

  return p;
}

bool
parser_run_string(ParserIn *in, char *p, ParserOut **out) {
  char *data = p;

  while (*data) {
    char next = parser_in_next(in);

    if (*data != next) {
      *out = parser_out_error("character does not match: expected %c, got %c", *data, next);

      return false;
    }

    data++;
  }

  *out = parser_out_new_copy_data(PARSER_OUT_STRING, sizeof(char) * strlen(p) + 1, p);

  return true;
};

Parser *
parser_uint(void) {
  return parser_new(PARSER_UINT);
}

bool
parser_run_uint(ParserIn *in, ParserOut **out) {
  ParserInMark mark = parser_in_mark(in);

  char data = parser_in_peek(in);
  int count = 0;

  while (data >= '0' && data <= '9') {
    count++;
    parser_in_next(in);
    data = parser_in_peek(in);
  }

  if (count == 0) {
    *out = parser_out_error("expected uint character, got %c\n", data);
    return false;
  }

  parser_in_rewind(in, &mark);

  char *str = malloc(sizeof(char) * count + 1);
  parser_in_take(in, count, str);
  str[count] = '\0';

  *out = parser_out_new_take_data(PARSER_OUT_INT, str);

  return true;
}

struct ParserDataAdjacent {
  Parser *before;
  Parser *sub;
  Parser *after;
};

void
parser_data_adjacent_free(ParserDataAdjacent *pd) {
  if (!pd) return;

  if (pd->before) parser_free(pd->before);
  parser_free(pd->sub);
  if (pd->after) parser_free(pd->after);

  free(pd);
}

Parser *
parser_adjacent(Parser *before, Parser *sub, Parser *after) {
  if (!sub) {
    printf("parser_adjacent was given a null sub-parser\n");
    exit(1);
  }

  ParserDataAdjacent *adj = malloc(sizeof(ParserDataAdjacent));
  adj->before = before;
  adj->sub = sub;
  adj->after = after;

  Parser *p = parser_new(PARSER_ADJACENT);
  p->data = adj;

  return p;
}

bool
parser_run_adjacent(ParserIn *in, ParserDataAdjacent *adj, ParserOut **out) {
  ParserOut *dispose;
  ParserOut *sub;

  if (adj->before) {
    if (parser_run(in, adj->before, &dispose)) {
      parser_out_free(dispose);
    } else {
      // TODO add context;
      *out = dispose;
      return false;
    }
  }

  if (!parser_run(in, adj->sub, &sub)) {
    // TODO add context;
    *out = sub;
    return false;
  }

  if (adj->after) {
    if (parser_run(in, adj->after, &dispose)) {
      parser_out_free(dispose);
    } else {
      // TODO add context;
      *out = dispose;
      return false;
    }
  }

  *out = sub;
  return true;
}

struct ParserDataParserList {
  int count;
  Parser **subs;
};

void
parser_data_parser_list_free(ParserDataParserList *pd) {
  if (!pd) return;

  for (int i = 0; i < pd->count; i++) {
    parser_free(pd->subs[i]);
  }

  free(pd);
}

int
parser_data_parser_list_size(ParserDataParserList *pd) {
  return pd->count;
}

Parser *
parser_data_parser_list_get(ParserDataParserList *pd, int index) {
  return pd->subs[index];
}

Parser *
parser_sequence(int count, ...) {
  Parser *p = parser_new(PARSER_SEQUENCE);
  ParserDataParserList *pd = malloc(sizeof(ParserDataParserList));
  pd->count = count;
  pd->subs = malloc(sizeof(Parser *) * count);

  va_list argp;
  va_start(argp, count);
  for (int i = 0; i < count; i++) {
    Parser *sub = va_arg(argp, Parser *);
    pd->subs[i] = sub;
  }
  va_end(argp);

  p->data = pd;

  return p;
}

bool
parser_run_sequence(ParserIn *in, ParserDataParserList *p, ParserOut **out) {
  int count = 0;
  ParserOut *next = NULL;
  ParserOutDataList *pod = parser_out_data_list_new();

  for (int i = 0; i < parser_data_parser_list_size(p); i++) {
    Parser *n = parser_data_parser_list_get(p, i);

    if (parser_run(in, n, &next)) {
      parser_out_data_list_append(pod, next);
    } else {
      // TODO add context
      *out = next;
      return false;
    }
  }

  *out = parser_out_new_take_data(PARSER_OUT_LIST, pod);

  return true;
}

Parser *
parser_first_of(int count, ...) {
  Parser *p = parser_new(PARSER_FIRST_OF);
  ParserDataParserList *pd = malloc(sizeof(ParserDataParserList));
  pd->count = count;
  pd->subs = malloc(sizeof(Parser *) * count);

  va_list argp;
  va_start(argp, count);
  for (int i = 0; i < count; i++) {
    Parser *sub = va_arg(argp, Parser *);
    pd->subs[i] = sub;
  }
  va_end(argp);

  p->data = pd;

  return p;
}

bool
parser_run_first_of(ParserIn *in, ParserDataParserList *pd, ParserOut **out) {
  for (int i = 0; i < parser_data_parser_list_size(pd); i++) {
    ParserInMark mark = parser_in_mark(in);
    Parser *n = parser_data_parser_list_get(pd, i);

    if (parser_run(in, n, out)) {
      return true;
    } else {
      parser_out_free(*out);
      parser_in_rewind(in, &mark);
    }
  }

  printf("parser_run_first_of todo error handling\n"); // TODO error handling
  exit(1);

  return false;
}

struct ParserDataTakeMany {
  int min;
  Parser *sub;
};

void
parser_data_take_many_free(ParserDataTakeMany *pd) {
  if (!pd) return;

  parser_free(pd->sub);

  free(pd);
}

Parser *
parser_take_many_1(Parser *sub) {
  Parser *p = parser_new(PARSER_TAKE_MANY);

  ParserDataTakeMany *pd = malloc(sizeof(ParserDataTakeMany));
  pd->min = 1;
  pd->sub = sub;

  p->data = pd;

  return p;
}

bool
parser_run_take_many(ParserIn *in, ParserDataTakeMany *pd, ParserOut **out) {
  int count = 0;
  ParserOut *next = NULL;
  ParserOutDataList *pod = parser_out_data_list_new();

  while (true) {
    ParserInMark pim = parser_in_mark(in);

    if (parser_run(in, pd->sub, &next)) {
      count++;
      parser_out_data_list_append(pod, next);
    } else {
      parser_in_rewind(in, &pim);
      break;
    }
  }

  if (count < pd->min) {
    printf("expected at least %i, got %i\n", pd->min, count);
    printf("parser_run_take_many todo error handling\n"); // TODO error handling
    exit(1);
  }

  *out = parser_out_new_take_data(PARSER_OUT_LIST, pod);

  return true;
}

struct ParserDataSepBy {
  Parser *sep;
  Parser *sub;
};

void
parser_data_sep_by_free(ParserDataSepBy *pd) {
  if (!pd) return;

  parser_free(pd->sep);
  parser_free(pd->sub);

  free(pd);
}

Parser *
parser_sep_by(Parser *sep, Parser *sub) {
  Parser *p = parser_new(PARSER_SEP_BY);

  ParserDataSepBy *pd = malloc(sizeof(ParserDataSepBy));
  pd->sep = sep;
  pd->sub = sub;

  p->data = pd;

  return p;
}

bool
parser_run_sep_by(ParserIn *in, ParserDataSepBy *pd, ParserOut **out) {
  ParserOut *next = NULL;
  ParserOutDataList *pod = parser_out_data_list_new();

  if (parser_run(in, pd->sub, &next)) {
    parser_out_data_list_append(pod, next);
  } else {
    // TODO add context
    *out = next;
    return false;
  }

  while (true) {
    ParserInMark pim = parser_in_mark(in);

    bool sep_found = parser_run(in, pd->sep, &next);
    parser_out_free(next); // we never care about the actual result here.

    if (!sep_found) {
      parser_in_rewind(in, &pim);
      break;
    }

    if (parser_run(in, pd->sub, &next)) {
      parser_out_data_list_append(pod, next);
    } else {
      // TODO add context
      *out = next;
      return false;
    }
  }

  *out = parser_out_new_take_data(PARSER_OUT_LIST, pod);

  return true;
}

// === RUNNING ===

bool
parser_run(ParserIn *in, Parser *p, ParserOut **out) {
  bool result;

  switch (p->type) {
    case PARSER_ADJACENT:
      result = parser_run_adjacent(in, p->data, out);
      break;
    case PARSER_FIRST_OF:
      result = parser_run_first_of(in, p->data, out);
      break;
    case PARSER_SEP_BY:
      result = parser_run_sep_by(in, p->data, out);
      break;
    case PARSER_SEQUENCE:
      result = parser_run_sequence(in, p->data, out);
      break;
    case PARSER_STRING:
      result = parser_run_string(in, p->data, out);
      break;
    case PARSER_TAKE_MANY:
      result = parser_run_take_many(in, p->data, out);
      break;
    case PARSER_UINT:
      result = parser_run_uint(in, out);
      break;
    case PARSER_WHITESPACE:
      result = parser_run_whitespace(in, out);
      break;
    default:
      printf("unimplemented parser type %i\n", p->type);
      exit(1);
      break;
  }

  return result;
}

// === ERR OPS ===

void
parser_print_error(ParserOut *o) {
  printf("parser_print_error todo\n");
  exit(1);
}
