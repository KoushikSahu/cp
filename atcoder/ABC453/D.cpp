#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
#include <tuple>
using namespace std;
using namespace __gnu_pbds;
template <class T>
using ordered_set =
    tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
#define M_PI 3.14159265358979323846
#define MOD 1000000007
#define INF 1000000005
#define NEG_INF -1000000005
#define sz(x) (int)x.size()
#define all(x) x.begin(), x.end()
typedef long long ll;
typedef vector<int> vint;
typedef vector<vint> vvint;
typedef vector<ll> vll;
typedef vector<vll> vvll;
typedef pair<int, int> ipair;
typedef pair<ll, ll> llpair;

void __print(int x) { cerr << x; }
void __print(long x) { cerr << x; }
void __print(long long x) { cerr << x; }
void __print(unsigned x) { cerr << x; }
void __print(unsigned long x) { cerr << x; }
void __print(unsigned long long x) { cerr << x; }
void __print(float x) { cerr << x; }
void __print(double x) { cerr << x; }
void __print(long double x) { cerr << x; }
void __print(char x) { cerr << '\'' << x << '\''; }
void __print(const char *x) { cerr << '\"' << x << '\"'; }
void __print(const string &x) { cerr << '\"' << x << '\"'; }
void __print(bool x) { cerr << (x ? "true" : "false"); }

template <typename T, typename V> void __print(const pair<T, V> &x) {
  cerr << '{';
  __print(x.first);
  cerr << ',';
  __print(x.second);
  cerr << '}';
}
template <typename T> void __print(const T &x) {
  int f = 0;
  cerr << '{';
  for (auto &i : x)
    cerr << (f++ ? "," : ""), __print(i);
  cerr << "}";
}
void _print() { cerr << "]\n"; }
template <typename T, typename... V> void _print(T t, V... v) {
  __print(t);
  if (sizeof...(v))
    cerr << ", ";
  _print(v...);
}
#ifndef ONLINE_JUDGE
#define debug(x...)                                                            \
  cerr << "[" << #x << "] = [";                                                \
  _print(x)
#else
#define debug(x...)
#endif

/*
Author: Koushik Sahu
Created: 08:24:51 PM(20:24:51) IST(+05:30) 13-04-2026 Mon
 */

void solve() {
  int n, m;
  cin >> n >> m;
  vector<string> s(n);
  for (int i = 0; i < n; i++)
    cin >> s[i];
  pair<int, int> start, end;
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      if (s[i][j] == 'S')
        start = {i, j};
      else if (s[i][j] == 'G')
        end = {i, j};
    }
  }
  vector<pair<int, int>> dirs{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
  queue<tuple<pair<int, int>, int>> q;
  function<bool(int, int)> valid = [&](int x, int y) {
    return x >= 0 && x < n && y >= 0 && y < m;
  };
  bool seen[n][m][4];
  tuple<pair<int, int>, int> prev[n][m][4];
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      for (int k = 0; k < 4; k++) {
        seen[i][j][k] = false;
        prev[i][j][k] = make_tuple(make_pair(-1, -1), -1);
      }
    }
  }
  for (int i = 0; i < 4; i++) {
    int dr = dirs[i].first;
    int dc = dirs[i].second;
    int r = start.first + dr;
    int c = start.second + dc;
    if (valid(r, c) && s[r][c] != '#') {
      prev[r][c][i] = {start, -1};
      seen[r][c][i] = true;
      q.push({{r, c}, i});
    }
  }
  int ans_dir = -1;
  while (!q.empty()) {
    auto [curr, prev_d] = q.front();
    q.pop();
    if (curr == end) {
      ans_dir = prev_d;
      break;
    }
    for (int i = 0; i < 4; i++) {
      if (s[curr.first][curr.second] == 'o' && i != prev_d)
        continue;
      if (s[curr.first][curr.second] == 'x' && i == prev_d)
        continue;
      int dr = dirs[i].first;
      int dc = dirs[i].second;
      int r = curr.first + dr;
      int c = curr.second + dc;
      if (valid(r, c) && !seen[r][c][i] && s[r][c] != '#') {
        seen[r][c][i] = true;
        prev[r][c][i] = {curr, prev_d};
        q.push({{r, c}, i});
      }
    }
  }
  vector<int> ans;
  pair<int, int> curr = end;
  while (curr != start) {
    if (ans_dir == -1)
      break;
    ans.push_back(ans_dir);
    auto tmp = prev[curr.first][curr.second][ans_dir];
    curr = get<0>(tmp);
    ans_dir = get<1>(tmp);
  }
  if (curr != start) {
    cout << "No\n";
    return;
  }
  cout << "Yes\n";
  for (int i = (int)ans.size() - 1; i >= 0; i--) {
    switch (ans[i]) {
    case 0: {
      cout << 'U';
      break;
    }
    case 1: {
      cout << 'D';
      break;
    }
    case 2: {
      cout << 'L';
      break;
    }
    case 3: {
      cout << 'R';
      break;
    }
    }
  }
  cout << '\n';
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T = 1;
  // cin>>T;
  while (T--) {
    solve();
  }
  return 0;
}
