#include <string>
#include <fstream>
#include <iostream>
#include <bits/stdc++.h> 


bool sameContent(::std::string *words1, ::std::string *words2) {
  ::std::sort(words1->begin(), words1->end());
  ::std::sort(words2->begin(), words2->end());

  //::std::cout << "words1: " << *words1 << '\n';
  //::std::cout << "words2: " << *words2 << '\n';

  return words1->compare(*words2) == 0;
}


int main () {
  ::std::string lines[2];
  ::std::ifstream myfile ("../input_2.txt");

  if (myfile.is_open()) {
    int i {0};
    while (getline (myfile, lines[i])) {
      //::std::cout << lines[i] << '\n';
      ++i;
    }
    myfile.close();
  } else {
    ::std::cout << "Unable to open file";
  }

  const bool result {sameContent(&lines[0], &lines[1])};
  ::std::cout << "CPP \n";
  ::std::cout << "result1 = " << ::std::boolalpha << result << ::std::endl;
}
