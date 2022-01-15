#include <bits/stdc++.h>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;
using namespace __gnu_pbds;
template<class T> using ordered_set = tree<T, null_type, less<T>, rb_tree_tag, tree_order_statistics_node_update>;

int rand(int a, int b){
  return a + rand()%(b-a+1);
}

int main(int argc, char* argv[]){
  srand(atoi(argv[1]));
  cout<<1<<'\n';
  int n = rand(1, 5);
  cout<<n<<'\n';
  for(int i=0; i<n; i++){
    cout<<rand(1, 5)<<' ';
  }
  cout<<'\n';
  return 0;
}

