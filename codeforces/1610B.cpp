#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;
using namespace __gnu_pbds;
template<class T> using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
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

/*
  Author: Koushik Sahu
  Created: 24 November 2021 Wed 23:41:27
*/

const int nxm = 2e5+5;
int n, a[nxm];

bool is_palindrome(const vector<int>& b){
  int l = 0, h = sz(b)-1;
  while(l<=h){
    if(b[l++]!=b[h--]) return false;
  }
  return true;
}

void solve(){
  cin>>n;
  for(int i=0; i<n; i++) cin>>a[i];
  bool done = true;
  set<int> to_check;
  for(int i=0; i<(n-1)/2; i++){
    if(a[i]!=a[n-1-i]){
      done = false;
      to_check.insert(a[i]);
      to_check.insert(a[n-1-i]);
      break;
    }
  }
  for(int j: to_check){
    vector<int> res;
    for(int i=0; i<n; i++){
      if(j!=a[i]) res.push_back(a[i]);
    }
    done |= is_palindrome(res);
  }
  if(done){
    cout<<"YES\n";
  }else{
    cout<<"NO\n";
  }
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
