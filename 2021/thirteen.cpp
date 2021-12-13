#include <bits/stdc++.h>
using namespace std;
#define N 2000

int T;

bool g[N][N];

void fold(char axis, int p) {
    if (axis == 'x') {
        for (int i = p; i < N; i++) {
            for (int j = 0; j < N; j++) {
                if (g[i][j]) {
                    g[p-(i-p)][j] = true;
                    g[i][j] = false;
                }
            }
        }
    } else if (axis == 'y') {
        for (int i = 0; i < N; i++) {
            for (int j = p; j < N; j++) {
                if (g[i][j]) {
                    g[i][p-(j-p)] = true;
                    g[i][j] = false;
                }
            }
        }
    }
}

void solve() {
    freopen("input", "r", stdin);

    vector<pair<int,int>> dots;
    int x, y;
    while (scanf("%d,%d", &x, &y) != 0) {
        g[x][y] = true;
    }
    vector<pair<char, int>> l;
    char axis;
    while (scanf("fold along %c=%d\n", &axis, &x) != EOF) {
        l.push_back({axis, x});
    }

    for (auto [a, b] : l) {
        fold(a, b);
    }

    for (int i = 0; i < 6; i++) {
        for (int j = 0; j < 40; j++) {
            cout << (g[j][i] ? '#' : ' ');
        }
        cout << '\n';
    }
}

int main() {
    //cin >> T;
    T = 1;
    while(T--) solve();
    return 0;
}
