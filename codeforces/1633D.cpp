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
  Created: 2022-03-16 12:20 IST
*/

int find_wt(int targ){
  int seen[targ+1];
  fill_n(seen, targ+1, -1);
  queue<int> q;
  q.push(1);
  seen[1] = 0;
  while(!q.empty()){
    int curr = q.front();
    q.pop();
    if(curr == targ) return seen[targ];
    for(int i=1; i<=curr; i++){
      int now = curr + curr / i;
      if(now<=targ && seen[now]==-1){
        seen[now] = 1 + seen[curr];
        q.push(now);
      }
    }
  }
  return -1;
}

void solve(){
  const int nxm = 1005;
  int n, k, b[nxm], c[nxm];
  cin>>n>>k;
  for(int i=0; i<n; i++) cin>>b[i];
  for(int i=0; i<n; i++) cin>>c[i];

  int wt[nxm];
  for(int i=0; i<n; i++){
    wt[i] = find_wt(b[i]);
  }
  k = min(k, 12*n);
  int dp[n+1][k+1];
  for(int i=0; i<=k; i++) dp[0][i] = 0;
  for(int i=0; i<=n; i++){
    if(i==0) dp[i][0] = 0;
    else{
      if(wt[i-1] == 0) dp[i][0] = c[i-1] + dp[i-1][0];
      else dp[i][0] = dp[i-1][0];
    }
  }
  for(int i=1; i<=n; i++){
    for(int j=1; j<=k; j++){
      if(wt[i-1] <= j) dp[i][j] = max(c[i-1] + dp[i-1][j-wt[i-1]], dp[i-1][j]);
      else dp[i][j] = dp[i-1][j];
    }
  }
  cout<<dp[n][k]<<'\n';
}

int main(){
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T=1;
  cin>>T;
  while(T--){
    solve();
  }
  return 0;
}

