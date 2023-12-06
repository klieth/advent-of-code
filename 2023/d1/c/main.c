#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

typedef struct V Vertex;

typedef struct VM VertexMap;
void vm_free(VertexMap *);
Vertex *vm_get(VertexMap *, char);
void vm_insert(VertexMap **, char, Vertex *);
void vm_each(VertexMap *, void (*)(Vertex *));

typedef struct VQ VertexQueue;
Vertex *vq_dequeue(VertexQueue *vq);

// ===== STRUCTS =====

struct VQNode {
  struct VQNode *next;
  struct VQNode *prev;
  Vertex *value;
};

struct VQ {
  struct VQNode *head;
  struct VQNode *tail;
};

struct VM {
  VertexMap *next;
  char key;
  Vertex *value;
};

struct V {
  Vertex *parent;
  char prev_char;
  VertexMap *children;
  bool end_of_word;
  int word_value;
  Vertex *suffix_link;
  Vertex *end_link;
};

// ===== VERTEX =====

Vertex *
vertex_new(Vertex *parent, char prev_char) {
  Vertex *v = malloc(sizeof(Vertex));
  v->parent = parent;
  v->prev_char = prev_char;
  v->children = NULL;
  v->end_of_word = false;
  v->word_value = 0;
  v->suffix_link = NULL;
  v->end_link = NULL;
  return v;
}

void
vertex_free(Vertex *v) {
  vm_each(v->children, vertex_free);
  vm_free(v->children);
  free(v);
}

Vertex *
vertex_traverse_or_add(Vertex *v, char c) {
  Vertex *next;
  if ( (next = vm_get(v->children, c)) ) {
    return next;
  } else {
    next = vertex_new(v, c);
    vm_insert(&v->children, c, next);
    return next;
  }
}

// ===== VERTEX MAP =====

VertexMap *
vm_new(char key, Vertex *value) {
  VertexMap *vm = malloc(sizeof(VertexMap));
  vm->next = NULL;
  vm->key = key;
  vm->value = value;
  return vm;
}

void
vm_free(VertexMap *m) {
  if (m) {
    vm_free(m->next);
    free(m);
  }
}

void
vm_insert(VertexMap **m, char key, Vertex *value) {
  VertexMap *n = vm_new(key, value);

  if (m != NULL) {
    n->next = *m;
  }

  *m = n;
}

Vertex *
vm_get(VertexMap *m, char c) {
  if (m == NULL) {
    return NULL;
  } else if (m->key == c) {
    return m->value;
  } else {
    return vm_get(m->next, c);
  }
}

void
vm_each(VertexMap *m, void (*fn)(Vertex *)) {
  if (m) {
    fn(m->value);
    vm_each(m->next, fn);
  }
}

// ===== VERTEX QUEUE =====

struct VQNode *
vqn_new(Vertex *v) {
  struct VQNode *vqn = malloc(sizeof(struct VQNode));
  vqn->next = NULL;
  vqn->prev = NULL;
  vqn->value = v;
  return vqn;
}

VertexQueue *
vq_new() {
  VertexQueue *vq = malloc(sizeof(VertexQueue));
  vq->head = NULL;
  vq->tail = NULL;
  return vq;
}

void
vq_free(VertexQueue *vq) {
  if (vq) {
    while (vq->head) {
      vq_dequeue(vq);
    }
    free(vq);
  }
}

void
vq_enqueue(VertexQueue *vq, Vertex *v) {
  if (vq->head == NULL && vq->tail == NULL) {
    vq->head = vq->tail = vqn_new(v);
  } else if (vq->head == NULL || vq->tail == NULL) {
    printf("VertexQueue bad state: head or tail was NULL\n");
    exit(1);
  } else {
    struct VQNode *new = vqn_new(v);
    new->prev = vq->tail;
    vq->tail->next = new;
    vq->tail = new;
  }
}

Vertex *
vq_dequeue(VertexQueue *vq) {
  if (vq->head == NULL && vq->tail == NULL) {
    return NULL;
  } else if (vq->head == NULL || vq->tail == NULL) {
    printf("VertexQueue bad state: head or tail was NULL\n");
    exit(1);
  } else {
    struct VQNode *removed = vq->head;
    vq->head = removed->next;

    Vertex *v = removed->value;
    free(removed);

    if (vq->head == NULL) vq->tail = NULL;

    return v;
  }
}

