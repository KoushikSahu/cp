#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;
using namespace __gnu_pbds;
template<class T> using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;
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

const int nxm = 4e5+5;
int q, x, a, p[nxm];
bool ok[nxm];

void solve1(){
	fill_n(p, nxm, 0);
	cin>>q>>x;
	set<ipair> s;
	for(int i=0; i<x; i++){
		s.insert(make_pair(p[i], i));
	}
	while(q--){
		cin>>a;
		int tmp = a%x;
		s.erase(make_pair(p[tmp], tmp));
		p[tmp]++;
		s.insert(make_pair(p[tmp], tmp));
		int l = s.begin()->first;
		int m = s.begin()->second;
		cout<<l*x+m<<'\n';
	}
}

void solve(){
	fill_n(p, nxm, 0);
	fill_n(ok, nxm, 0);
	cin>>q>>x;
	int ans = 0;
	for(int i=0; i<q; i++){
		cin>>a;
		int tmp = a%x;
		ll num = 1ll*x*p[tmp]+(ll)tmp;
		p[tmp]++;
		if(num<nxm) ok[num] = true;
		while(ok[ans]){
			ans++;
		}
		cout<<ans<<'\n';
	}
}

int main(){
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);
	int T=1;
	//cin>>T;
	while(T--){
		solve1();
	}
	return 0;
}

