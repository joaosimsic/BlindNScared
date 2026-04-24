#include "player.h"
#include "wfc.h"

Player player;

void init_player(void) {
  for (int i = 0; i < MAP_HEIGHT * MAP_WIDTH; i++) {
    int y = i / MAP_WIDTH;
    int x = i % MAP_WIDTH;

    if (map[y][x].final_type == TILE_FLOOR) {
      player.y = y;
      player.x = x;
      return;
    }
  }

  player.y = MAP_HEIGHT / 2;
  player.x = MAP_WIDTH / 2;
}

bool move_player(int dy, int dx) {
  int ny = player.y + dy;
  int nx = player.x + dx;

  if (ny < 0 || ny >= MAP_HEIGHT || nx < 0 || nx >= MAP_WIDTH)
    return false;

  TileType tile = map[ny][nx].final_type;

  if (tile == TILE_WALL || tile == TILE_VOID)
    return false;

  player.y = ny;
  player.x = nx;

  return true;
}