// implement the Aho-Corasick algorithm for string searching
void
ac_set_words(Vertex *root, int count, char *words[], int values[]) {
  for (int i = 0; i < count; i++) {
    Vertex *working = root;

    for (char *word = words[i]; *word; word++) {
      working = vertex_traverse_or_add(working, *word);
    }

    working->end_of_word = true;
    working->word_value = values[i];
  }
}

void
ac_calc_links(Vertex *root, Vertex *v) {
  if (v == root) {
    v->suffix_link = root;
    v->end_link = root;
    return;
  }

  if (v->parent == root) {
    v->suffix_link = root;

    if (v->end_of_word) {
      v->end_link = v;
    } else {
      v->end_link = root;
    }

    return;
  }

  Vertex *curr_better = v->parent->suffix_link;
  char ch = v->prev_char;

  while (true) {
    Vertex *suffix_link = NULL;
    if ( (suffix_link = vm_get(curr_better->children, ch)) ) {
      v->suffix_link = suffix_link;
      break;
    }

    if (curr_better == root) {
      v->suffix_link = root;
      break;
    }

    curr_better = curr_better->suffix_link;
  }

  if (v->end_of_word) {
    v->end_link = v;
  } else {
    v->end_link = v->suffix_link->end_link;
  }
}

void
ac_prepare(Vertex *root) {
  VertexQueue *vq = vq_new();
  vq_enqueue(vq, root);

  Vertex *next;
  while ( (next = vq_dequeue(vq)) ) {
    ac_calc_links(root, next);

    for (VertexMap *iter = next->children; iter != NULL; iter = iter->next) {
      vq_enqueue(vq, iter->value);
    }
  }

  vq_free(vq);
}

typedef struct {
  int first;
  int last;
} FirstLastResult;

FirstLastResult
ac_get_first_last(Vertex *root, char *search_string) {
  FirstLastResult result = { 0, 0 };
  bool first_found = false;

  Vertex *curr_state = root;

  for (char *ch = search_string; *ch; ch++) {
    while (true) {
      Vertex *next = NULL;
      if ( (next = vm_get(curr_state->children, *ch)) ) {
        curr_state = next;
        break;
      }

      if (curr_state == root) break;

      curr_state = curr_state->suffix_link;
    }

    Vertex *check_state = curr_state;

    while (true) {
      check_state = check_state->end_link;

      if (check_state == root) break;

      result.last = check_state->word_value;
      if (!first_found) {
        result.first = check_state->word_value;
        first_found = true;
      }

      check_state = check_state->suffix_link;
    }
  }

  return result;
}

int part1(char *input[], int input_len) {
  Vertex *root = vertex_new(NULL, '\0');
  char *words[] = {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"};
  int values[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
  ac_set_words(root, sizeof(words)/sizeof(char*), words, values);
  ac_prepare(root);

  FirstLastResult result;
  int calibration = 0;

  for (int i = 0; i < input_len; i++) {
    result = ac_get_first_last(root, input[i]);
    calibration += (result.first * 10) + result.last;
  }

  vertex_free(root);

  return calibration;
}

int part2(char *input[], int input_len) {
  Vertex *root = vertex_new(NULL, '\0');
  char *words[] = {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
  int values[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9};
  ac_set_words(root, sizeof(words)/sizeof(char*), words, values);
  ac_prepare(root);

  FirstLastResult result;
  int calibration = 0;

  for (int i = 0; i < input_len; i++) {
    result = ac_get_first_last(root, input[i]);
    calibration += (result.first * 10) + result.last;
  }

  vertex_free(root);

  return calibration;
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    printf("no filename specified\n");
    exit(1);
  }

  FILE *f = fopen(argv[1], "r");
  if (f == NULL) {
    if (errno == 2) {
      printf("specified file does not exist\n");
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

  int num_lines = 1; // we don't count trailing newline, so we can always add 1 for the last unterminated line.
  for (char *c = buffer; *c; c++) {
    if (*c == '\n' && *(c + 1) != '\0') num_lines++;
  }

  char **lines = malloc(sizeof(char*) * num_lines);
  char *iter = strtok(buffer, "\n");
  for (int i = 0; iter; i++) {
    lines[i] = iter;
    iter = strtok(NULL, "\n");
  }

  printf("%i\n", part1(lines, num_lines));
  printf("%i\n", part2(lines, num_lines));
}
