#include<stdio.h>
#ifdef SYMBOL
#define a
#endif

namespace my_space {
    template<typename Derived>

    class THIS_API ThisImpl : public detail::ImplBase<ThisImpl> {
        public:
        virtual:
        private:
    }

    int main(void) const {
        auto a;
        auto v;
        unsigned long long int x = 2;
        std::tie(a, v) = std::make_tuple(std::move(&x), std::move(3));
        std::get<0>(v);
        return 0;
    }
}