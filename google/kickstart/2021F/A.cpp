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

const int INF = 1e9+5;
int n;
string s;

void solve(){
    cin>>n>>s;
    vint l(n, 0), r(n, 0);
    for(int i=0; i<n; i++){
        if(i==0) l[i] = (s[i]=='1' ? 0 : INF);
        else{
            if(s[i]=='1') l[i] = 0;
            else if(l[i-1]<INF) l[i] = l[i-1]+1;
            else l[i] = INF;
        }
    }
    for(int i=n-1; i>=0; i--){
        if(i==n-1) r[i] = (s[i]=='1' ? 0 : INF);
        else{
            if(s[i]=='1') r[i] = 0;
            else if(r[i+1]<INF) r[i] = r[i+1]+1;
            else r[i] = INF;
        }
    }
    ll ans = 0;
    for(int i=0; i<n; i++) ans += 1ll*min(l[i], r[i]);
    cout<<ans<<'\n';
}

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
    int T=1;
    cin>>T;
    for(int i=1; i<=T; i++){
        cout<<"Case #"<<i<<": ";
        solve();
    }
    return 0;
}

