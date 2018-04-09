#include <iostream>
#include <gcj/gcj.h>

using namespace gcj;

void test_case(u32 t) {
  std::cout << t << '\n';
}

int main()
{
  case_loop(test_case);

  // You can also define the test_case handler as a lambda expression.
  // case_loop([](u32 t){
  //   std::cout << t << '\n';
  // });
}
