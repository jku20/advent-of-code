#include <bits/stdc++.h>
#define N 1001
using namespace std;

int T;

int dx[] = {1, -1, 0, 0};
int dy[] = {0, 0, -1, 1};

int bg[N][N];
int g[N][N];
bool vis[N][N];

bool val(int x, int y, int n, int m) {
    return x >= 0 && x < n && y >= 0 && y < m;
}

void solve() {
    freopen("input", "r", stdin);

    char buf[N];
    int n = 0, m = 0;
    while (scanf("%s", buf) != EOF) {
        char *c = buf;
        while (*c) bg[n][c - buf - 1] = *(c++) - '0';
        m = c - buf;
        n++;
    }

    for (int i = 0; i < 5*n; i++) {
        for (int j = 0; j < 5*m; j++) {
            int nr = bg[i % n][j % m] + i / n + j / m;
            g[i][j] = nr > 9 ? (nr % 10 + 1) : nr;
        }
    }

    n *= 5, m *= 5;

    priority_queue<pair<int, pair<int,int>>> q;
    q.push({0, {0, 0}});
    while (q.size()) {
        auto t = q.top(); q.pop();
        int w = t.first; auto [x, y] = t.second;
        if (!val(x, y, n, m) || vis[x][y]) continue;

        vis[x][y] = true;
        w -= g[x][y];

        if (x == n-1 && y == m-1)
            cout << -w - g[0][0] << endl;

        for (int i = 0; i < 4; i++) {
            int nx = x + dx[i], ny = y + dy[i];
            q.push({w, {nx, ny}});
        }
    }
}

int main() {
    //cin >> T;
    T = 1;
    while(T--) solve();
    return 0;
}
