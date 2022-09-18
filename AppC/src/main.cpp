#include <iostream>

#include "core_c.h"
#include "core_r.h"

#include "wrapper_c.h"
#include "wrapper_r.h"

using namespace std;



int main() {
  cout << "Hello world!" << endl;
  cout << "Adding: " << add(4, 5) << endl;
  cout << "Product: " << product(4, 5) << endl;
  cout << "Subtracting: " << sub(10, 5) << endl;
  cout << "Ratio: " << ratio(10, 5) << endl;

  auto c = both_c(4, 5);
  cout << "Both from C: " << c.sum << ',' << c.product << endl;

  auto d = both_r(4, 5);
  cout << "Both from Rust: " << d.sum << ',' << d.product << endl;
  return 0;
}

