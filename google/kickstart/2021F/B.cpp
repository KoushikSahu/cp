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

int d, n, k, h, s, e;

void solve(){
    cin>>d>>n>>k;
    vint th[d][2];
    for(int i=0; i<n; i++){
        cin>>h>>s>>e;
        s--, e--;
        th[s][0].push_back(h);
        if(e+1<d) th[e+1][1].push_back(h);
    }
    map<int, int> curr;
    map<int, int, greater<int>> rest;
    int curr_cnt = 0;
    ll val = 0, ans = 0;
    for(int i=0; i<d; i++){
        for(int j: th[i][1]){
            if(rest.find(j)==rest.end()){
                curr[j]--;
                if(!curr[j]) curr.erase(j);
                curr_cnt--;
                val -= j;
                if(!rest.empty()){
                    ipair p = *rest.begin();
                    rest[p.first]--;
                    if(!rest[p.first]) rest.erase(p.first);
                    curr[p.first]++;
                    curr_cnt++;
                    val += p.first;
                }
            }else{
                rest[j]--;
                if(!rest[j]) rest.erase(j);
            }
        }
        for(int j: th[i][0]){
            if(curr_cnt<k){
                curr[j]++;
                curr_cnt++;
                val += j;
            }else{
                pair<int, int> cp = *curr.begin();
                if(cp.first < j){
                    curr[cp.first]--;
                    if(curr[cp.first]==0) curr.erase(cp.first);
                    curr_cnt--;
                    val -= cp.first;
                    rest[cp.first]++;
                    curr[j]++;
                    val += j;
                    curr_cnt++;
                }else{
                    rest[j]++;
                }
            }
        }
        ans = max(ans, val);
    }
    cout<<ans<<'\n';
}

int main(){
    int T=1;
    cin>>T;
    for(int i=1; i<=T; i++){
        cout<<"Case #"<<i<<": ";
        solve();
    }
    return 0;
}

