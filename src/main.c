#include "display.h"
#include <stdbool.h>

int main(void) {
  display_init();
  display_console_init(true, true);
  display_console_write(0, 0, "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ");
}