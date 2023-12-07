#!/bin/bash

os="undefined"
file="undefined"
output="undefined"
compiler_args="undefined"
processed_args=$@

if [[ "$OSTYPE" == "darwin"* ]]; then
    os="macos"
elif [[ "$OSTYPE" == "linux"* ]]; then
    os="linux"
elif [[ "$OSTYPE" == "win32"* ]]; then
    os="windows"
fi

echo "$@"

compile() {
    echo "$os"
    if [[ $os == "macos" ]]; then
        output=$(clang $file $args)
        echo $output
    
    elif [[ $os == "linux" ]]; then
        gcc || echo "Error"
    fi
}

#first argument
case "$1" in
    "c"|"compile")
    echo "compiling"
    ;;

    *)
    echo "Usage: comps {command} {file.cpp/c} {arguments}"
    ;;
esac

#file
case "$2" in
    *".c"|*".cpp")
        echo "Compiling $2"
        file="$2"
        # compile
    ;;

    *)
        echo "Can only compile .cpp/.c files."
    ;;
esac

echo $processed_args

for arg in $processed_args; do
    if [[ $arg == "--"* ]]; then
        echo "$arg has a --"
    else
        echo "$arg has no --"
    fi
done

# case "$3" in
#     "--"*|"-"*)
#     compiler_args="$3"
#     ;;

#     *)
#     case "$3" in
#         "arm64")
#         compiler_args="$compiler_args $3"
#         esac

# esac



# echo "You ara on a $os machine. arguem"