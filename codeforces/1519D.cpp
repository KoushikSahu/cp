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

const int nxm = 5005;
int n;
ll a[nxm], b[nxm], pref[nxm];

ll sum(int l, int r){
    if(l>r) return 0;
    return pref[r] - (l==0 ? 0 : pref[l-1]);
}

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
    cin>>n;
    for(int i=0; i<n; i++) cin>>a[i];
    for(int i=0; i<n; i++) cin>>b[i];
    pref[0] = a[0]*b[0];
    for(int i=1; i<n; i++){
        pref[i] = pref[i-1]+a[i]*b[i];
    }
    ll dp[n][n];
    for(int i=0; i<n; i++) dp[i][i] = a[i]*b[i];
    for(int i=n-1; i>=0; i--){
        for(int j=i+1; j<n; j++){
            if(j-i == 1) dp[i][j] = a[i]*b[j] + b[i]*a[j];
            else dp[i][j] = dp[i+1][j-1] + a[i]*b[j] + b[i]*a[j];
        }
    }
    ll mx = LONG_LONG_MIN;
    for(int i=0; i<n; i++){
        for(int j=i; j<n; j++){
            mx = max(mx, sum(0, i-1) + dp[i][j] + sum(j+1, n-1));
        }
    }
    cout<<mx<<'\n';
    return 0;
}

