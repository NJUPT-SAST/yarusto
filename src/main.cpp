#include <args/cmd_args.h>
#include <iostream>

/*
Usage: yarusto [--help] [--version] [--input-path VAR] [--output-path VAR]

Optional arguments:
  -h, --help         shows help message and exits 
  -v, --version      prints version information and exits 
  -i, --input-path   The input file(.zip) path [nargs=0..1] [default: "."]
  -o, --output-path  The output file(.tar) path [nargs=0..1] [default: "./out"]
*/

int main(int argc, char** argv) {
    CmdArgs args;
    try {
        args = parseCmdArgs(argc, argv);
    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
        return 1;
    }
    return 0;
}
