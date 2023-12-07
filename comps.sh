#!/bin/bash

os=""
file=""
output=""
compiler_args=""
processed_args=( "$@" )
mode=""
usage="Usage: comps {command} {file.cpp/c} {arguments}"
run=false

if [[ "$OSTYPE" == "darwin"* ]]; then
    os="macos"
elif [[ "$OSTYPE" == "linux"* ]]; then
    os="linux"
elif [[ "$OSTYPE" == "win32"* ]]; then
    os="windows"
fi

compile() {
    if [[ $os == "macos" ]]; then
        output=$(clang "$file" "$compiler_args")
        echo "$output"
    
    elif [[ $os == "linux" ]]; then
        {
            output_name="$(basename "${file%.*}")"
            command=(-o "$output_name" "$file" $compiler_args)
            g++ "${command[@]}"

            if [[ $run == true ]]; then
                ./$output_name
            fi
        } || {
            echo "Error"
        }
    fi
}

#first argument
case "$1" in
    "c"|"compile")
        mode="c"
        ;;
    "cr")
        mode="c"
        run=true
        ;;

    *)
    echo "$usage"
    exit 1
    ;;
esac

#file
case "$2" in
    *".c"|*".cpp")
        if [[ "$mode" == "c" ]]; then
            echo "Compiling $2 (linux)"
            file="$2"
            compile

        else
            echo "$usage"
        fi
    ;;

    *)
        echo "Can only compile .cpp/.c files."
        exit 1
    ;;
esac

# echo  "${processed_args[@]}"

for arg in "${processed_args[@]}"; do
    if [[ $arg == "--"* ]]; then
        compiler_args="$compiler_args $arg"
    else

        echo ""
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