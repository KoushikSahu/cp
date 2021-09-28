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

int n, p;
string s;

int dist(char a, char b){
    int i1 = a-'a';
    int i2 = b-'a';
    if(i1>i2) swap(i1, i2);
    return min(i2-i1, i1-i2+26);
}

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
    cin>>n>>p>>s;
    int ans = 0;
    int mn = n, mx = -1;
    for(int i=0; i<n/2; i++){
        int val = dist(s[i], s[n-1-i]);
        ans += val;
        if(val){
            mn = min(mn, i);
            mx = max(mx, i);
        }
    }
    if(ans == 0){
        cout<<0<<'\n';
        return 0;
    }
    p -= 1;
    if(p>=n/2) p = (n-1-p);
    if(p <= mn){
        ans += (mn-p + mx-mn);
    }else if(p>=mx){
        ans += (p-mx + mx-mn);
    }else{
        ans += (min(p-mn, mx-p) + mx-mn);
    }
    cout<<ans<<'\n';
    return 0;
}

