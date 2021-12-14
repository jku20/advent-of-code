#include <bits/stdc++.h>
using namespace std;

int T;

map<pair<char,char>, char> ft;
map<pair<char,char>, long long> prs;

void solve() {
    freopen("input", "r", stdin);
    int N = 40;

    string s; cin >> s;

    char a[100], b;
    while (scanf("%s -> %c", a, &b) != EOF)
        ft[{a[0], a[1]}] = b;

    for (int i = 0; i < (int) s.size()-1; i++)
        prs[{s[i], s[i+1]}]++;

    for (int i = 0; i < N; i++) {
        auto pre_prs = map(prs);
        for (auto &[_, v] : prs) 
            v = 0;
        for (auto [p, nm] : pre_prs) {
            pair<char,char> np1 = {p.first, ft[p]}, np2 = {ft[p], p.second};
            prs[np1] += nm;
            prs[np2] += nm;
        }
    }

    vector<long long> els(26);
    for (auto &[k, i] : prs) {
        els[k.first - 'A'] += i;
        els[k.second - 'A'] += i;
    }
    els[s.back()-'A']++;
    els[s.front()-'A']++;
    for (auto &i : els) i/=2;

    sort(els.begin(), els.end());
    long long mn = -1;
    for (auto i : els) {
        if (i != 0) {
            mn = i;
            break;
        }
    }
    long long mx = els.back();

    cout << mx - mn << endl;
}

int main() {
    //cin >> T;
    T = 1;
    while(T--) solve();
    return 0;
}
