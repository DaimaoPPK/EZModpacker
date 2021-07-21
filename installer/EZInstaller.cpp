#include "preprocessor.hpp"

extern std::string line;

int main(int argc, char const *argv[]){
    for (size_t i = 1; i < argc; i++)
    {
        std::ifstream file(argv[i]);
        while (std::getline(file, line)) {
            Interpret();
        }
        file.close();
        ProcessReport();
    }

    if (argc < 2){
        printf("Usage: EZInstaller [modpackinfo_file]\n");
    }
}
