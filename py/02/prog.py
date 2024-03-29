import os

# 12 red cubes, 13 green cubes, and 14 blue cubes

def load_input(file_path):
    with open(file_path, 'r') as f:
        data = f.readlines()
    return data

def parse_input_line_to_dict(line):
    dico = {"id":None}
    _colors = ["blue", "red", "green"]
    dico["id"] = int(line.split(":")[0].split("Game ")[1].strip().rstrip())
    for color in _colors:
        dico[color] = 0
    for pull in line.split(":")[1].split(";"):
        for pulled_color in pull.split(","):
            for color in _colors:
                if color in pulled_color:
                    nb_color_in_pull = int(pulled_color.split(color)[0].strip().rstrip())
                    if dico[color] < nb_color_in_pull:
                        dico[color] = nb_color_in_pull
    return dico
RED = 12
GREEN = 13
BLUE = 14
input_file_path = os.path.join(os.getcwd(), "input.txt")
raw_data_list = load_input(input_file_path)
bag_dict_list = []

for line in raw_data_list:
    bag_dict_list.append(parse_input_line_to_dict(line))

# Question 1
"""
possible_bags = []
bag_ids_sum = 0
for bag in data:
    if bag["red"] <= RED and bag["green"] <= GREEN and bag["blue"] <= BLUE:
        possible_bags.append(bag["id"])
        bag_ids_sum = bag_ids_sum + bag["id"]
print(bag_ids_sum)
"""

#Question 2
result = 0
for bag in bag_dict_list:
    result = result + bag["red"] * bag["green"] * bag["blue"]
print(result)













