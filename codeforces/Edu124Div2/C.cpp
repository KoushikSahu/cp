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
  Created: 2022-03-10 20:39 IST
*/

void solve(){
  const int nxm = 2e5+5;
  int n, a[nxm], b[nxm];
  cin>>n;
  for(int i=0; i<n; i++) cin>>a[i];
  for(int i=0; i<n; i++) cin>>b[i];
  int dist[4];
  for(int i=0; i<4; i++){
    int val = 0;
    if(i==0) val = a[0];
    else if(i==1) val = a[n-1];
    else if(i==2) val = b[0];
    else val = b[n-1];
    dist[i] = INF;
    for(int j=0; j<n; j++){
      dist[i] = min(dist[i], abs(val - (i<2 ? b[j] : a[j])));
    }
  }
  ll ans = LONG_LONG_MAX;
  // connect a[0] with b[0]
  ans = min(ans, 1ll*abs(a[0] - b[0]) + dist[1] + dist[3]);
  // connect a[0] wth b[n-1]
  ans = min(ans, 1ll*abs(a[0] - b[n-1]) + dist[1] + dist[2]);
  // connect b[0] with a[n-1]
  ans = min(ans, 1ll*abs(b[0] - a[n-1]) + dist[0] + dist[3]);
  // connect b[n-1] with a[n-1]
  ans = min(ans, 1ll*abs(b[n-1] - a[n-1]) + dist[0] + dist[2]);
  // connect a[0] with b[0] and a[n-1] with b[n-1]
  ans = min(ans, 1ll*abs(b[n-1] - a[n-1]) + abs(b[0]-a[0]));
  // connect a[0] with b[n-1] and a[n-1] with b[0]
  ans = min(ans, 1ll*abs(b[n-1] - a[0]) + abs(b[0]-a[n-1]));
  // not connections of 2 corners
  ans = min(ans, 1ll*dist[0] + dist[1] + dist[2] + dist[3]);
  cout<<ans<<'\n';
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

