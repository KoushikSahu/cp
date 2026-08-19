#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
#include <vector>
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

template <typename T, typename V>
void __print(const pair<T, V> &x) {
  cerr << '{';
  __print(x.first);
  cerr << ',';
  __print(x.second);
  cerr << '}';
}
template <typename T>
void __print(const T &x) {
  int f = 0;
  cerr << '{';
  for (auto &i : x)
    cerr << (f++ ? "," : ""), __print(i);
  cerr << "}";
}
void _print() { cerr << "]\n"; }
template <typename T, typename... V>
void _print(T t, V... v) {
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
Created: 06:29:50 PM(18:29:50) IST(+05:30) 15-08-2026 Sat
 */

struct Comparator {
  bool operator()(ipair below, ipair above) {
    if (above.first > below.first) {
      return above.second > below.second + (above.first - below.first);
    }
    return above.second + (below.first - above.first) > below.second;
  }
};

void solve() {
  int q, v;
  cin >> q >> v;
  priority_queue<ipair, vector<ipair>, Comparator> pq;
  for (int i = 0; i < q; i++) {
    int ty;
    cin >> ty;
    if (ty == 1) {
      int tq, wq;
      cin >> tq >> wq;
      pq.push({tq, wq});
    } else {
      int tq;
      cin >> tq;
      if (pq.empty()) {
        cout << -1 << '\n';
        continue;
      }
      auto tp = pq.top();
      pq.pop();
      cout << min(v, tp.second + (tq - tp.first)) << '\n';
    }
  }
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
