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

int t;
string s;

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
    cin>>t;
    while(t--){
        cin>>s;
        int n = sz(s);
        if(n==1){
            cout<<"YES\n"<<s;
            for(char x='a'; x<='z'; x++){
                if(x!=s[0]) cout<<x;
            }
            cout<<'\n';
            continue;
        }
        set<char> adj[26];
        for(int i=1; i<n; i++){
            if(s[i]!=s[i-1]){
                adj[s[i]-'a'].insert(s[i-1]);
                adj[s[i-1]-'a'].insert(s[i]);
            }
        }
        bool valid = true;
        vector<char> adj_cnt[3];
        for(int i=0; i<26; i++){
            if(sz(adj[i])>2) valid = false;
            else{
                adj_cnt[sz(adj[i])].push_back(i+'a');
            }
        }
        valid &= (sz(adj_cnt[1])>=2);
        if(!valid) cout<<"NO\n";
        else{
            string ans = "";
            ans += adj_cnt[1][0];
            bool done[26] = {0};
            done[ans.back()-'a'] = true;
            while(true){
                set<char> pos = adj[ans.back()-'a'];
                bool okay = false;
                for(char i: pos){
                    if(!done[i-'a'] && (sz(ans) == 1 || i != ans[sz(ans)-2])){
                        okay = true;
                        ans += i;
                        done[i-'a'] = true;
                        goto label;
                    }
                }
                if(!okay) break;
                label:;
            }
            for(char c: adj_cnt[0]) ans += c;
            if(sz(ans)==26) cout<<"YES\n"<<ans<<'\n';
            else cout<<"NO\n";
        }
    }
    return 0;
}

