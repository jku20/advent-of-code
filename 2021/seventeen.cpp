#include <bits/stdc++.h>

using namespace std;

int T;

//hard code input
int LX = 144, RX = 178, LY = -100, RY = -76;
//int LX = 20, RX = 30, LY = -10, RY = -5;

//only for part 1
int hpos(int vy) {
    return vy * (vy + 1) / 2;
}

bool works(int vx, int vy) {
    //most amount of steps is 500
    int x = 0, y = 0;
    for (int i = 0; i < 500; i++) {
        x += vx, y += vy;
        vy -= 1, vx -= (0 < vx) - (vx < 0);
        if (x >= LX && x <= RX && y >= LY && y <= RY) return true;
    }
    return false;
}

void solve() {
    freopen("input", "r", stdin);

    //iterate over possible velocities
    int ans = 0;
    int mnx = -200, mxx = 200;
    int mny = -200, mxy = 200;
    for (int i = mnx; i < mxx; i++) 
        for (int j = mny; j < mxy; j++) 
            if (works(i, j)) 
                ans++;
    cout << ans << endl;

}

int main() {
    //cin >> T;
    T = 1;
    while(T--) solve();
    return 0;
}
