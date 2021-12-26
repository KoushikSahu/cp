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

const int nxm = 105;
int h, w;
char c[nxm][nxm];
bool seen[nxm][nxm];
int ans = 0;
int x[] = {0, 1};
int y[] = {1, 0};

bool is_valid(int x, int y){
	return (x>=0 && x<h && y>=0 && y<w);
}

void dfs(ipair p){
	seen[p.first][p.second] = true;
	ans = max(ans, p.first+p.second);
	for(int i=0; i<2; i++){
		int vx = p.first+x[i];
		int vy = p.second+y[i];
		if(is_valid(vx, vy) && c[vx][vy]!='#' && !seen[vx][vy]) dfs(make_pair(vx, vy));
	}
}

void solve(){
	cin>>h>>w;
	for(int i=0; i<h; i++){
		for(int j=0; j<w; j++){
			cin>>c[i][j];
		}
	}
	fill_n(seen[0], h*w, false);
	dfs(make_pair(0, 0));
	cout<<ans+1<<'\n';
}

int main(){
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);
	int T=1;
	//cin>>T;i
	while(T--){
		solve();
	}
	return 0;
}

