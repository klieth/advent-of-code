#define _GNU_SOURCE
// required for qsort_r

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

#include <parser.h>

#define todo() do { printf("%s todo: line %i\n", __FUNCTION__, __LINE__); exit(1); } while (0)

typedef struct {
  char cards[5];
  int bid;
} Hand;

int
build_hands(ParserOut *out, Hand **hands) {
  int num_hands = parser_out_data_list_size(out->data);
  *hands = malloc(sizeof(Hand) * num_hands);

  for (int i = 0; i < num_hands; i++) {
    ParserOut *raw_hand = parser_out_data_list_get(out->data, i);

    ParserOut *raw_cards = parser_out_data_list_get(raw_hand->data, 0);
    for (int j = 0; j < 5; j++) {
      //(*hands)[i].cards[j] = *(char *)parser_out_data_list_get(raw_cards->data, j);
      (*hands)[i].cards[j] = *(char *)parser_out_data_list_get(raw_cards->data, j)->data;
    }

    ParserOut *raw_bid = parser_out_data_list_get(raw_hand->data, 1);
    (*hands)[i].bid = atoi(raw_bid->data);
  }

  return num_hands;
}

Parser *
parse_hand() {
  return parser_sequence(2,
    parser_take_N(5, parser_any_char()),
    parser_adjacent(parser_ws(), parser_uint(), parser_ws())
  );
}

int
parse(char *buffer, Hand **hands) {
  Parser *p;

  p = parser_take_many_1(parse_hand());

  ParserIn *in = parser_in_new(buffer);
  ParserOut *out = NULL;

  int num_hands;
  if (parser_run(in, p, &out)) {
    num_hands = build_hands(out, hands);
  } else {
    parser_print_error(out);
    exit(1);
  }

  parser_in_free(in);
  parser_free(p);
  parser_out_free(out);

  return num_hands;
}

typedef enum {
  FIVE_OAK,
  FOUR_OAK,
  FULL_HOUSE,
  THREE_OAK,
  TWO_PAIR,
  ONE_PAIR,
  HIGH_CARD,
} Rank;

typedef struct {
  Rank rank;
  Hand *hand;
} RankedHand;

int
card_value(char card, bool use_jokers) {
  switch (card) {
    case '2': case '3': case '4': case '5': case '6': case '7': case '8': case '9':
      return card - '0';
    case 'T':
      return 10;
    case 'J':
      if (use_jokers) {
        return 0;
      } else {
        return 11;
      }
    case 'Q':
      return 12;
    case 'K':
      return 13;
    case 'A':
      return 14;
  }
}

Rank
hand_rank(Hand *hand, bool use_jokers) {
  int counts[15] = {0};
  for (int i = 0; i < 5; i++) {
    int value = card_value(hand->cards[i], use_jokers);
    counts[value] += 1;
  }

  int top = 0;
  int second = 0;

  // 0 index is always for jokers, we never want to count that so we start at 1
  // here on purpose.
  for (int i = 1; i < 15; i++) {
    if (counts[i] > top) {
      second = top;
      top = counts[i];
    } else if (counts[i] > second) {
      second = counts[i];
    }
  }

  int jokers = counts[0];

  // top group 5
  if (top == 5 || jokers == 5) return FIVE_OAK;
  // top group 4
  else if (top == 4 && jokers == 1) return FIVE_OAK;
  else if (top == 4 && second == 1) return FOUR_OAK;
  // top group 3
  else if (top == 3 && jokers == 2) return FIVE_OAK;
  else if (top == 3 && second == 2) return FULL_HOUSE;
  else if (top == 3 && second == 1 && jokers == 1) return FOUR_OAK;
  else if (top == 3 && second == 1 && jokers == 0) return THREE_OAK;
  // top group 2
  else if (top == 2 && jokers == 3) return FIVE_OAK;
  else if (top == 2 && second == 2 && jokers == 1) return FULL_HOUSE;
  else if (top == 2 && second == 2 && jokers == 0) return TWO_PAIR;
  else if (top == 2 && second == 1 && jokers == 2) return FOUR_OAK;
  else if (top == 2 && second == 1 && jokers == 1) return THREE_OAK;
  else if (top == 2 && second == 1 && jokers == 0) return ONE_PAIR;
  // top group 1
  else if (top == 1 && jokers == 4) return FIVE_OAK;
  else if (top == 1 && second == 1 && jokers == 3) return FOUR_OAK;
  else if (top == 1 && second == 1 && jokers == 2) return THREE_OAK;
  else if (top == 1 && second == 1 && jokers == 1) return ONE_PAIR;
  else if (top == 1 && second == 1 && jokers == 0) return HIGH_CARD;
  else {
    printf("unreachable: line %i\n", __LINE__);
    exit(1);
  }
}

static int
cmp_ranked_hand(const void *a, const void *b, void *state) {
  RankedHand *rh1 = (RankedHand *)a;
  RankedHand *rh2 = (RankedHand *)b;
  bool use_jokers = *(bool *)state;

  // The enum is defined with the highest rank first, so this is on purpose
  // opposite the logic you might expect.
  if (rh1->rank > rh2->rank) {
    return -1;
  } else if (rh1->rank < rh2->rank) {
    return 1;
  }

  for (int i = 0; i < 5; i++) {
    int cv1 = card_value(rh1->hand->cards[i], use_jokers);
    int cv2 = card_value(rh2->hand->cards[i], use_jokers);
    if (cv1 > cv2) {
      return 1;
    } else if (cv1 < cv2) {
      return -1;
    }
  }

  return 0;
}

int
run(int num_hands, Hand *hands, bool use_jokers) {
  // rank the hands
  RankedHand *ranks = malloc(sizeof(RankedHand) * num_hands);

  for (int i = 0; i < num_hands; i++) {
    ranks[i].hand = &hands[i];
    ranks[i].rank = hand_rank(&hands[i], use_jokers);
  }

  // sort the hands
  qsort_r(ranks, num_hands, sizeof(RankedHand), cmp_ranked_hand, &use_jokers);

  // calculate the result
  int sum = 0;

  for (int i = 0; i < num_hands; i++) {
    sum += ranks[i].hand->bid * (i + 1);
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

  Hand *hands = NULL;
  int num_hands = parse(buffer, &hands);

  free(buffer);

  printf("part1: %i\n", run(num_hands, hands, false));
  printf("part2: %i\n", run(num_hands, hands, true));
}
