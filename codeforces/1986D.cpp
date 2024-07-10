#include <bits/stdc++.h>
using namespace std;

int solve() {
  int n;
  cin >> n;
  string s;
  cin >> s;
  if (n < 3) {
    cout << stoi(s) << "\n";
  } else if (n == 3) {
    int first = stoi(s.substr(0, 2));
    int second = stoi(s.substr(1, 2));
    vector<int> pos = {
        first + (s[2] - '0'),
        first * (s[2] - '0'),
        second + (s[0] - '0'),
        second * (s[0] - '0'),
    };
    int ans = *min_element(pos.begin(), pos.end());
    cout << ans << "\n";
  } else {
    if (s.find('0') != string::npos) {
      cout << "0\n";
      return 0;
    }
    int ans = INT_MAX;
    for (int i = 0; i + 1 < n; i++) {
      vector<int> nums;
      int curr = 0;
      for (int j = 0; j < n; j++) {
        if (j == i) {
          nums.push_back(stoi(s.substr(i, 2)));
        } else if (j == i + 1) {
          continue;
        } else {
          nums.push_back(s[j] - '0');
        }
      }
      for (int num : nums) {
        if (num != 1) {
          curr += num;
        }
      }
      ans = min(ans, curr);
    }
    cout << ans << "\n";
  }
  return 0;
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int t = 1;
  cin >> t;
  while (t--) {
    solve();
  }
  return 0;
}
