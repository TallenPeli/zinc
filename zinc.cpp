#include <iostream>
#include <fstream>
#include <string>
#include <unistd.h>
#include <vector>
#include <cstdlib>
#include <functional>

using namespace std;

bool keepTranslation = false;
bool verboseOutput = false;
bool isZincFile = false;

string addLibraries(){
    string code;
    code.append("\n//Standard Zinc functions from zincstd\n");
    code.append("template <typename T>\nvoid print(const T& input){std::cout << input;}\n");
    code.append("template <typename T>\nvoid println(const T& input = T()) { if constexpr (!std::is_void_v<T>) std::cout << input << std::endl; }\n");
    code.append("void println(){std::cout << std::endl;}\n");
    code.append("void newline(){std::cout << std::endl;}\n");
    code.append("std::string input(std::string prompt){std::string Input;std::cout << prompt;std::cin >> Input;return Input;}\n");
    code.append("std::string getline(std::string prompt){std::string Input;std::cout << prompt;getline(std::cin, Input);return(Input);}\n");
    
    return code;
}

void flagHandler(int flagPos, char **flag){
    string file = flag[1];
    if (file.substr(0, 2) == "./") {
        file = file.substr(2);
    }

    if (flagPos >= 3) {
        string arg2 = flag[2];
        if (arg2 == "-k" || arg2 == "--keep-translation") {
            keepTranslation = true;
        }else if(arg2 == "-v" || arg2 == "--verbose") {
            verboseOutput = true;
        }
    }
    if (flagPos >= 4) {
        string arg3 = flag[3];
        if (arg3 == "-k" || arg3 == "--keep-translation") {
            keepTranslation = true;
        }else if(arg3 == "-v" || arg3 == "--verbose") {
            verboseOutput = true;
        }
    }

    if(verboseOutput){
        cout << "| Verbose output: [True]" << endl;
        cout << "| Keep translation: [" << (keepTranslation ? "True" : "False") << "]" << endl;
    }
}

void runCode(){
    if(verboseOutput){
        cout << "| Running the program..." << endl;
    }
    system("./zinc_output");
}

void compileCode(){
    int compileResult = system("g++ -o zinc_output zinc_to.cpp");

    if (compileResult == 0) {
        if(verboseOutput){
            cout << "| Compilation successful." << endl;
        }
        runCode();
        if(keepTranslation){
            if(verboseOutput){
                cout << "\n| Kept c++ translation.\n";
            }
        }else{
            system("rm ./zinc_to.cpp");
        }

    } else {
        // Compilation failed
        cerr << "| Compilation failed. Please check the code for errors." << std::endl;
    }
}

string join(const vector<string>& elements, const string& delimiter) {
    string result;
    for (size_t i = 0; i < elements.size(); ++i) {
        result += elements[i];
        if (i < elements.size() - 1) {
            result += delimiter;
        }
    }
    return result;
}

