#include <bits/stdc++.h>
using namespace std;
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

const int nxm = 2e5+5;
int n, u, v;
vint g[nxm];
array<int, nxm> a;

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
    cin>>n;
    for(int i=0; i<n-1; i++){
        cin>>u>>v;
        u--, v--;
        g[u].push_back(v);
        g[v].push_back(u);
    }
    array<int, nxm> par, dist;
    par.fill(-1);
    dist.fill(0);
    queue<int> q;
    q.push(0);
    while(!q.empty()){
        u = q.front();
        q.pop();
        for(int i: g[u]){
            if(i!=par[u]){
                dist[i] = dist[u] + 1;
                par[i] = u;
                q.push(i);
            }
        }
    }
    map<int, int> mp;
    for(int i=0; i<n; i++){
        cin>>a[i];
        a[i]--;
        mp[a[i]] = i;
    }
    bool ans = true;
    for(int i=1; i<n; i++){
        if((dist[a[i]] < dist[a[i-1]]) || (mp[par[a[i]]] < mp[par[a[i-1]]])){
            ans = false;
        }
    }
    if(ans) cout<<"Yes\n";
    else cout<<"No\n";
    return 0;
}

