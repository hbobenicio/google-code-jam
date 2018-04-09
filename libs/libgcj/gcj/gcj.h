#ifndef GCJ_H
#define GCJ_H

#include <iostream>
#include <cstdint>

namespace gcj {

  using i16 = int16_t;
  using i32 = int32_t;
  using i64 = int64_t;
  using u16 = uint16_t;
  using u32 = uint32_t;
  using u64 = uint64_t;

  /**
   * Reads N as u32 from stdin and
   * loops N times, calling the argument.
   * It also prints the 'Case #i: ' to stdout.
   * 
   * @param f Callable object like [](u32 t){ ... }
   */
  template <typename Callable>
  static void case_loop(Callable f)
  {
    u32 T;
    std::cin >> T;

    for (u32 t = 1; t <= T; t++) {
      std::cout << "Case #" << t << ": ";
      f(t);
    }
  }

}

#endif
