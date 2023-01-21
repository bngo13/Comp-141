#include <iostream>
#include <vector>
using namespace std;

vector<pair<string, string>> dictentry(vector<pair<string, string>> dictionary, string key, string value, int i) {
    if (i >= dictionary.size()) {
        dictionary.push_back(pair<string, string>(key, value));
        return dictionary;
    } else if (dictionary[i].first == key) {
        dictionary[i].second = value;
        return dictionary;
    }

    return dictentry(dictionary, key, value, i + 1);
}

int main() {
    vector<pair<string, string>> ab;
    ab = dictentry(ab, "a", "b", 0);
    ab = dictentry(ab, "c", "d", 0);
    cout << ab.back().first << endl;

    return 0;
}
