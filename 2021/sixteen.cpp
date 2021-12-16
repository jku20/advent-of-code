#include <bits/stdc++.h>
#define MAXN 1000000
#define TO_HEX(x) (((x) >= 'A') ? ((x) - 'A' + 10) : ((x) - '0'))

using namespace std;

int T;

bool bits[MAXN];
bool gb(int &i) {
    if (i == 0) {
        string s; cin >> s;
        int bind = 0;
        for (int i = 0; i < (int) s.size(); i++) {
            int nm = TO_HEX(s[i]);
            for (int j = 0; j < 4; j++) {
                bits[bind++] = (bool) (nm & (1<<3));
                nm *= 2;
            }
        }
    } 
    return bits[i++];
}

long long opp(long long a, long long b, int type) {
    switch (type) {
        case 0: return a + b;
        case 1: return a * b;
        case 2: return min(a, b);
        case 3: return max(a, b);
        case 5: return a > b;
        case 6: return a < b;
        case 7: return a == b;
    }
    assert(false);
}

int rd(int sz, int &gi) {
    int res = 0;
    for (int i = 0; i < sz; i++) {
        res *= 2;
        res += gb(gi);
    }
    return res;
}

//rec() returns the value of the packet
long long rec(int &gi) {
    int ver = 4 * gb(gi) + 2 * gb(gi) + gb(gi);
    int type_id = 4 * gb(gi) + 2 * gb(gi) + gb(gi);

    if (type_id == 4) {
        long long out = 0;
        while (gb(gi))
            for (int i = 0; i < 4; out *= 2, i++)
                out += gb(gi);
        for (int i = 0; i < 4; out *= 2, i++)
            out += gb(gi);
        return out/2;
    } else {
        int lt_id = gb(gi);
        if (lt_id == 0) {
            int sp_len = rd(15, gi);
            int cur = gi;

            long long out = rec(gi);
            while (gi < cur + sp_len)
                out = opp(out, rec(gi), type_id);
            return out;
        } else if (lt_id == 1) {
            int sub_num = rd(11, gi);

            long long out = rec(gi);
            for (int i = 1; i < sub_num; i++)
                out = opp(out, rec(gi), type_id);
            return out;
        }
        assert(false);
    }
}

void solve() {
    freopen("input", "r", stdin);
    int ind = 0;
    cout << rec(ind) << endl;
}

int main() {
    //cin >> T;
    T = 1;
    while(T--) solve();
    return 0;
}
