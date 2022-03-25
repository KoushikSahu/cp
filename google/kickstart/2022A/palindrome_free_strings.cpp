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

/*
  Author: Koushik Sahu
  Created: 2022-03-20 10:51 IST
*/

bool check(string s, int n){
  bool dp[n][n];
  fill_n(dp[0], n*n, false);
  bool ans = true;
  for(int i=0; i<n; i++) dp[i][i] = true;
  for(int i=n-1; i>=0; i--){
    for(int j=i+1; j<n; j++){
      if(j-i == 1){
        if(s[i] == s[j]) dp[i][j] = true;
      }else{
        dp[i][j] = (s[i] == s[j]) && dp[i+1][j-1];
      }
      if(j-i+1 >= 5 && dp[i][j]) ans = false;
    }
  }
  debug(s, ans);
  return ans;
}

void solve(){
  int n;
  string s;
  cin>>n>>s;
  vint pos;
  for(int i=0; i<n; i++){
    if(s[i] == '?') pos.push_back(i);
  }
  int len = sz(pos);
  bool ans = false;
  for(int mask=0; mask<(1<<len); mask++){
    for(int i=0; i<len; i++){
      s[pos[i]] = ((mask&(1<<i)) > 0) ? '1' : '0';
    }
    if(check(s, n)) ans = true;
  }
  if(ans) cout<<"POSSIBLE\n";
  else cout<<"IMPOSSIBLE\n";
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
