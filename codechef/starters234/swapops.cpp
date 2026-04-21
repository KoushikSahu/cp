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
Created: 09:43:22 PM(21:43:22) IST(+05:30) 21-04-2026 Tue
 */

enum CharType { ZERO = 0, ONE = 1, QUESTION_MARK = 2 };

void solve() {
  int n;
  cin >> n;
  string s;
  cin >> s;
  int cnt[3];
  fill_n(&cnt[0], 3, 0);
  for (int i = 0; i < (int)s.length(); i++) {
    if (s[i] == '0')
      cnt[ZERO]++;
    else if (s[i] == '1')
      cnt[ONE]++;
    if (s[i] == '?')
      cnt[QUESTION_MARK]++;
  }
  int ans_mn = (int)s.length(), ans_mx = 0;
  auto can_maximize = [&](int ones, int mn_zeroes) {
    if (ones == 0)
      return mn_zeroes == 0;

    vector<int> available_questions;
    vector<bool> forced_zero(n, false);
    int curr_zeroes = 0, forced_zeroes = 0;

    for (int idx = 0; idx < ones; idx++) {
      if (s[idx] == '0')
        curr_zeroes += 1;
      else if (s[idx] == '?')
        available_questions.push_back(idx);
    }

    for (int l = 0, r = ones - 1;; l++, r++) {
      while (curr_zeroes < mn_zeroes) {
        while (!available_questions.empty() && available_questions.back() < l)
          available_questions.pop_back();
        if (available_questions.empty())
          return false;

        int pos = available_questions.back();
        available_questions.pop_back();
        forced_zero[pos] = true;
        curr_zeroes += 1;
        forced_zeroes += 1;
      }

      if (cnt[ZERO] + forced_zeroes > n - ones)
        return false;
      if (r == n - 1)
        break;

      int nr = r + 1;
      if (s[nr] == '0')
        curr_zeroes += 1;
      else if (s[nr] == '?')
        available_questions.push_back(nr);

      if (s[l] == '0' || forced_zero[l])
        curr_zeroes -= 1;
    }

    return true;
  };
  for (int i = cnt[ONE]; i <= cnt[ONE] + cnt[QUESTION_MARK]; i++) {
    int l = 0, r = -1;
    int char_cnt[3];
    fill_n(&char_cnt[0], 3, 0);
    int curr_mn = s.length();
    int curr_cnt = i;
    while (curr_cnt--) {
      r += 1;
      if (s[r] == '0')
        char_cnt[ZERO] += 1;
      else if (s[r] == '1')
        char_cnt[ONE] += 1;
      else if (s[r] == '?')
        char_cnt[QUESTION_MARK] += 1;
    }
    curr_mn = min(curr_mn, char_cnt[ZERO] + char_cnt[QUESTION_MARK] -
                               min(char_cnt[QUESTION_MARK], i - cnt[ONE]));
    while (r < s.length() - 1) {
      r += 1;
      if (s[r] == '0')
        char_cnt[ZERO] += 1;
      else if (s[r] == '1')
        char_cnt[ONE] += 1;
      else if (s[r] == '?')
        char_cnt[QUESTION_MARK] += 1;

      if (s[l] == '0')
        char_cnt[ZERO] -= 1;
      else if (s[l] == '1')
        char_cnt[ONE] -= 1;
      else if (s[l] == '?')
        char_cnt[QUESTION_MARK] -= 1;
      l += 1;

      curr_mn = min(curr_mn, char_cnt[ZERO] + char_cnt[QUESTION_MARK] -
                                 min(char_cnt[QUESTION_MARK], i - cnt[ONE]));
    }
    ans_mn = min(ans_mn, curr_mn);
  }
  for (int i = cnt[ONE]; i <= cnt[ONE] + cnt[QUESTION_MARK]; i++) {
    int low = 0, high = n - i;
    while (low <= high) {
      int mid = low + (high - low) / 2;
      if (can_maximize(i, mid)) {
        ans_mx = max(ans_mx, mid);
        low = mid + 1;
      } else {
        high = mid - 1;
      }
    }
  }
  cout << ans_mn << ' ' << ans_mx << '\n';
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T = 1;
  cin >> T;
  while (T--) {
    debug(T);
    solve();
  }
  return 0;
}
