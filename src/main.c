#include "common.h"
#include "wfc.h"

int main() {
  printf("initilizing superposition...\n");
  init_map();

  int entropy_count = count_possibilities(map[0][0].entropy);
  printf("System check: Cell [0][0] has %d possible states.\n", entropy_count);

  if (entropy_count == 4) {
    printf("Engine ready. Reality is unwritten.\n");
  } else {
    printf("Core system failure.\n");
  }

  return 0;
}
