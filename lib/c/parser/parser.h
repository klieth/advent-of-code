#ifndef PARSER_H
#define PARSER_H

#include <stdbool.h>


// === Parser ===

typedef enum {
  PARSER_ADJACENT,
  PARSER_CHARS,
  PARSER_DEBUG,
  PARSER_FIRST_OF,
  PARSER_SEP_BY,
  PARSER_SEQUENCE,
  PARSER_STRING,
  PARSER_TAKE_MANY,
  PARSER_UINT, // TODO: generic number parser implementation?
  PARSER_WHITESPACE,
} ParserType;

typedef struct {
  ParserType type;
  void *data;
} Parser;

// parser_new is not intended to be called by implementers; use the parser builders below
void parser_free(Parser *p);

// === ParserOut ===

typedef enum {
  // *_ERROR is actually just a *_STRING, but used as a marker to denote an error.
  PARSER_OUT_ERROR,

  PARSER_OUT_INT,
  PARSER_OUT_LIST,
  PARSER_OUT_STRING,
  PARSER_OUT_WHITESPACE,
} ParserOutType;

typedef struct ParserOut {
  ParserOutType type;
  void *data;
} ParserOut;

void parser_out_free(ParserOut *);

void parser_print_error(ParserOut *);


// === ParserOutDataList ===

typedef struct ParserOutDataList ParserOutDataList;

int parser_out_data_list_size(ParserOutDataList *);
ParserOut *parser_out_data_list_get(ParserOutDataList *, int);



// === ParserIn ===

typedef struct {
  char *str;
} ParserInMark;

typedef struct {
  char *str;
  ParserInMark mark;
} ParserIn;

// does not take ownership of the string input, caller is responsible for
// freeing after parsing is complete.
ParserIn * parser_in_new(char *str);
void parser_in_free(ParserIn *);



// === PARSER BUILDERS ===

Parser *parser_debug(void (*)(ParserIn *));
Parser *parser_whitespace(void);
Parser *parser_char(char);
Parser *parser_string(char *);
Parser *parser_uint(void);
Parser *parser_adjacent(Parser *before, Parser *, Parser *after);
Parser *parser_sequence(int count, ...);
Parser *parser_first_of(int count, ...);
Parser *parser_take_many_1(Parser *);
Parser *parser_sep_by(Parser *separator, Parser *);



// === RUN ===

bool parser_run(ParserIn *, Parser *, ParserOut **);



#endif // PARSER_H
