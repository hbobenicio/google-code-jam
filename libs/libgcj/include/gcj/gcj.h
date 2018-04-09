#ifndef GCJ_H
#define GCJ_H

#include <iostream>
#include <gcj/types.h>

/**
 * Reads N as u32 from stdin and
 * loops N times, calling the argument.
 * It also prints the 'Case #i: ' to stdout.
 * 
 * @param f Callable object like [](u32 t){ ... }
 */
template <typename Callable>
static void tloop(Callable f)
{
  u32 T;
  std::cin >> T;

  for(u32 t = 1; t <= T; t++) {
    std::cout << "Case #" << t << ": ";
    f(t);
  }
}

#endif
