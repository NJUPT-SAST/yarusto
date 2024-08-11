#pragma once

#include <string>

struct CmdArgs {
    std::string inputPath;
    std::string outputPath;
};

CmdArgs parseCmdArgs(int argc, char** argv);