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

string x;

void solve(){
  cin>>x;
  int d = sz(x);
  if(d<=2){
    cout<<x<<'\n';
    return ;
  }
  int mnd = (0 - x[0] + '0') / (d-1);
  int mxd = (9 - x[0] + '0') / (d-1);
  ll ans = LONG_LONG_MAX;
  for(int i=mnd; i<=mxd; i++){
    string tmp = x;
    for(int j=1; j<d; j++) tmp[j] = tmp[j-1]+i;
    ll tmpl = stoll(tmp);
    ll xl = stoll(x);
    if(tmpl>=xl) ans = min(ans, tmpl);
  }
  if(x[0] < '9'){
    x[0]++;
  }else{
    x = "10"+x;
  }
  mnd = (0 - x[0] + '0') / (sz(x)-1);
  mxd = (9 - x[0] + '0') / (sz(x)-1);
  for(int i=mnd; i<=mxd; i++){
    string tmp = x;
    for(int j=1; j<sz(x); j++) tmp[j] = tmp[j-1]+i;
    ll tmpl = stoll(tmp);
    ans = min(ans, tmpl);
  }
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

