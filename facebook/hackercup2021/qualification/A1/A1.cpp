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

string s;

bool is_vowel(char x){
	return x == 'A' ||
		x == 'E' ||
		x == 'I' ||
		x == 'O' ||
		x == 'U';
}

void testcase(){
	cin>>s;
	int n = sz(s);
	int freq[26];
	fill_n(freq, 26, 0);
	for(char i: s){
		freq[i-'A']++;
	}
	int mxv=0, mxc=0;
	char c='B', v='A';
	for(int i=0; i<26; i++){
		if(is_vowel('A'+i)){
			if(freq[i] > mxv){
				mxv = freq[i];
				v = 'A'+i;
			}
		}else{
			if(freq[i] > mxc){
				mxc = freq[i];
				c = 'A'+i;
			}
		}
	}
	int ans = 205;
	int tmp = 0;
	for(char i: s){
		if(is_vowel(i)){
			if(i!=v) tmp += 2;
		}else{
			tmp += 1;
		}
	}
	ans = min(ans, tmp);
	tmp = 0;
	for(char i: s){
		if(is_vowel(i)) tmp += 1;
		else{
			if(i!=c) tmp += 2;
		}
	}
	ans = min(ans, tmp);
	cout<<ans<<'\n';
}

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
	int t;
	cin>>t;
	for(int i=1; i<=t; i++){
		cout<<"Case #"<<i<<": ";
		testcase();
	}
    return 0;
}

