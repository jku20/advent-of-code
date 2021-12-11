#include <bits/stdc++.h>
using namespace std;

int T;

const int dx[] = {1, -1, 0, 0, 1, -1, 1, -1};
const int dy[] = {0, 0, 1, -1, 1, -1, -1, 1};

int octo[10][10];

bool val(int x, int y) {
    return x >= 0 && x < 10 && y >= 0 && y < 10;
}

int dfs(const int x, const int y) {
    if (!val(x, y)) return 0;
    octo[x][y] += 1;
    if (octo[x][y] != 9 + 1) return 0;
    int out = 1;
    for (int i = 0; i < 8; i++) {
        out += dfs(dx[i] + x, dy[i] + y);
    }
    return out;
}

int step() {
    int cur = 0;
    for (int j = 0; j < 10; j++) {
        for (int k = 0; k < 10; k++) {
            cur += dfs(j, k);
        }
    }
    for (int j = 0; j < 10; j++) {
        for (int k = 0; k < 10; k++) {
            if (octo[j][k] > 9) {
                octo[j][k] = 0;
            }
        }
    }
    return cur;
}

void solve() {
    freopen("input", "r", stdin);

    for (int i = 0; i < 10; i++) {
        string s; cin >> s;
        for (int k = 0; k < 10; k++) {
            octo[i][k] = s[k] - '0';
        }
    }

    int i = 1;
    while (step() != 100) {
        i++;
    }
    cout << i << endl;
}

int main() {
    //cin >> T;
    T = 1;
    while(T--) solve();
    return 0;
}
