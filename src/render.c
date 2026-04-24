#include "render.h"
#include "common.h"
#include "player.h"
#include "wfc.h"
#include <asm-generic/ioctls.h>
#include <sys/ioctl.h>
#include <unistd.h>

void render_frame(void) {
  struct winsize ws;
  ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws);

  int oy = (ws.ws_row - MAP_HEIGHT) / 2;
  int ox = (ws.ws_col - MAP_WIDTH) / 2;

  printf("\033[2J");

  for (int i = 0; i < MAP_HEIGHT * MAP_WIDTH; i++) {
    int y = i / MAP_WIDTH;
    int x = i % MAP_WIDTH;

    if (x == 0)
      printf("\033[%d;%dH", oy + y + 1, ox + 1);

    if (player.y == y && player.x == x) {
      printf("\033[38;5;226m@\033[0m");
    } else {
      TileType tile = map[y][x].final_type;
      const char *symbol;
      int color;

      if (tile == TILE_VOID) {
        symbol = " ";
        color = 232;
      } else if (tile == TILE_FLOOR) {
        symbol = "·";
        color = 82;
      } else if (tile == TILE_WALL) {
        symbol = "█";
        color = 196;
      } else if (tile == TILE_ALTAR) {
        symbol = "♦";
        color = 135;
      } else {
        symbol = "?";
        color = 255;
      }

      printf("\033[38;5;%dm%s\033[0m", color, symbol);
    }

    if (x == MAP_WIDTH - 1)
      printf("\n");
  }

  fflush(stdout);
}
