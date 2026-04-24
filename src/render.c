#include "render.h"
#include "player.h"
#include "wfc.h"

void render_frame(void) {
  printf("\033[2J\033[H");

  for (int i = 0; i < MAP_HEIGHT * MAP_WIDTH; i++) {
    int y = i / MAP_WIDTH;
    int x = i % MAP_WIDTH;

    if (player.y == y && player.x == x) {
      printf("\033[38;5;226m@\033[0m");
    } else {
      TileType tile = map[y][x].final_type;
      unsigned char symbol;
      int color;

      if (tile == TILE_VOID) {
        symbol = ' ';
        color = 232;
      } else if (tile == TILE_FLOOR) {
        symbol = 183;
        color = 82;
      } else if (tile == TILE_WALL) {
        symbol = 219;
        color = 196;
      } else if (tile == TILE_ALTAR) {
        symbol = 4;
        color = 135;
      } else {
        symbol = '?';
        color = 255;
      }

      printf("\033[38;5;%dm%c\033[0m", color, symbol);
    }

    if (x == MAP_WIDTH - 1)
      printf("\n");
  }

  fflush(stdout);
}
