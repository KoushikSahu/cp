#include <bits/stdc++.h>
using namespace std;

int rand(int a, int b){
    return a + rand()%(b-a+1);
}

int main(int argc, char* argv[]){
    srand(atoi(argv[1]));
    int t = 1;
    cout<<t<<'\n';
    int n = rand(2, 100000);
    cout<<n<<'\n';
    for(int i=0; i<n; i++) cout<<rand(-1000000000, 1000000000)<<' ';
    cout<<'\n';
    return 0;
}

