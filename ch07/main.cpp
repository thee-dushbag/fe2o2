#include <iostream>
#include <variant>
#include <vector>

using Age = int;
using Name = const char *;

struct Empty {};

struct Rgb {
  int r : 8, g : 8, b : 8;
};

struct School {
  const char *name, *country;
};

using Payload = std::variant<Empty, Age, Rgb, Name, School>;

int main() {
  std::vector<Payload> const payloads{60,
                                      "Kenya",
                                      Rgb{
                                          .r = 19,
                                          .g = 34,
                                          .b = 56,
                                      },
                                      "Simon",
                                      School{
                                          .name = "Jomo Kenya University",
                                          .country = "Kenya",
                                      },
                                      "Faith",
                                      School{
                                          .name = "Harvard University",
                                          .country = "USA",
                                      },
                                      "Lydia"};
  struct PayloadVisitor {
    void operator()(Name const &name) const {
      std::cout << "Your name is " << name << "!\n";
    }
    void operator()(Rgb const &rgb) const {
      std::cout << "RGB(r=" << rgb.r << ", g=" << rgb.g << ", b=" << rgb.b
                << ")\n";
    }
    void operator()(School const &school) const {
      std::cout << "The school " << school.name << " can be found in "
                << school.country << ".\n";
    }
    void operator()(Age const &age) const {
      std::cout << "You are " << age << " years old!\n";
    }
    void operator()(Empty const &e) const {}
  };

  std::cout << "sizeof(Payload) = " << sizeof(Payload) << '\n';

  for (auto &payload : payloads)
    std::visit(PayloadVisitor(), payload);
}
