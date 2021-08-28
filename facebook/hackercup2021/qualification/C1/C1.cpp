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

const int nxm = 55;
int n, c[nxm], par[nxm], a, b;
bool leaf[nxm];

void dfs(int u, vint* g){
	bool is_leaf = true;
	for(int v: g[u]){
		if(v!=par[u]){
			is_leaf = false;
			par[v] = u;
			c[v] += c[u];
			dfs(v, g);
		}
	}
	leaf[u] = is_leaf;
}

int lca(int i, int j){
	set<int> seen;
	while(i!=-1){
		seen.insert(i);
		i = par[i];
	}
	while(j!=-1){
		if(seen.find(j)!=seen.end()) return j;
		j = par[j];
	}
	return -1;
}

void testcase(){
	cin>>n;
	for(int i=0; i<n; i++) cin>>c[i];
	vint g[nxm];
	for(int i=0; i<n-1; i++){
		cin>>a>>b;
		a--, b--;
		g[a].push_back(b);
		g[b].push_back(a);
	}
	fill_n(par, n, -1);
	fill_n(leaf, n, 0);
	dfs(0, g);
	int ans = *max_element(c, c+n);
	for(int i=0; i<n; i++){
		for(int j=i+1; j<n; j++){
			if(leaf[i] && leaf[j]){
				if(lca(i, j)==0) ans = max(ans, c[i]+c[j]-c[0]);
			}
		}
	}
	cout<<ans<<'\n';
}

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
	int t=1;
	cin>>t;
	for(int i=1; i<=t; i++){
		cout<<"Case #"<<i<<": ";
		testcase();
	}
    return 0;
}

