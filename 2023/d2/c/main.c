#include <errno.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <parser.h>

// === GAME PARSING & BUILDING ===

typedef struct {
  int red;
  int green;
  int blue;
} Handful;

Handful *
handful_build(ParserOut *raw_handful) {
  Handful *h = malloc(sizeof(Handful));

  int num_colors = parser_out_data_list_size(raw_handful->data);
  for (int i = 0; i < num_colors; i++) {
    ParserOut *set = parser_out_data_list_get(raw_handful->data, i);

    int amt = atoi(parser_out_data_list_get(set->data, 0)->data);
    char *color = parser_out_data_list_get(set->data, 1)->data;

    if (strcmp(color, "red") == 0) {
      h->red = amt;
    } else if (strcmp(color, "green") == 0) {
      h->green = amt;
    } else if (strcmp(color, "blue") == 0) {
      h->blue = amt;
    } else {
      printf("unrecognized color: %s\n", color);
      exit(1);
    }
  }

  return h;
}

int
handful_build_many(ParserOut *raw_handfuls, Handful ***handfuls_out) {
  int num_handfuls = parser_out_data_list_size(raw_handfuls->data);
  *handfuls_out = malloc(sizeof(Handful *) * num_handfuls);

  for (int i = 0; i < num_handfuls; i++) {
    (*handfuls_out)[i] = handful_build(parser_out_data_list_get(raw_handfuls->data, i));
  }

  return num_handfuls;
}

typedef struct {
  int id;
  int num_handfuls;
  Handful **handfuls;
} Game;

int
game_build_many(ParserOut *out, Game ***games_out) {
  int num_games = parser_out_data_list_size(out->data);
  *games_out = malloc(sizeof(Game *) * num_games);

  for (int i = 0; i < num_games; i++) {
    (*games_out)[i] = malloc(sizeof(Game));

    ParserOut *game_ast = parser_out_data_list_get(out->data, i);

    char *game_id_str = parser_out_data_list_get(game_ast->data, 1)->data;
    (*games_out)[i]->id = atoi(game_id_str);

    ParserOut *handfuls = parser_out_data_list_get(game_ast->data, 3);
    (*games_out)[i]->num_handfuls = handful_build_many(handfuls, &(*games_out)[i]->handfuls);
  }

  return num_games;
}

void dbg(ParserIn *in) {
  printf("dbg %s\n", in->mark.str);
}

Parser *
handful(void) {
  return parser_sep_by(
      parser_char(','),
      parser_sequence(2,
        parser_adjacent(parser_opt_ws(), parser_uint(), NULL),
        parser_adjacent(parser_opt_ws(), parser_first_of(3, parser_string("red"), parser_string("green"), parser_string("blue")), parser_opt_ws())
      )
  );
}

Parser *
handfuls(void) {
  return parser_sep_by(parser_char(';'), handful());
}

int
parse(char *raw, Game ***games_out) {
  Parser *p;

  p = parser_sequence(4,
      parser_string("Game"),
      parser_adjacent(parser_opt_ws(), parser_uint(), NULL),
      parser_char(':'),
      parser_adjacent(parser_opt_ws(), handfuls(), parser_opt_ws()));
  p = parser_take_many_til_1(p, parser_end_of_input());

  ParserIn *i = parser_in_new(raw);
  ParserOut *out = NULL;

  int num_games;
  if (parser_run(i, p, &out)) {
    num_games = game_build_many(out, games_out);
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(i);
  parser_free(p);

  return num_games;
}

// === PART 1 ===

int
part1(int num_games, Game *games[]) {
  int sum = 0;

  for (int i = 0; i < num_games; i++) {
    Game *game = games[i];
    bool game_possible = true;

    for (int j = 0; game_possible && j < game->num_handfuls; j++) {
      Handful *handful = game->handfuls[j];

      if (handful->red > 12 || handful->green > 13 || handful->blue > 14) {
        game_possible = false;
      }
    }

    if (game_possible) {
      sum += game->id;
    }
  }

  return sum;
}

// === PART 2 ===

int
part2(int num_games, Game *games[]) {
  int sum = 0;

  for (int i = 0; i < num_games; i++) {
    Game *game = games[i];
    int max_red = 0;
    int max_green = 0;
    int max_blue = 0;

    for (int j = 0; j < game->num_handfuls; j++) {
      Handful *handful = game->handfuls[j];

      if (handful->red > max_red) {
        max_red = handful->red;
      }

      if (handful->green > max_green) {
        max_green = handful->green;
      }

      if (handful->blue > max_blue) {
        max_blue = handful->blue;
      }
    }

    sum += max_red * max_green * max_blue;
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

  Game **games;
  int num_games = parse(buffer, &games);

  free(buffer);

  printf("part 1: %i\n", part1(num_games, games));
  printf("part 2: %i\n", part2(num_games, games));
}
