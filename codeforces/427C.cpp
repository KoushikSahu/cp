#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;
using namespace __gnu_pbds;
template<class T> using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
#define M_PI 3.14159265358979323846
#define MOD 1000000007
#define sz(x) (int)x.size()
#define all(x) x.begin(), x.end()
typedef long long ll;
typedef vector<int> vint;
typedef vector<vint> vvint;
typedef vector<ll> vll;
typedef vector<vll> vvll;
typedef pair<int, int> ipair;
typedef pair<ll, ll> llpair;

void __print(int x) {cerr << x;}
void __print(long x) {cerr << x;}
void __print(long long x) {cerr << x;}
void __print(unsigned x) {cerr << x;}
void __print(unsigned long x) {cerr << x;}
void __print(unsigned long long x) {cerr << x;}
void __print(float x) {cerr << x;}
void __print(double x) {cerr << x;}
void __print(long double x) {cerr << x;}
void __print(char x) {cerr << '\'' << x << '\'';}
void __print(const char *x) {cerr << '\"' << x << '\"';}
void __print(const string &x) {cerr << '\"' << x << '\"';}
void __print(bool x) {cerr << (x ? "true" : "false");}

template<typename T, typename V>
void __print(const pair<T, V> &x) {cerr << '{'; __print(x.first); cerr << ','; __print(x.second); cerr << '}';}
template<typename T>
void __print(const T &x) {int f = 0; cerr << '{'; for (auto &i: x) cerr << (f++ ? "," : ""), __print(i); cerr << "}";}
void _print() {cerr << "]\n";}
template <typename T, typename... V>
void _print(T t, V... v) {__print(t); if (sizeof...(v)) cerr << ", "; _print(v...);}
#ifndef ONLINE_JUDGE
#define debug(x...) cerr << "[" << #x << "] = ["; _print(x)
#else
#define debug(x...)
#endif

const int nxm = 1e5+5;
int n, p[nxm], m, u, v;

void dfs_order(int u, vector<int>* g, bool* seen, vector<int>& order){
  seen[u] = true;
  for(int v: g[u]){
    if(!seen[v]) dfs_order(v, g, seen, order);
  }
  order.push_back(u);
}

void dfs_scc(int u, vector<int>* gr, bool* seen, vector<int>& scc){
  seen[u] = true;
  scc.push_back(u);
  for(int v: gr[u]){
    if(!seen[v]) dfs_scc(v, gr, seen, scc);
  }
}

vector<vector<int>> scc(int n, vector<int>* g, vector<int>* gr){
  vector<int> order;
  bool seen[n];
  fill(seen, seen+n, false);
  for(int i=0; i<n; i++){
    if(!seen[i]) dfs_order(i, g, seen, order);
  }
  reverse(order.begin(), order.end());
  fill(seen, seen+n, false);
  vector<vector<int>> sccs;
  for(int i: order){
    if(!seen[i]){
      vector<int> scc;
      dfs_scc(i, gr, seen, scc);
      sccs.push_back(scc);
    }
  }
  return sccs;
}


void solve(){
  cin>>n;
  vector<int> g[n], gr[n];
  for(int i=0; i<n; i++) cin>>p[i];
  cin>>m; 
  for(int i=0; i<m; i++){
    cin>>u>>v;
    u--, v--;
    g[u].push_back(v);
    gr[v].push_back(u);
  }
  vvint sccs = scc(n, g, gr);
  ll ans = 1;
  ll cst = 0;
  for(vint s: sccs){
    int mn = p[s[0]];
    for(int i: s) mn = min(mn, p[i]);
    cst += mn;
    int tmp = 0;
    for(int i: s) tmp += (mn == p[i]);
    ans *= tmp;
    ans %= MOD;
  }
  cout<<cst<<' '<<ans<<'\n';
}

int main(){
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T=1;
  //cin>>T;
  while(T--){
    solve();
  }
  return 0;
}

