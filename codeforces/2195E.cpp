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
Created: 08:35:32 PM(20:35:32) IST(+05:30) 30-03-2026 Mon
 */

struct TreeNode {
  int idx, parent;
  TreeNode *left, *right;

  TreeNode(int idx, int parent, TreeNode *left, TreeNode *right)
      : idx(idx), parent(parent), left(left), right(right) {}
};

void solve() {
  int n;
  cin >> n;
  map<int, TreeNode *> node_mp;
  TreeNode *one_node = new TreeNode(1, 0, nullptr, nullptr);
  TreeNode *root = new TreeNode(0, -1, one_node, nullptr);
  node_mp[0] = root;
  node_mp[1] = one_node;
  for (int i = 1; i <= n; i++) {
    int l, r;
    cin >> l >> r;
    if (l != 0 || r != 0) {
      TreeNode *curr = node_mp.find(i) != node_mp.end()
                           ? node_mp[i]
                           : new TreeNode(i, -1, nullptr, nullptr);
      if (node_mp.find(l) != node_mp.end()) {
        curr->left = node_mp[l];
        curr->left->parent = i;
      } else {
        node_mp[l] = new TreeNode(l, i, nullptr, nullptr);
        curr->left = node_mp[l];
      }
      if (node_mp.find(r) != node_mp.end()) {
        curr->right = node_mp[r];
        curr->right->parent = i;
      } else {
        node_mp[r] = new TreeNode(r, i, nullptr, nullptr);
        curr->right = node_mp[r];
      }
      node_mp[i] = curr;
    }
  }
  vector<int> cnt(n + 1, 0);
  function<void(TreeNode *)> dfs = [&](TreeNode *x) {
    if (x == nullptr) {
      return;
    }
    dfs(x->left);
    dfs(x->right);
    cnt[x->idx] = 1 + (x->left != nullptr ? cnt[x->left->idx] : 0) +
                  (x->right != nullptr ? cnt[x->right->idx] : 0);
  };
  dfs(root);
  vector<int> ans(n + 1, -1);
  function<void(int)> recursive_ans_construct = [&](int idx) {
    if (ans[idx] != -1) {
      return;
    }
    if (idx == 0) {
      ans[idx] = 0;
      return;
    }
    TreeNode *nd = node_mp[idx];
    if (ans[nd->parent] == -1) {
      recursive_ans_construct(nd->parent);
    }
    ans[idx] = ((1 + ((cnt[idx] - 1) * 2) % MOD) % MOD + ans[nd->parent]) % MOD;
  };
  for (int i = 1; i <= n; i++) {
    recursive_ans_construct(i);
  }
  for (int i = 1; i <= n; i++) {
    cout << ans[i] << ' ';
  }
  cout << '\n';
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
