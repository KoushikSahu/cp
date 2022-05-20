#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;
using namespace __gnu_pbds;
template<class T> using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
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

/*
  Author: Koushik Sahu
  Created: 2022-04-02 14:51 IST
*/

void solve(){
  const int nxm = 15;
  int n, f[nxm], p;

  cin>>n;
  bool leaf[n+1];
  fill_n(leaf, n+1, true);
  vector<int> g[n + 1];
  f[0] = 0;
  for(int i=1; i<=n; i++) cin>>f[i];
  for(int i=1; i<=n; i++){
    cin>>p;
    g[i].push_back(p);
    leaf[p] = false;
  }
  vint leaves;
  for(int i=1; i<=n; i++) if(leaf[i]) leaves.push_back(i);
  ll ans = 0;
  bool seen[n+1];
  fill_n(seen, n+1, false);
  function<int(int)> dfs = [&](int src){
    seen[src] = true;
    int ret = f[src];
    for(int v: g[src]){
      if(!seen[v]) ret = max(ret, dfs(v));
    }
    return ret;
  };
  do{
    ll curr = 0;
    fill_n(seen, n+1, false);
    for(int l: leaves){
      curr += dfs(l);
    }
    ans = max(ans, curr);
  }while(next_permutation(all(leaves)));
  cout<<ans<<'\n';
}

int main(){
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T=1;
  cin>>T;
  for(int i=1; i<=T; i++){
    cout<<"Case #"<<i<<": ";
    solve();
  }
  return 0;
}

