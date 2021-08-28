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

const int nxm = 55, INF = 55;
int n;
string s[nxm];

void testcase(){
	cin>>n;
	for(int i=0; i<n; i++) cin>>s[i];
	int row[n], col[n];
	fill_n(row, n, 0);
	fill_n(col, n, 0);
	for(int i=0; i<n; i++){
		for(int j=0; j<n; j++){
			if(s[i][j] == 'O'){
				row[i] = col[j] = INF;
			}else if(s[i][j] == '.'){
				if(row[i]!=INF) row[i]++;
				if(col[j]!=INF) col[j]++;
			}
		}
	}
	int mn = min({*min_element(row, row+n), *min_element(col, col+n)});
	int rep = 0;
	for(int i=0; i<n; i++){
		if(row[i] == mn){
			for(int j=0; j<n; j++){
				if(s[i][j] == '.' && col[j] == 1){
					rep++;
				}
			}
		}
	}
	int ans = count(row, row+n, mn)
		+ count(col, col+n, mn)
		- rep;
	if(mn < INF) cout<<mn<<' '<<ans<<'\n';
	else cout<<"Impossible\n";
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

