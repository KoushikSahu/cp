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
    Created: 30 November 2021 Tue 10:08:12
*/

const int nxm = 2e5+5;
int n, m, a, b;
vector<int> g[nxm];
int dsu[nxm];

int find(int u){
    if(dsu[u]<0) return u;
    return dsu[u] = find(dsu[u]);
}

bool merge(int u, int v){
    int pu = find(u);
    int pv = find(v);
    if(pu == pv) return false;
    else{
        if(abs(pv)>abs(pu)){
            swap(u, v);
            swap(pu, pv);
        }
        dsu[pu] += dsu[pv];
        dsu[pv] = pu;
    }
    return true;
}

void solve(){
    cin>>n>>m;
    for(int i=0; i<n; i++) dsu[i] = -1;
    for(int i=0; i<m; i++){
        cin>>a>>b;
        a--, b--;
        g[a].push_back(b);
        g[b].push_back(a);
    }
    int cnt = 0;
    vint ans;
    ans.push_back(0);
    for(int i=n-1; i>0; i--){
        cnt++;
        for(int v: g[i]){
            if(v>i){
                bool added = merge(i, v);
                if(added) cnt--;
            }
        }
        ans.push_back(cnt);
    }
    reverse(all(ans));
    for(int i: ans) cout<<i<<'\n';
}

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    int T = 1;
    // cin>>T; 
    while(T--){
        solve();
    }
    return 0;
}
