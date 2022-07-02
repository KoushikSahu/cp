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
	Created: 28 June 2022 Tue 23:33:01
*/

void solve(){
	int n, q;
	string s;
	cin>>n>>q>>s;
	vvint freq(n, vint(26, 0));
	for(int i=0; i<n; i++){
		freq[i][s[i]-'A'] += 1;
	}
	for(int j=0; j<26; j++){
		for(int i=1; i<n; i++){
			freq[i][j] += freq[i-1][j];
		}
	}
	function<int(int, int, int)> query = [&](int l, int r, int c){
		if(l==0) return freq[r][c];
		return freq[r][c] - freq[l-1][c];
	};
	int ans = 0;
	int l, r;
	while(q--){
		cin>>l>>r;
		l--, r--;
		int total = r - l + 1, odd_cnt = 0;
		for(int c=0; c<26; c++){
			odd_cnt += (query(l, r, c)%2);
		}
		if(total%2 && odd_cnt==1) ans += 1;
		else if(total%2==0 && odd_cnt==0) ans += 1;
	}
	cout<<ans<<'\n';
}

int main(){
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);
	int T = 1;
	cin>>T;
	for(int i=1; i<=T; i++){
		cout<<"Case #"<<i<<": ";
		solve();
	}
	return 0;
}
