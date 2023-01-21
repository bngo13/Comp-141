// Brian and Aaron
//
#include <iostream>
// /* min max array */

long* arrayscan(long array[], long minmax[], int i, int array_length) {

    if (i >= array_length) {
        return minmax;
    }

    if (array[i] < minmax[0]) minmax[0] = array[i];
    if (array[i] > minmax[1]) minmax[1] = array[i];

    return arrayscan(array, minmax, i + 1, array_length);
}

// For testing purposes
int main() {
    int const len = 16;
    long array[len] = {3, 6, 5, 4, 7, 9, 1, 8, 7, 6, 5, 3, 4, 5, 15, 5};

    long* minmax = new long;
    minmax[0] = array[0];
    minmax[1] = array[1];
    minmax = arrayscan(array, minmax, 1, len);

    std::cout << minmax[0] << std::endl;
    std::cout << minmax[1] << std::endl;


    return 0;
}
