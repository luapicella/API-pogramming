#include <iostream>

class Test{
    public:
    Test(){
        std::cout << "Un oggetto di tipo Test è stato costruito all'indirizzo" << this << std::endl;
    }
    ~Test(){
        std::cout << "Un oggetto di tipo Test è stato distrutto all'indirizzo" << this << std::endl;
    }
};

int m() {
    for (int i = 0; i<5; i++){
        Test t;
        if (i == 1) throw std::exception();
        std::cout << "----" << std::endl;
    }
}

int main(int, char**){
    try
    {
        m();
    }
    catch(const std::exception& e)
    {
        std::cerr << e.what() << '\n';
    }
    
}
