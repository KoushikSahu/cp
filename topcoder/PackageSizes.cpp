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
 
/*
    Author: Koushik Sahu
*/

class PackageSizes {
    public:
    int getMinimum(vector <int> sizes, int total) {
        const int MAX = 1e9+5;
        set<int> s;
        for(int i: sizes) s.insert(i);
        vint szs;
        for(int i: s) szs.push_back(i);
        int n = sz(szs);
        int dp[total+1][n+1];
        for(int i=0; i<=total; i++) dp[i][0] = MAX;
        for(int i=0; i<=n; i++) dp[0][i] = 0;
        for(int sm=1; sm<=total; sm++){
            for(int i=1; i<=n; i++){
                dp[sm][i] = MAX;
                for(int j=0; j*szs[i-1]<=sm; j++){
                    dp[sm][i] = min(dp[sm][i], j+dp[sm-j*szs[i-1]][i-1]);
                }
            }
        }
        return dp[total][n] < MAX ? dp[total][n] : -1;
    }
};

#undef M_PI
#undef MOD
#undef all
#undef sz

// BEGIN CUT HERE
#include <cstdio>
#include <ctime>
#include <iostream>
#include <string>
#include <vector>
namespace moj_harness {
	using std::string;
	using std::vector;
	int run_test_case(int);
	void run_test(int casenum = -1, bool quiet = false) {
		if (casenum != -1) {
			if (run_test_case(casenum) == -1 && !quiet) {
				std::cerr << "Illegal input! Test case " << casenum << " does not exist." << std::endl;
			}
			return;
		}
		
		int correct = 0, total = 0;
		for (int i=0;; ++i) {
			int x = run_test_case(i);
			if (x == -1) {
				if (i >= 100) break;
				continue;
			}
			correct += x;
			++total;
		}
		
		if (total == 0) {
			std::cerr << "No test cases run." << std::endl;
		} else if (correct < total) {
			std::cerr << "Some cases FAILED (passed " << correct << " of " << total << ")." << std::endl;
		} else {
			std::cerr << "All " << total << " tests passed!" << std::endl;
		}
	}
	
	int verify_case(int casenum, const int &expected, const int &received, std::clock_t elapsed) { 
		std::cerr << "Example " << casenum << "... "; 
		
		string verdict;
		vector<string> info;
		char buf[100];
		
		if (elapsed > CLOCKS_PER_SEC / 200) {
			std::sprintf(buf, "time %.2fs", elapsed * (1.0/CLOCKS_PER_SEC));
			info.push_back(buf);
		}
		
		if (expected == received) {
			verdict = "PASSED";
		} else {
			verdict = "FAILED";
		}
		
		std::cerr << verdict;
		if (!info.empty()) {
			std::cerr << " (";
			for (size_t i=0; i<info.size(); ++i) {
				if (i > 0) std::cerr << ", ";
				std::cerr << info[i];
			}
			std::cerr << ")";
		}
		std::cerr << std::endl;
		
		if (verdict == "FAILED") {
			std::cerr << "    Expected: " << expected << std::endl; 
			std::cerr << "    Received: " << received << std::endl; 
		}
		
		return verdict == "PASSED";
	}

	int run_test_case(int casenum__) {
		switch (casenum__) {
		case 0: {
			int sizes[]               = { 2, 3 };
			int total                 = 6;
			int expected__            = 2;

			std::clock_t start__      = std::clock();
			int received__            = PackageSizes().getMinimum(vector <int>(sizes, sizes + (sizeof sizes / sizeof sizes[0])), total);
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}
		case 1: {
			int sizes[]               = { 6, 9, 20 };
			int total                 = 8;
			int expected__            = -1;

			std::clock_t start__      = std::clock();
			int received__            = PackageSizes().getMinimum(vector <int>(sizes, sizes + (sizeof sizes / sizeof sizes[0])), total);
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}
		case 2: {
			int sizes[]               = { 2, 5, 12 };
			int total                 = 7;
			int expected__            = 2;

			std::clock_t start__      = std::clock();
			int received__            = PackageSizes().getMinimum(vector <int>(sizes, sizes + (sizeof sizes / sizeof sizes[0])), total);
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}
		case 3: {
			int sizes[]               = { 2, 5, 12 };
			int total                 = 0;
			int expected__            = 0;

			std::clock_t start__      = std::clock();
			int received__            = PackageSizes().getMinimum(vector <int>(sizes, sizes + (sizeof sizes / sizeof sizes[0])), total);
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}
		case 4: {
			int sizes[]               = { 3, 2, 7, 3 };
			int total                 = 10;
			int expected__            = 2;

			std::clock_t start__      = std::clock();
			int received__            = PackageSizes().getMinimum(vector <int>(sizes, sizes + (sizeof sizes / sizeof sizes[0])), total);
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}

		// custom cases

/*      case 5: {
			int sizes[]               = ;
			int total                 = ;
			int expected__            = ;

			std::clock_t start__      = std::clock();
			int received__            = PackageSizes().getMinimum(vector <int>(sizes, sizes + (sizeof sizes / sizeof sizes[0])), total);
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}*/
/*      case 6: {
			int sizes[]               = ;
			int total                 = ;
			int expected__            = ;

			std::clock_t start__      = std::clock();
			int received__            = PackageSizes().getMinimum(vector <int>(sizes, sizes + (sizeof sizes / sizeof sizes[0])), total);
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}*/
/*      case 7: {
			int sizes[]               = ;
			int total                 = ;
			int expected__            = ;

			std::clock_t start__      = std::clock();
			int received__            = PackageSizes().getMinimum(vector <int>(sizes, sizes + (sizeof sizes / sizeof sizes[0])), total);
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}*/
		default:
			return -1;
		}
	}
}

#include <cstdlib>
int main(int argc, char *argv[]) {
	if (argc == 1) {
		moj_harness::run_test();
	} else {
		for (int i=1; i<argc; ++i)
			moj_harness::run_test(std::atoi(argv[i]));
	}
}
// END CUT HERE
