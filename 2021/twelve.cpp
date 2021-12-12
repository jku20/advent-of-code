#include <bits/stdc++.h>
using namespace std;

int T;

map<string, vector<string>> adj;
map<string, int> ways;

void solve() {
    freopen("input", "r", stdin);

    char t[15];
    while (scanf("%s", t) != EOF) {
        char *p = strtok(t, "-");
        string a = string(p);
        p = strtok(NULL, "-");
        string b = string(p);
        adj[a].push_back(b);
        adj[b].push_back(a);
    }

    queue<vector<string>> q;
    q.push({"start"});
    while(q.size()) {
        vector<string> s = q.front(); q.pop();
        string e = s.back();
        ways[e] += 1;
        for (auto u : adj[e]) {
            if ((u == "start" || u == "end") && count(s.begin(), s.end(), u) > 0) continue;
            if (islower(*u.begin()) && count(s.begin(), s.end(), u) != 0) {
                if (!all_of(s.begin(), s.end(), [&s](string h) {
                        if (islower(*h.begin())) return count(s.begin(), s.end(), h) == 1;
                        return true;
                    })) continue;
            }
            vector<string> t = vector<string>(s);
            t.push_back(u);
            q.push(t);
        }
    }
    cout << ways["end"] << endl;
}

int main() {
    //cin >> T;
    T = 1;
    while(T--) solve();
    return 0;
}
