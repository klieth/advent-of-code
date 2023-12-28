#ifndef PARSER_H
#define PARSER_H

#include <stdbool.h>


// === Parser ===

typedef enum {
  PARSER_ADJACENT,
  PARSER_ANY_CHAR,
  PARSER_CHARS,
  PARSER_DEBUG,
  PARSER_END_OF_INPUT,
  PARSER_FIRST_OF,
  PARSER_INT,
  PARSER_OPTIONAL,
  PARSER_REF,
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

  PARSER_OUT_CHAR,
  PARSER_OUT_INT,
  PARSER_OUT_LIST,
  PARSER_OUT_STRING,
  PARSER_OUT_NO_DATA,
} ParserOutType;

typedef struct ParserOut {
  ParserOutType type;
  void *data;
  int line;
  int col;
} ParserOut;

void parser_out_free(ParserOut *);

void parser_print_error(ParserOut *);


// === ParserOutDataList ===

typedef struct ParserOutDataList ParserOutDataList;

int parser_out_data_list_size(ParserOutDataList *);
ParserOut *parser_out_data_list_get(ParserOutDataList *, int);


// === ParserOutDataInt ===

typedef struct ParserOutDataInt ParserOutDataInt;

char *parser_out_data_get_int_raw(ParserOutDataInt *);
unsigned int parser_out_data_get_uint(ParserOutDataInt *);
int parser_out_data_get_int(ParserOutDataInt *);



// === ParserIn ===

typedef struct {
  char *str;
  int line;
  int col;
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

#define parser_ws() parser_whitespace()
#define parser_opt_ws() parser_optional(parser_whitespace())

// A parser can only be owned in one location, so any other references to this
// pointer must be wrapped in a parser_ref()
Parser *parser_ref(Parser *);

Parser *parser_debug(void (*)(ParserIn *));
Parser *parser_whitespace(void);
Parser *parser_any_char(void);
Parser *parser_char(char);
Parser *parser_end_of_input(void);
Parser *parser_string(char *);
Parser *parser_uint(void);
Parser *parser_int(void);
Parser *parser_adjacent(Parser *before, Parser *, Parser *after);
Parser *parser_sequence(int count, ...);
Parser *parser_optional(Parser *);
Parser *parser_first_of(int count, ...);
Parser *parser_take_many_1(Parser *);
Parser *parser_take_many_til_1(Parser *, Parser *til);
Parser *parser_take_N(int, Parser *);
Parser *parser_drop_til(Parser *);
Parser *parser_sep_by(Parser *separator, Parser *);



// === RUN ===

bool parser_run(ParserIn *, Parser *, ParserOut **);



#endif // PARSER_H
