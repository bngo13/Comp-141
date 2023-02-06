#include <iostream>
#include <regex>
#include <string>

using namespace std;

int main() {
    string input;
    cin >> input;


    regex int_reg("^(!-)?[0-9]*$");
    regex punct_reg("^[+,*,(,),.]$");

    if (regex_search(input, int_reg)) {
        cout << "number" << endl;
    } else if (regex_search(input, punct_reg)) {
        cout << "symbol" << endl;
    } else {
        cout << "wrong input. skill issue" << endl;
    }
    return 0;
}















































