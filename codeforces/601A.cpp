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

const int nxm = 405;
int n, m, u, v;
set<int> gt[nxm], gr[nxm];

int bfs(set<int>* s){
  int seen[n];
  fill_n(seen, n, -1);
  queue<int> q;
  q.push(0);
  seen[0] = 0;
  while(!q.empty()){
    int f = q.front();
    q.pop();
    for(int i: s[f]){
      if(seen[i]==-1){
        seen[i] = seen[f] + 1;
        q.push(i);
        if(i==n-1) return seen[i];
      }
    }
  }
  return -1;
}

void solve(){
  cin>>n>>m;
  for(int i=0; i<m; i++){
    cin>>u>>v;
    u--, v--;
    gt[u].insert(v);
    gt[v].insert(u);
  }
  for(int i=0; i<n; i++){
    for(int j=0; j<n; j++){
      if(gt[i].find(j)==gt[i].end()){
        gr[i].insert(j);
      }
    }
  }
  int dt = bfs(gt);
  int dr = bfs(gr);
  int ans = (dt==-1 || dr==-1) ? -1 : max(dt, dr);
  cout<<ans<<'\n';
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

