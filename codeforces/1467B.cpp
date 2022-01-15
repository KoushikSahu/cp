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

const int nxm = 3e5+5;
int n, a[nxm];
bool hv[nxm];

bool valid(int idx){
  return (a[idx-1]<a[idx] && a[idx]>a[idx+1]) ||
    (a[idx-1]>a[idx] && a[idx]<a[idx+1]);
}

void solve(){
  cin>>n;
  fill_n(hv, n, false);
  for(int i=0; i<n; i++) cin>>a[i];
  for(int i=1; i<n-1; i++){
    hv[i] = valid(i);
  }
  int ans = 0;
  for(int i=0; i<n; i++) ans += hv[i];
  int final_ans = ans;
  for(int i=0; i<n; i++){
    int tmp = a[i];
    vint choices;
    if(i==0) choices.push_back(a[i+1]);
    else if(i==n-1) choices.push_back(a[i-1]);
    else{
      choices.push_back(a[i-1]);
      choices.push_back(a[i+1]);
      choices.push_back(0);
      choices.push_back(1e9+5);
      choices.push_back((a[i-1]+a[i+1])/2);
    }
    for(int j: choices){
      a[i] = j;
      int tmpp = ans;
      for(int k=-1; k<=1; k++){
        int idx = i+k;
        if(idx>0 && idx<n-1){
          if(!hv[idx] && valid(idx)) tmpp++;
          else if(hv[idx] && !valid(idx)) tmpp--;
        }
      }
      final_ans = min(final_ans, tmpp);
    }
    a[i] = tmp;
  }
  cout<<final_ans<<'\n';
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

