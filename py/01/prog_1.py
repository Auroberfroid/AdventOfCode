import os

def load_input(file_path):
    with open(file_path, 'r') as f:
        data = f.readlines()
    return data

def process_line(line):  
    line_size = len(line)
    for i, try_int in enumerate(line, 0):
        found_as_str = False
        try:
            x = int(try_int)
            break
        except:
            continue
    return x

input_file_path = os.path.join(os.getcwd(), "input.txt")
data_list = load_input(input_file_path)
output_list = []

for line in data_list:
    x = process_line(line)
    y = process_line(line[::-1])
    z = x * 10 + y
    output_list.append(z)
    print(x, y)

result = sum(output_list)
print(result)
