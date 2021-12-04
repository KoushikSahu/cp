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

class SumUnique {
    public:
    int getSum(vector <int> values) {
        set<int> unq;
        for(int i: values) unq.insert(i);
        ll sm = 0;
        for(int i: unq) sm += i;
        return sm;
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
			int values[]              = { 1, 2, 3, 4, 5 };
			int expected__            = 15;

			std::clock_t start__      = std::clock();
			int received__            = SumUnique().getSum(vector <int>(values, values + (sizeof values / sizeof values[0])));
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}
		case 1: {
			int values[]              = { 1, 2, 3, 4, 3 };
			int expected__            = 10;

			std::clock_t start__      = std::clock();
			int received__            = SumUnique().getSum(vector <int>(values, values + (sizeof values / sizeof values[0])));
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}
		case 2: {
			int values[]              = { 1, 1, 1, 1, 1, 1, 1, 1, 1 };
			int expected__            = 1;

			std::clock_t start__      = std::clock();
			int received__            = SumUnique().getSum(vector <int>(values, values + (sizeof values / sizeof values[0])));
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}
		case 3: {
			int values[]              = { 18, 22, 9, 1, 7, 18, 13, 49, 13, 9, 49, 49, 7 };
			int expected__            = 119;

			std::clock_t start__      = std::clock();
			int received__            = SumUnique().getSum(vector <int>(values, values + (sizeof values / sizeof values[0])));
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}

		// custom cases

/*      case 4: {
			int values[]              = ;
			int expected__            = ;

			std::clock_t start__      = std::clock();
			int received__            = SumUnique().getSum(vector <int>(values, values + (sizeof values / sizeof values[0])));
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}*/
/*      case 5: {
			int values[]              = ;
			int expected__            = ;

			std::clock_t start__      = std::clock();
			int received__            = SumUnique().getSum(vector <int>(values, values + (sizeof values / sizeof values[0])));
			return verify_case(casenum__, expected__, received__, clock()-start__);
		}*/
/*      case 6: {
			int values[]              = ;
			int expected__            = ;

			std::clock_t start__      = std::clock();
			int received__            = SumUnique().getSum(vector <int>(values, values + (sizeof values / sizeof values[0])));
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
