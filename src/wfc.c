#include "wfc.h"
#include "common.h"

Cell map[MAP_HEIGHT][MAP_WIDTH];

void init_map(void) {
  const Cell default_cell = {
      .entropy = ALL_TILES, .collapsed = false, .final_type = 0};

  Cell *ptr = &map[0][0];

  int total_cells = MAP_HEIGHT * MAP_WIDTH;

  for (int i = 0; i < total_cells; i++) {
    ptr[i] = default_cell;
  }
}

unsigned int get_allowed_neighbors(TileType center_tile) {
  switch (center_tile) {
  case TILE_ALTAR:
    return TILE_FLOOR;
  case TILE_FLOOR:
    return TILE_WALL | TILE_FLOOR | TILE_ALTAR;
  case TILE_WALL:
    return TILE_VOID | TILE_FLOOR | TILE_WALL;
  case TILE_VOID:
    return TILE_VOID | TILE_WALL;
  default:
    return 0;
  }
}

int count_possibilities(unsigned int entropy) {
  return __builtin_popcount(entropy);
}
