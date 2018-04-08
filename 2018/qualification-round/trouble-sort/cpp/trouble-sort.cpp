#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

unsigned int error_position(const vector<unsigned long long int>& v) {
  for(unsigned int i = 0, j = 1; i < v.size() - 1; i++, j++) {
    if (v[i] > v[j]) {
      return i;
    }
  }

  return v.size();
}

int main()
{
  unsigned int T;
  unsigned long long int tmp;

  cin >> T;
  for(unsigned int t = 1; t <= T; t++) {
    cout << "Case #" << t << ": ";

    unsigned int n;
    cin >> n;

    vector<unsigned long long int> v, v0, v1;
    v.reserve(n);
    v0.reserve(n / 2);
    v1.reserve(n / 2);

    while(n--) {
      cin >> tmp;
      v.push_back(tmp);
    }

    for(unsigned int i = 0; i < v.size(); i++) {
      if (i % 2 == 0) {
        v0.push_back(v[i]);
      } else {
        v1.push_back(v[i]);
      }
    }

    sort(begin(v0), end(v0));
    sort(begin(v1), end(v1));

    for(unsigned int i = 0; i < v.size(); i++) {
      if (i % 2 == 0) {
        v[i] = v0[i / 2];
      } else {
        v[i] = v1[i / 2];
      }
    }

    unsigned int err_index = error_position(v);
    if (err_index == v.size()) {
      cout << "OK\n";
    } else {
      cout << err_index << '\n';
    }
  }
}
