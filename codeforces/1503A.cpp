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

int t, n;
string s;

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
    cin>>t;
    while(t--){
        cin>>n>>s;
        int z = count(all(s), '0');
        int o = count(all(s), '1');
        string ans = "";
        if(z%2==0 && s[0]=='1' && s[n-1]=='1'){
            int zz = 0, oo = 0;
            for(int i=0; i<n; i++){
                if(s[i] == '0'){
                    zz++;
                    if(zz%2==1) ans.push_back(')');
                    else ans.push_back('(');
                }else{
                    oo++;
                    if(oo <= o/2) ans.push_back('(');
                    else ans.push_back(')');
                }
            }
            string inv = ans;
            for(int i=0; i<n; i++){
                if(s[i] == '0'){
                    inv[i] = inv[i]=='(' ? ')' : '(';
                }
            }
            cout<<"YES\n"<<ans<<'\n'<<inv<<'\n';
        }else{
            cout<<"NO\n";
        }
    }
    return 0;
}