int main(int argc, char **argv) {
    // Define functions
    flagHandler(argc, argv);

    char cwd[1024];

    string argument = argv[1];

    string tmpcwd = getcwd(cwd, sizeof(cwd));
    string fullcwd = tmpcwd + "/" + argument;

    if(verboseOutput){
        cout << "| Full path [" << fullcwd << "]" << endl;
    }

    ifstream zincFile(fullcwd.c_str());
    if (!zincFile.is_open()) {
        cerr << "Error: File '" << fullcwd << "' does not exist." << endl;
        return 1;
    }

    // Create a vector to store the translated C++ code
    std::vector<std::string> translatedCode;

    translatedCode.push_back("#include <iostream>\n#include <string>\n#include <type_traits>");

    // Read and process each line of the input script
    std::string line;

    while (std::getline(zincFile, line)) {
        if(line == "using zincstd;" && !isZincFile){
            isZincFile = true;
            line.erase();
            line.append(addLibraries());
        }else if (!isZincFile){
            cout << "Error : Not a valid ZINC file [2]" << endl;
            exit(2);
        }

        // Translate 'fn ' to 'int ' for function declarations
        size_t fnPos = line.find("fn");
        if (fnPos == 0 || (fnPos != std::string::npos && (line[fnPos - 1] == ' ' || line[fnPos - 1] == ';' || line[fnPos - 1] == '}'|| line[fnPos - 1] == '{'))) {
            line.replace(fnPos, 2, "void ");
        }

        // Translate main
        size_t mainPos = line.find("main()");
        if (mainPos == 0 || (mainPos != std::string::npos && (line[mainPos - 1] == ' ' || line[mainPos - 1] == ';' || line[mainPos - 1] == '}'|| line[mainPos - 1] == '{'))) {
            line.replace(mainPos, 0, "int ");
        }

        size_t stringPos = line.find("string");
        if (stringPos == 0 || (stringPos != std::string::npos && (line[stringPos - 1] == ' ' || line[stringPos - 1] == ';' || line[stringPos - 1] == '}'|| line[stringPos - 1] == '{'))) {
            line.replace(stringPos, 6, "std::string");
        }

        // Translate loops
        size_t loopPos = line.find("loop(");
        if (loopPos == 0 || loopPos != std::string::npos && (line[loopPos - 1] == ' ' || line[loopPos - 1] == ';' || line[loopPos - 1] == '}' || line[loopPos - 1] == '{')) {
            // Check if there is an opening parenthesis
            size_t openParenthesisPos = line.find("(", loopPos);

            if (openParenthesisPos != std::string::npos) {
                // Extract the argument inside the loop function call
                size_t variableCommaSplitPos = line.find(",", openParenthesisPos);
                if (variableCommaSplitPos != std::string::npos) {
                    string loopArgument = line.substr(openParenthesisPos + 1, variableCommaSplitPos - openParenthesisPos - 1);

                    // Check the nesting level to determine the loop variable name
                    
                    size_t closeParenthsisPos = line.find(")", variableCommaSplitPos);
                    
                    string loopVarName = line.substr(variableCommaSplitPos + 1, closeParenthsisPos - variableCommaSplitPos - 1);

                    // Replace the loop function call with a for loop
                    line.replace(loopPos, closeParenthsisPos - loopPos + 1,
                        "for(int " + loopVarName + " = 0; " + loopVarName + " < " + loopArgument + "; " + loopVarName + "++)");
                }
            }
        }

        // Translate lists
        size_t listPos = line.find("list ");
        if (listPos != std::string::npos) {
            // Find the list name 
            size_t spaceAfterList = line.find("[", listPos + 5);
            if (spaceAfterList != std::string::npos) {
                string listName = line.substr(listPos + 5, spaceAfterList - listPos - 5);

                // Find the opening bracket of the list
                size_t openBracketPos = line.find("[");
                if (openBracketPos != std::string::npos) {
                    // Find the closing bracket of the list
                    size_t closeBracketPos = line.find("]", openBracketPos);
                    if (closeBracketPos != std::string::npos) {
                        // Extract the list contents
                        string listContents = line.substr(openBracketPos + 1, closeBracketPos - openBracketPos - 1);

                        // Tokenize the list contents by comma
                        vector<string> items;
                        size_t start = 0;
                        size_t commaPos;
                        while ((commaPos = listContents.find(",", start)) != std::string::npos) {
                            items.push_back(listContents.substr(start, commaPos - start));
                            start = commaPos + 2;  // Skip the comma and the space
                        }
                        // Add the last item
                        items.push_back(listContents.substr(start));

                        // Generate the C++ array declaration
                        line = "std::string " + listName + "[" + to_string(items.size()) + "] = {" + join(items, ", ") + "};";
                    }
                }
            }
        }

        // Add the translated line to the vector
        translatedCode.push_back(line);
    }

    // Close the input script file
    zincFile.close();

    // Output the translated C++ code to a file
    std::ofstream outputFile("zinc_to.cpp");

    if (!outputFile.is_open()) {
        std::cerr << "Error: Unable to create output file." << std::endl;
        return 1;
    }

    for (const std::string& translatedLine : translatedCode) {
        outputFile << translatedLine << std::endl;
    }

    compileCode();

    // Close the output file
    outputFile.close();

    // Compile the saved C++ code

    return 0;
}