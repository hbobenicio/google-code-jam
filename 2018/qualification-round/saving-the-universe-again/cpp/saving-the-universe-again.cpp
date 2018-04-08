#include <iostream>
#include <cstdio>
#include <string>
#include <utility>
#include <algorithm>

using namespace std;

bool impossible(const string& str, unsigned int shield)
{
  unsigned int shoot_count = 0;

  for (const auto& c: str) {
    if (c == 'S') {
      shoot_count++;
    }
  }

  return shoot_count > shield;
}

unsigned int damage(const string& str)
{
  unsigned int power = 1;
  unsigned int total_damage = 0;

  for (const auto& c: str) {
    switch(c) {
      case 'S':
        total_damage += power;
        break;
      case 'C':
        power *= 2;
        break;
    }
  }

  return total_damage;
}

int find_swap_pattern(const string& str)
{
  for (int i = str.size() - 1; i > 0; i--) {
    char c = str[i];

    if (c == 'S') {
      int j = i - 1;
      if (j >= 0) {
        if (str[j] == 'C') {
          return j;
        }
      }
    }
  }

  return -1;
}

int main()
{
  unsigned int T;

  cin >> T;
  for(unsigned int t = 1; t <= T; t++) {
    cout << "Case #" << t << ": ";

    unsigned int shield;
    string str;

    cin >> shield >> str;
    
    if (impossible(str, shield)) {
      cout << "IMPOSSIBLE\n";
      continue;
    }

    unsigned int swap_count = 0;
    while(damage(str) > shield) {
      int swap_index = find_swap_pattern(str);
      if (swap_index != -1) {
        std::swap(str[swap_index], str[swap_index + 1]);
        swap_count++;
      }
    }

    cout << swap_count << '\n';
  }
}
