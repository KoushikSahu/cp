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
  Created: 2022-03-20 08:36 IST
*/

int complement(int x, int n){
  return x ^ (n-1);
}

void solve(){
  int n, k;
  cin>>n>>k;
  bool taken[n];
  fill_n(taken, n, false);
  if(k == 0){
    for(int i=0; i<n; i++){
      if(!taken[i]){
        int c = complement(i, n);
        cout<<i<<' '<<c<<'\n';
        taken[i] = taken[c] = true;
      }
    }
  }else if(k < n-1){
    int c = complement(k, n);
    cout<<0<<' '<<c<<'\n';
    cout<<k<<' '<<n-1<<'\n';
    taken[0] = taken[k] = taken[c] = taken[n-1] = true;
    for(int i=0; i<n; i++){
      if(!taken[i]){
        c = complement(i, n);
        cout<<i<<' '<<c<<'\n';
        taken[i] = taken[c] = true;
      }
    }
  }else{
    if(n==4) cout<<-1<<'\n';
    else{
      cout<<n-1<<' '<<n-2<<'\n';
      cout<<n-3<<' '<<1<<'\n';
      cout<<0<<' '<<2<<'\n';
      taken[n-1] = taken[n-2] = taken[n-3] = taken[1] = taken[0] = taken[2] = true;
      for(int i=0; i<n; i++){
        if(!taken[i]){
          int c = complement(i, n);
          cout<<i<<' '<<c<<'\n';
          taken[i] = taken[c] = true;
        }
      }
    }
  }
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

