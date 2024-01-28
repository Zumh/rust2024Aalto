#include <iostream>

using namespace std;

int main() {
  int n = 10;
  int firstNumber = 0;
  int secondNumber = 1;
  int sumOfPrevious;

  cout << "Fibonacci series first 10 numbers: ";

  for (int i = 1; i <= n; ++i) {
    sumOfPrevious = firstNumber + secondNumber;
    cout << sumOfPrevious;
    if (i < n) {
      cout << ", ";
    }

    firstNumber = secondNumber;
    secondNumber = sumOfPrevious;
  }

  cout << " ";
  return 0;
}
