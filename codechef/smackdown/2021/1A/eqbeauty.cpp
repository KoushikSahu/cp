#include <bits/stdc++.h>
using namespace std;
#define M_PI 3.14159265358979323846
#define MOD 1000000007
#define all(x) x.begin(), x.end()
#define sz(x) (int)x.size()
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
int n; 
array<int, nxm> a;

bool valid(int idx){
  return idx!=0 && idx!=n-1;
}

void solve(){
  cin>>n;
  for(int i=0; i<n; i++) cin>>a[i];
  sort(a.begin(), a.begin()+n);
  int x = a[0], y = a[n-1];
  int ans = y-a[1];
  for(int i=1; i<n-1; i++){
    int x_diff = a[i] - x;
    int req = y - x_diff;
    int idx = lower_bound(a.begin()+1, a.begin()+i, req) - a.begin();
    if(idx != i){
      int y_diff = y - a[idx];
      ans = min(ans, abs(x_diff - y_diff));
      if(idx>1){
        y_diff = y - a[idx-1];
        ans = min(ans, abs(x_diff - y_diff));
      }
    }
    if(i>1){
      int y_diff = y-a[i-1];
      ans = min(ans, abs(x_diff - y_diff));
    }
    if(i<n-1){
      int y_diff = y-a[i+1];
      ans = min(ans, abs(x_diff - y_diff));
    }
  }
  cout<<ans<<'\n';
}

int main(){
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T = 1;
  cin>>T; 
  while(T--){
    solve();
  }
  return 0;
}
