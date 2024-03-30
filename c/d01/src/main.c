#include <stdio.h>
#include <stdlib.h>

#define INPUT_PATH "/mnt/c/Users/aurel/Projects/AdventOfCode/c/d01/input/input.txt"

char ASCII_OFFSET = '0';
int DIGIT_LIST[10];

// init the digit_mapper
int init() {
    int i;
    for (i = 0; i < 10; i++) {
        DIGIT_LIST[i] = i;
    }
    return 0;
}

// return the first int from the left of the line 
int get_from_left(char* line, int size) {
    int i;
    int j;
    int res = -1;
    int check;
    for (i = 0; i <= size-1; i++) {
        for (j = 0; j < 10; j++) {
            check = line[i] - ASCII_OFFSET;
            if (check == DIGIT_LIST[j]) {
                return line[i] - ASCII_OFFSET;
            }
        }
    }
    return res;
}

// return the first int from the left of the line 
int get_from_right(char* line, int size) {
    int i;
    int j;
    int res = -1;
    int check;
    for (i = size-1; i >= 0; i--) {
        printf("i: %d, line[i]: %c", i, (line[i] + line[i+1]));
        for (j = 0; j < 10; j++) {
            check = line[i] - ASCII_OFFSET;
            if (check == DIGIT_LIST[j]) {
                return line[i] - ASCII_OFFSET;
            }
        }
    }
    return res;
}

int main() {
    //stores res
    int total = 0;
    //input file fd
    FILE* fd = NULL;
    //line lenght
    size_t alloc_mem_size;
    //read return
    int nb_char;
    //pointer to the line
    char* line = NULL;
    //left value from line
    int left;
    //right value from line
    int right;
    //line value after concat left and right
    int line_total;

    init();

    fd = fopen(INPUT_PATH, "r");
    if (NULL == fd) {
        printf("Error while opening the file...");
        return EXIT_FAILURE;
    }

    while ((nb_char = getline(&line, &alloc_mem_size, fd)) != EOF) {
        left = get_from_left(line, nb_char);
        right = get_from_right(line, nb_char);
        line_total = left * 10 + right;
        printf("line is :%s", line);
        printf("left digit is: %d || right digit is: %d || line_total is: %d || current total is: %d \n", left, right, line_total, total);
        printf("---------------------------------------------------------------------------------------------\n");
        if (right == -1 || left == -1) {
            printf("No value found in right or left, exit failure");
            return EXIT_FAILURE;
        }
        total = total + line_total;
    }
    free(line);
    printf("the total is: %d", total);

    return EXIT_SUCCESS;
}