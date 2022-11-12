#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;
using namespace __gnu_pbds;
template<class T> using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
#define M_PI 3.14159265358979323846
#define MOD 998244353
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
	Created: 06 November 2022 Sun 21:14:24
*/

/*
a1, a2, a3, ....ai, ....., an
b1, b2, b3, .., bi

total = (m/ai)^i - (seq having gcd 2) - (seq having gcd 3) .... - (seq having gcd m/ai)
      = (m/ai)^i - (m/2*ai)^i  - (m/3*ai)^i - ... - 1
*/

template <typename T>
T pow_binexpo(T x, ll y, ll m=MOD){
    x %= m;
    T ans = 1; 
    while(y){
        if(y%2){
            ans *= x;
            ans %= m;
        }
        x *= x;
        x %= m;
        y /= 2;
    }
    return ans;
}

void solve(){
	int n, m;
	cin>>n>>m;
	vll a(n);
	for(int i=0; i<n; i++) cin>>a[i];
	ll ans = 0;
	for(int i=0; i<n; i++){
		ll total = pow_binexpo<ll>(m/a[i], i+1);
		debug(total);
		for(int j=2; j<m/a[i]; j++){
			total -= pow_binexpo<ll>(m/(j*a[i]), i+1);
			total %= MOD;
			total += MOD;
			total %= MOD;
		}
		debug(total);
		ans += total;
		ans %= MOD;
	}
	cout<<ans<<'\n';
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
