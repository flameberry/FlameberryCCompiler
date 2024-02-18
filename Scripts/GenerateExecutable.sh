#!/bin/zsh

while getopts "i:o:" opt; do
    case $opt in
        i)
            input_path=$OPTARG
            ;;
        o)
            output_path=$OPTARG
            ;;
        \?)
            echo "Usage: $0 -i <input_assembly_path> [-o <output_executable_path>]"
            exit 1
            ;;
    esac
done

if [ -z "$input_path" ]; then
    echo "Input assembly path is required. Use -i <input_assembly_path>"
    exit 1
fi

if [ -z "$output_path" ]; then
    output_path="${input_path%.s}"
fi

as "$input_path" -o "${output_path}.o"
ld -e _main -o "$output_path" "${output_path}.o"

echo "Executable generated at: $output_path"
./"$output_path"
