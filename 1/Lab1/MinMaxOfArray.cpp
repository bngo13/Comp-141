// Brian and Aaron
//
#include <iostream>
#include <vector>
// /* min max array */

std::vector<long> arrayscan(long array[], std::vector<long> minmax, int i, int array_length) {
    // minmax[0] = smallest number
    // minmax[1] = largest number

    if (i >= array_length) {
        return minmax;
    }

    if (array[i] < minmax.at(0)) minmax[0] = array[i];
    if (array[i] > minmax.at(1)) minmax[1] = array[i];

    return arrayscan(array, minmax, i + 1, array_length);
}

// For testing purposes
int main() {
    int const len = 16;
    long array[len] = {3, 6, 5, 4, 7, 9, 1, 8, 7, 6, 5, 3, 4, 5, 15, 5};

    std::vector<long> minmax;
    minmax.push_back(array[1]);
    minmax.push_back(array[0]);
    minmax = arrayscan(array, minmax, 1, len);

    std::cout << minmax.at(0) << std::endl;
    std::cout << minmax.at(1) << std::endl;


    return 0;
}
