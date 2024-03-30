import os

def load_input(file_path):
    with open(file_path, 'r') as f:
        data = f.readlines()
    return data

def process_line(line, is_reversed):  
    line_size = len(line)
    for i, try_int in enumerate(line, 0):
        found_as_str = False
        try:
            x = int(try_int)
            break
        except:
            for _digit_mapper_key in digit_mapper.keys():
                digit_mapper_len = len(_digit_mapper_key)
                # check if we went in this loop
                if is_reversed: # reverse the key to compare with the reversed string
                    digit_mapper_key = _digit_mapper_key[::-1]
                else:
                    digit_mapper_key = _digit_mapper_key
                if not digit_mapper_len <= line_size - (i + 1): # check that we can have enough lenght to compare 
                    continue
                if digit_mapper_key == "".join(line[i:i+digit_mapper_len]): # check if string matches
                    x = digit_mapper[_digit_mapper_key]
                    found_as_str = True
                    break
        if found_as_str:
            break
    return x

digit_mapper = {"zero":0,
                "one":1,
                "two":2,
                "three":3,
                "four":4,
                "five":5,
                "six":6,
                "seven":7,
                "eight":8,
                "nine":9}

input_file_path = os.path.join(os.getcwd(), "input.txt")
data_list = load_input(input_file_path)
output_list = []

for line in data_list:
    x = process_line(line, False)
    y = process_line(line[::-1], True)
    z = x * 10 + y
    output_list.append(z)
    print(x, y)

result = sum(output_list)
print(result)
