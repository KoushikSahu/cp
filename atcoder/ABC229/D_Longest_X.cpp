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
    Created: 27 November 2021 Sat 18:37:49
*/

string s;
int k;

void solve1(){
    cin>>s>>k;
    int n = sz(s);
    int r=-1, cnt=0;
    int ans = 0;
    for(int l=0; l<n; l++){
        while(r+1<n && (
            (s[r+1]=='.' && cnt+1<=k) || 
            (s[r+1]=='X')
        )){
            r++;
            cnt += (s[r]=='.');
        }
        ans = max(ans, r-l+1);
        cnt -= (s[l]=='.');
    }
    cout<<ans<<'\n';
}

void solve(){
    cin>>s>>k;
    int n = sz(s);
    vint cnt(n, 0);
    for(int i=0; i<n; i++){
        if(s[i]=='.') cnt[i]++;
        if(i>0) cnt[i] += cnt[i-1];
    }
    auto fn = [&](int l, int r){
        if(l>r) return 0;
        if(l<1) return cnt[r];
        return cnt[r] - cnt[l-1];
    };
    int low = 0, high = n;
    int ans = 0;
    while(low<=high){
        int mid = low + (high-low)/2;
        bool pos = false;
        for(int i=0; i+mid-1<n; i++){
            pos |= (fn(i, i+mid-1)<=k);
        }
        if(pos){
            ans = mid;
            low = mid+1;
        }else{
            high = mid-1;
        }
    }
    cout<<ans<<'\n';
}

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    int T = 1;
    // cin>>T; 
    while(T--){
        solve1();
    }
    return 0;
}
