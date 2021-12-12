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

int a, b, k;

int maxstepcnt(int x){
  int tmp = x;
  int cnt = 0;
  while(x%2==0){
    x /= 2;
    cnt++;
  }
  for(int i=3; x!=1 && i<=sqrt(tmp); i+=2){
    while(x%i==0){
      x/=i;
      cnt++;
    }
  }
  if(x!=1) cnt++;
  return cnt;
}

void solve(){
  cin>>a>>b>>k;
  int mx = maxstepcnt(a) + maxstepcnt(b);
  int mn = 3;
  if(a == b){
    mn = 0;
  }else{
    if(__gcd(a, b) == min(a, b)){
      mn = 1;
    }else{
      mn = 2;
    }
  }
  if((k>=mn && k<=mx && k==1 && mn==1) || (k>=mn && k<=mx && k!=1)) cout<<"YES\n";
  else cout<<"NO\n";
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

