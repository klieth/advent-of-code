#include <errno.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include <parser.h>

#define todo() do { printf("%s todo: line %i\n", __FUNCTION__, __LINE__); exit(1); } while (0)

typedef struct {
  int num_winning;
  int *winning;
  int num_numbers;
  int *numbers;
} Card;

int
card_build_many(ParserOut *out, Card ***cards) {
  int num_cards = parser_out_data_list_size(out->data);
  *cards = malloc(sizeof(Card *) * num_cards);

  for (int i = 0; i < num_cards; i++) {
    Card *card = malloc(sizeof(Card));
    ParserOut *raw_card = parser_out_data_list_get(out->data, i);

    ParserOut *sep_by = parser_out_data_list_get(raw_card->data, 2);
    int num_seps = parser_out_data_list_size(sep_by->data);
    if (num_seps != 2) {
      printf("invalid card parsed: Card %i had '%i' separated lists\n", i + 1, num_seps);
      exit(1);
    }

    ParserOut *winning = parser_out_data_list_get(sep_by->data, 0);
    card->num_winning = parser_out_data_list_size(winning->data);
    card->winning = malloc(sizeof(int) * card->num_winning);
    for (int j = 0; j < card->num_winning; j++) {
      card->winning[j] = atoi(parser_out_data_list_get(winning->data, j)->data);
    }

    ParserOut *numbers = parser_out_data_list_get(sep_by->data, 1);
    card->num_numbers = parser_out_data_list_size(numbers->data);
    card->numbers = malloc(sizeof(int) * card->num_numbers);
    for (int j = 0; j < card->num_numbers; j++) {
      card->numbers[j] = atoi(parser_out_data_list_get(numbers->data, j)->data);
    }

    (*cards)[i] = card;
  }

  return num_cards;
}

int
parse(char *buffer, Card ***cards) {
  Parser *p;

  p = parser_sequence(3,
    parser_adjacent(NULL, parser_string("Card"), parser_ws()),
    parser_adjacent(NULL, parser_uint(), parser_char(':')),
    parser_sep_by(
      parser_adjacent(parser_opt_ws(), parser_char('|'), parser_opt_ws()),
      parser_take_many_1(parser_adjacent(parser_opt_ws(), parser_uint(), parser_opt_ws()))
    )
  );
  p = parser_take_many_1(p);

  ParserIn *in = parser_in_new(buffer);
  ParserOut *out = NULL;

  int num_cards;
  if (parser_run(in, p, &out)) {
    num_cards = card_build_many(out, cards);
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(in);
  parser_free(p);
  parser_out_free(out);

  return num_cards;
}

int num_wins(Card *card) {
  int count = 0;

  for (int i = 0; i < card->num_numbers; i++) {
    for (int j = 0; j < card->num_winning; j++) {
      if (card->numbers[i] == card->winning[j]) {
        count++;
        break;
      }
    }
  }

  return count;
}

int
part1(Card **cards, int num_cards) {
  int sum = 0;

  for (int i = 0; i < num_cards; i++) {
    Card *c = cards[i];

    int wins = num_wins(c);

    sum += pow(2, wins - 1);
  }

  return sum;
}

int
part2(Card **cards, int num_cards) {
  int *copies = malloc(sizeof(int) * num_cards);
  for (int i = 0; i < num_cards; i++) copies[i] = 1;

  int sum = 0;

  for (int i = 0; i < num_cards; i++) {
    sum += copies[i];

    Card *c = cards[i];

    int wins = num_wins(c);

    for (int j = 0; j < wins; j++) {
      copies[i + j + 1] += copies[i];
    }
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

  Card **cards;
  int num_cards = parse(buffer, &cards);

  free(buffer);

  printf("part1: %i\n", part1(cards, num_cards));
  printf("part2: %i\n", part2(cards, num_cards));
}
