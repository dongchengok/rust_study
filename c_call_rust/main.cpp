#include <iostream>

extern "C" void hello_from_rust();
extern "C" void hello_from_xxx();

extern "C" int32_t foo(int32_t a, int32_t b);

int main(int, char**) {
    int a = 0;
    std::cout << "Hello, world!\n";
    int b = foo(1,2);
    std::cout << b << std::endl;
}
