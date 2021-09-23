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

const int nxm = 1e5+5;
int T, n[3];
array<array<int, nxm>, 3> a;

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL), cout.tie(NULL);
    cin>>T;
    while(T--){
        for(int i=0; i<3; i++) cin>>n[i];
        for(int i=0; i<3; i++){
            for(int j=0; j<n[i]; j++){
                cin>>a[i][j];
            }
            sort(a[i].begin(), a[i].begin()+n[i]);
        }
        ll ans = LONG_LONG_MAX;
        for(int m=0; m<3; m++){
            for(int t=0; t<3; t++){
                if(t!=m){
                    for(int b=0; b<3; b++){
                        if(b!=m && b!=t){
                            for(int y=0; y<n[m]; y++){
                                int y_val = a[m][y];
                                int low = 0, high = n[t]-1;
                                int x_val = -1;
                                while(low<=high){
                                    int mid = low + (high-low)/2;
                                    if(a[t][mid]>y_val) high = mid-1;
                                    else{
                                        x_val = a[t][mid];
                                        low = mid+1;
                                    }
                                }
                                if(x_val != -1){
                                    low = 0, high = n[b]-1;
                                    int z_val = -1;
                                    while(low<=high){
                                        int mid = low + (high-low)/2;
                                        if(a[b][mid]<y_val) low = mid+1;
                                        else{
                                            z_val = a[b][mid];
                                            high = mid-1;
                                        }
                                    }
                                    if(z_val!=-1){
                                        ll tmp = 1ll*(x_val-y_val)*(x_val-y_val) +
                                            1ll*(y_val-z_val)*(y_val-z_val) +
                                            1ll*(z_val-x_val)*(z_val-x_val);
                                        ans = min(ans, tmp);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        cout<<ans<<'\n';
    }
    return 0;
}

