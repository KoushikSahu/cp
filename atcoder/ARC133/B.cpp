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
  Created: 2022-01-22 18:28 IST
*/

struct Dp {
  int ans, idx;
  
  Dp(){
    ans = 0;
    idx = 0;
  }
};

const int nxm = 2e5+5;
int n, p[nxm], q[nxm];

int bin_search(vint& lis, int val){
  int low=0, high=sz(lis)-1, ans=-1;
  while(low<=high){
    int mid = low + ((high-low)>>1);
    if(lis[mid]>=val){
      ans = mid;
      high = mid-1;
    }else{
      low = mid+1;
    }
  }
  return ans;
}

void solve(){
  cin>>n;
  for(int i=0; i<n; i++) cin>>p[i];
  int idx[n+1];
  for(int i=0; i<n; i++){
    cin>>q[i];
    idx[q[i]] = i;
  }
  vector<ipair> vp;
  for(int i=0; i<n; i++){
    int val = p[i];
    while(val<=n){
      vp.push_back({i, idx[val]});
      val += p[i];
    }
  }
  sort(all(vp), [](ipair a, ipair b){
      return make_pair(a.first, -a.second) < make_pair(b.first, -b.second);
      });
  vint lis;
  lis.push_back(vp[0].second);
  for(int i=1; i<sz(vp); i++){
    if(vp[i].second > lis.back()){
      lis.push_back(vp[i].second);
    }else{
      int j = bin_search(lis, vp[i].second);
      lis[j] = vp[i].second;
    }
  }
  cout<<sz(lis)<<'\n';
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

