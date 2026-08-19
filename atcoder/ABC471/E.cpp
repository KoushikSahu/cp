#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;
using namespace __gnu_pbds;
template <class T>
using ordered_set =
    tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
#define M_PI 3.14159265358979323846
#define MOD 998244353
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
Created: 06:51:55 PM(18:51:55) IST(+05:30) 15-08-2026 Sat
 */

class Combinatorics {
public:
  Combinatorics(int mod, int n) : mod(mod), N(n + 1) {
    fact.assign(N, 1);
    fact_inverse.assign(N, 1);
    for (int i = 1; i < N; i++)
      fact[i] = (1LL * i * fact[i - 1]) % mod;
    fact_inverse[N - 1] = modpow(fact[N - 1], mod - 2);
    for (int i = N - 1; i >= 1; i--)
      fact_inverse[i - 1] = (1LL * i * fact_inverse[i]) % mod;
  }

  ll factorial(int x) {
    assert(x < N);
    return fact[x];
  }

  ll inverse_factorial(int x) {
    assert(x < N);
    return fact_inverse[x];
  }

  ll nCr(ll n, ll r) {
    if (r < 0 || r > n)
      return 0;
    assert(n < N && r < N);
    ll ans = (((fact[n] * fact_inverse[r]) % mod) * fact_inverse[n - r]) % mod;
    return ans;
  }

  ll nPr(ll n, ll r) {
    if (r < 0 || r > n)
      return 0;
    assert(n < N && r < N);
    ll ans = (fact[n] * fact_inverse[n - r]) % mod;
    return ans;
  }

private:
  int mod;
  int N;
  vector<ll> fact, fact_inverse;

  ll modpow(ll base, ll exp) {
    ll res = 1;
    while (exp) {
      if (exp & 1)
        res = res * base % mod;
      base = base * base % mod;
      exp >>= 1;
    }
    return res;
  }
};

void solve() {
  int n, k;
  cin >> n >> k;
  ll ans1 = 0, ans2 = 0;
  ll a[n];
  for (int i = 0; i < n; i++) {
    cin >> a[i];
  }
  ll sm = 0;
  for (int i = 0; i < n; i++) {
    sm += a[i];
    sm %= MOD;
  }
  for (int i = 0; i < n; i++) {
    ll tmp = (a[i] * a[i]) % MOD;
    ans1 += tmp;
    ans1 %= MOD;
    ans2 += (((a[i] * (sm - a[i]) % MOD)) % MOD + MOD) % MOD;
    ans2 %= MOD;
  }
  Combinatorics comb(MOD, n);
  ans1 *= comb.nCr(n - 1, k - 1);
  ans1 %= MOD;
  ans2 *= comb.nCr(n - 2, k - 2);
  ans2 %= MOD;
  cout << (ans1 + ans2) % MOD << '\n';
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
