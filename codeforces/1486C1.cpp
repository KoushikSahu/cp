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
  Created: 2022-03-10 13:30 IST
*/

map<ipair, int> mp;

int ask(int l, int r){
  if(mp.find({l, r}) != mp.end()) return mp[{l, r}];
  cout<<"? "<<l<<" "<<r<<endl;
  int idx;
  cin>>idx;
  return idx;
}

void solve(){
  int n;
  cin>>n;
  int l = 1, h = n;
  while(l < h){
    int sec_mx = ask(l, h);
    mp[{l, h}] = sec_mx;
    int mid = l + (h - l) / 2;
    if(sec_mx <= mid){
      if(l == mid){
        l = mid+1;
        continue;
      }
      int idx = ask(l, mid);
      mp[{l, mid}] = idx;
      if(idx == sec_mx) h = mid;
      else l = mid+1;
    }else{
      if(mid+1 == h){
        h = mid;
        continue;
      }
      int idx = ask(mid+1, h);
      mp[{mid+1, h}] = idx;
      if(idx == sec_mx) l = mid+1;
      else h = mid;
    }
  }
  cout<<"! "<<l<<'\n';
}

int main(){
  //ios_base::sync_with_stdio(false);
  //cin.tie(NULL);
  int T=1;
  //cin>>T;
  while(T--){
    solve();
  }
  return 0;
}

