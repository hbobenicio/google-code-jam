#include <iostream>
#include <cstdint>

using i32 = int32_t;
using i64 = int64_t;
using u32 = uint32_t;
using u64 = uint64_t;

template <typename Fn>
static void tloop(Fn f)
{
  u32 T;
  std::cin >> T;

  for(u32 t = 1; t <= T; t++) {
    std::cout << "Case #" << t << ": ";
    f(t);
  }
}

int main()
{
  tloop([](auto t){
    std::cout << '\n';
  });
}
