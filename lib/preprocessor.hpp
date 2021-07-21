#pragma once

#include <string>
#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <curl/curl.h>
#include <filesystem>

#if __cplusplus >= 201703L

#ifdef __linux__
#define MKDIR(dir) std::filesystem::create_directories(dir);
#define DLEXTENSION .so
#elif __WIN32 || __WIN64
#define MKDIR(dir) std::filesystem::create_directories(dir);
#define DLEXTENSION .dll
#elif __unix || __unix__
#define MKDIR(dir) std::filesystem::create_directories(dir);
#define DLEXTENSION .so
#elif __APPLE__ || __MACH__
#define MKDIR(dir) std::filesystem::create_directories(dir);
#define DLEXTENSION .dylib
#elif __FreeBSD__
#define MKDIR(dir) std::filesystem::create_directories(dir);
#define DLEXTENSION .so
#else
#error "Sorry but your OS is not supported. But don't worry, you just need to change a few stuffs in source code."
#endif

#else
#error "Sorry but you need to use C++17 standard"
#endif