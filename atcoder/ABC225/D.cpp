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

struct Car{
  int front, back;
};

const int nxm = 1e5+5;
int n, q, type, x, y;
Car c[nxm];

void solve(){
  cin>>n>>q;
  for(int i=0; i<n; i++){
    c[i].front = -1;
    c[i].back = -1;
  }
  while(q--){
    cin>>type;
    if(type==1){
      cin>>x>>y;
      x--, y--;
      c[x].back = y;
      c[y].front = x;
    }else if(type==2){
      cin>>x>>y;
      x--, y--;
      c[x].back = -1;
      c[y].front = -1;
    }else{
      cin>>x;
      int tmp = x-1;
      while(c[tmp].front!=-1){
        tmp = c[tmp].front;
      }
      vint ans;
      while(tmp!=-1){
        ans.push_back(tmp+1);
        tmp = c[tmp].back;
      }
      cout<<sz(ans)<<' ';
      for(int i: ans) cout<<i<<' ';
      cout<<'\n';
    }
  }
}

int main(){
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  int T=1;
  //cin>>T;st
  while(T--){
    solve();
  }
  return 0;
}

