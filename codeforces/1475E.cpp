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

const int nxm = 1e3+5;
int n, k, a[nxm];
int ncr[nxm][nxm];

void solve(){
  cin>>n>>k;
  for(int i=0; i<n; i++) cin>>a[i];
  sort(a, a+n, greater<int>());
  int kth = a[k-1];
  int p = count(a, a+k, kth);
  int q = count(a, a+n, kth);
  cout<<ncr[q][p]<<'\n';
}

int main(){
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  for(int i=0; i<nxm; i++) ncr[i][0] = 1;
  for(int i=1; i<nxm; i++){
    for(int j=1; j<=i; j++){
      ncr[i][j] = ncr[i-1][j-1]+ncr[i-1][j];
      ncr[i][j] %= MOD;
    }
  }
  int T=1;
  cin>>T;
  while(T--){
    solve();
  }
  return 0;
}

