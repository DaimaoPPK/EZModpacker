#include "preprocessor.hpp"
#include "EZLib"

std::string modpackname = "";
std::string curdir = "";
std::string line = "";
std::string dlname = "";
long dlcount;
long dircount;

std::vector<std::string> split(std::string stringToBeSplitted, std::string delimeter)
{
    std::vector<std::string> splittedString;
    int startIndex = 0;
    int  endIndex = 0;
    while( (endIndex = stringToBeSplitted.find(delimeter, startIndex)) < stringToBeSplitted.size() ){       
      std::string val = stringToBeSplitted.substr(startIndex, endIndex - startIndex);
      splittedString.push_back(val);
      startIndex = endIndex + delimeter.size();
    }
    if(startIndex < stringToBeSplitted.size()){
      std::string val = stringToBeSplitted.substr(startIndex);
      splittedString.push_back(val);
    }
    return splittedString;
}

size_t write_data(void *ptr, size_t size, size_t nmemb, void *stream){
    size_t written = fwrite(ptr, size, nmemb, (FILE *)stream);
    return written;
}

void makedir(std::string dirname){
    MKDIR(dirname);
}

void download(std::string url, std::string output, bool re){
    curl_global_init(CURL_GLOBAL_ALL);

    CURL *handle = curl_easy_init();
    FILE *fp = fopen((curdir+'/'+output).c_str(), "wb");
    std::cout << output << '\n';

    curl_easy_setopt(handle, CURLOPT_URL, url.c_str());
    curl_easy_setopt(handle, CURLOPT_NOPROGRESS, 0L);
    if (re){
        curl_easy_setopt(handle, CURLOPT_FOLLOWLOCATION, 1L);
    }
    curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION, write_data);

    if (fp){
        curl_easy_setopt(handle, CURLOPT_WRITEDATA, fp);
        curl_easy_perform(handle);
        fclose(fp);
    }
    curl_easy_cleanup(handle);
    curl_global_cleanup();
}

void CurseForgeFix(std::string splittedline[], bool re){
    splittedline[0] = splittedline[0].substr((re)?6:4);
    std::string p1 = splittedline[0].substr(0, splittedline[0].length() - 3);
    std::string p2 = splittedline[0].substr(splittedline[0].length() - 3).substr(0, 1).erase(1,1)+ splittedline[0].substr(splittedline[0].length() - 3).substr(1, 3);
    std::string fixedurl = "https://media.forgecdn.net/files/" + p1 + '/' + p2 + '/' + splittedline[1];
    std::cout << "Mod Name: " << splittedline[1] << '\n';
    std::cout << "Download Link: " << fixedurl << '\n';
    download(fixedurl, splittedline[1], re);
}

void ProcessReport(){
    std::cout<<"----------------------------------------------------------------------\n";
    std::cout<<"Process report:\n";
    std::cout<<"----------------------------------------------------------------------\n";
    std::cout<<"Created "<<dircount<<((dircount<=1)?" folder":" folders")<<'\n';
    std::cout<<"Downloaded "<<dlcount<<((dlcount<=1)?" file":" files")<<'\n';
    std::cout<<"----------------------------------------------------------------------\n"; 
    dircount = 0; dlcount = 0;
}

void Interpret(){
    std::fstream metafile(modpackname+'/'+".metadata");

    if (line.substr(0, 5) == "name:"){
        std::cout << "--------------------------------------------\n";
        std::cout << "Modpack Name: " << line.substr(5) << '\n'; 
        std::cout << "--------------------------------------------\n";
        modpackname = line.substr(5);
        makedir(modpackname);
    } else if(line.substr(0, 4) == "ver:"){
        std::cout << "Version: " << line.substr(4) << '\n';
        metafile << "ver: " << line.substr(4) << '\n';
    } else if (line[0] == '$'){
        curdir = modpackname+'/'+line.substr(1);
        makedir(curdir);
        dircount++;
    } else if (line[0] == '>'){
        dlname = line.substr(1);
    } else if(line.substr(0, 3) == "dl!"){
        download (line.substr(3), dlname, false);
    } else if (line.substr(0, 3) == "re!"){
        download(line.substr(3), dlname, true);
        dlcount++;
    } else if (line.substr(0,4) == "cfm!"){
        std::string arr[] = {split(line, "/").at(0), split(line, "/").at(1)};
        CurseForgeFix(arr, false);
        dlcount++;
    } else if (line.substr(0,6) == "recfm!"){
        std::string arr[] = {split(line, "/").at(0), split(line, "/").at(1)};
        CurseForgeFix(arr, true);
        dlcount++;
    }

    metafile.close();
}