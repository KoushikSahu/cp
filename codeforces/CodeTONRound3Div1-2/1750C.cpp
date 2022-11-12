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
	Created: 06 November 2022 Sun 20:27:14
*/

/*
0001100
1110011

0010100 -> 0000100 -> 0000000
0010100 -> 1111011 -> 0000000

0011100
1100011

0001100
1110011

111
111

010 -> 110 -> 111 -> 000
101 -> 110 -> 000 -> 000
*/

void mutate(string& b, const string a, int nOps){
	int n = sz(a);
	for(int i=0; i<n; i++){
		if(a[i] == '0'){
			if(nOps%2 == 0){
				b[i] = (1 - (b[i] - '0')) + '0';
			}
		}else{
			if(nOps%2 == 1){
				b[i] = (1 - (b[i] - '0')) + '0';
			}
		}
	}
}

void solve(){
	int n;
	cin>>n;
	string a, b;
	cin>>a>>b;
	int nOps = count(all(a), '0');
	mutate(b, a, nOps);
	if(all_of(all(b), [](char c){return c=='0';})){
		cout<<"YES\n";
		cout<<nOps+1<<'\n';
		for(int i=0; i<n; i++){
			if(a[i] == '0') cout<<i+1<<' '<<i+1<<'\n';
		}
		cout<<1<<' '<<n<<'\n';
	}else if(all_of(all(b), [](char c){return c == '1';})){
		cout<<"YES\n";
		cout<<nOps+2<<'\n';
		for(int i=0; i<n; i++){
			if(a[i] == '0') cout<<i+1<<' '<<i+1<<'\n';
		}
		cout<<1<<' '<<1<<'\n';
		cout<<2<<' '<<n<<'\n';
	}else{
		cout<<"NO\n";
	}
	return ;
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
