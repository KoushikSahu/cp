#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;
using namespace __gnu_pbds;
template<class T> using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
#define M_PI 3.14159265358979323846
#define MOD 998244353
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

int h, w, k, x[2], y[2];

void solve(){
  cin>>h>>w>>k;
  for(int i=0; i<2; i++) cin>>x[i]>>y[i];
  ll dp[2][2][k+1];
  // k = 0
  for(int i=0; i<2; i++){
    for(int j=0; j<2; j++){
      dp[i][j][0] = 0;
    }
  }
  dp[1][1][0] = 1;
  for(int i=1; i<=k; i++){
    for(int r=0; r<2; r++){
      for(int c=0; c<2; c++){
        dp[r][c][i] = 0;
        if(r==c){
          if(r==0){
            dp[r][c][i] += ((h-2+w-2)*dp[0][0][i-1])%MOD;
            dp[r][c][i] %= MOD;
            dp[r][c][i] += dp[1][0][i-1];
            dp[r][c][i] %= MOD;
            dp[r][c][i] += dp[0][1][i-1];
            dp[r][c][i] %= MOD;
          }else{
            dp[r][c][i] += (dp[0][1][i-1]*(h-1))%MOD;
            dp[r][c][i] %= MOD;
            dp[r][c][i] += (dp[1][0][i-1]*(w-1))%MOD;
            dp[r][c][i] %= MOD;
          }
        }else{
          if(r==0){
            dp[r][c][i] += dp[1][1][i-1];
            dp[r][c][i] %= MOD;
            dp[r][c][i] += (dp[0][1][i-1]*(h-2))%MOD;
            dp[r][c][i] %= MOD;
            dp[r][c][i] += (dp[0][0][i-1]*(w-1))%MOD;
            dp[r][c][i] %= MOD;
          }else{
            dp[r][c][i] += (dp[1][0][i-1]*(w-2))%MOD;
            dp[r][c][i] %= MOD;
            dp[r][c][i] += dp[1][1][i-1];
            dp[r][c][i] %= MOD;
            dp[r][c][i] += (dp[0][0][i-1]*(h-1))%MOD;
            dp[r][c][i] %= MOD;
          }
        }
      }
    }
  }
  cout<<dp[x[0]==x[1]][y[0]==y[1]][k]<<'\n';
}

int main(){
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T=1;
  //cin>>T;i
  while(T--){
    solve();
  }
  return 0;
}

