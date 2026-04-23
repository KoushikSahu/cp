#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
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
Created: 06:33:05 PM(18:33:05) IST(+05:30) 18-04-2026 Sat
 */

void solve() {
  int n, a, b;
  cin >> n >> a >> b;
  a -= 1, b -= 1;
  if (n % 2 == 1 || (a + b) % 2 == 0) {
    cout << "No\n";
    return;
  }
  string s1 = "", s2 = "";
  string ss1 = "";
  for (int i = 0; i < n - 1; i++) {
    ss1 += 'R';
  }
  ss1 += 'D';
  for (int i = 0; i < n - 1; i++) {
    ss1 += 'L';
  }
  ss1 += 'D';
  string ss1_rev = ss1;
  reverse(all(ss1_rev));
  for (int i = 0; i < n / 2 - 1; i++) {
    if (a >= 2) {
      s1 += ss1;
      a -= 2;
    } else {
      s2 = ss1_rev + s2;
    }
  }
  string ss2 = "DRUR";
  string ss2_rev = ss2;
  reverse(all(ss2_rev));
  for (int i = 0; i < n / 2 - 1; i++) {
    if (b >= 2) {
      s1 += ss2;
      b -= 2;
    } else {
      s2 = ss2_rev + s2;
    }
  }
  if (a == 0 && b == 1) {
    s1 += "DR";
  } else {
    s1 += "RD";
  }
  cout << "Yes\n";
  cout << s1 + s2 << '\n';
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T = 1;
  cin >> T;
  while (T--) {
    solve();
  }
  return 0;
}
