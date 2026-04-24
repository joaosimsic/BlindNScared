#ifndef PLAYER_H
#define PLAYER_H

#include "common.h"

typedef struct {
  int y, x;
} Player;

extern Player player;

void init_player(void);
bool move_player(int dy, int dx);

#endif // PLAYER_H
