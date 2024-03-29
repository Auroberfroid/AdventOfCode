import os
import json
import pandas as pd
from copy import copy
from math import ceil

class Pointer:
    def __init__(self, val):
        self._val = val
    def set_val(self, val):
        self._val = val
    def get_val(self):
        return self._val

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def parse_input(raw_data_input):
    """
        Returns a dict containg 2 df, one storing the maze char, the other one only None, it will store shortest distances from the start tile.
    """
    return {"maze":pd.DataFrame(data=[list(x) for x in raw_data_input], columns=[x for x in range(0,len(raw_data_input[0]))], dtype=str),
            "dist":pd.DataFrame(data=[list(-1 for y in range(0,len(raw_data_input))) for x in raw_data_input], columns=[x for x in range(0,len(raw_data_input[0]))], dtype=int),}

def find_maze_start(df):
    """
        Returns coordinates of the S tile from the maze
    """
    for col in range(0, len(df.columns)):
        for row in range(0, len(df)):
            if df[col][row] == 'S':
                return [col, row]
            
def init_dataframes(dfs):
    """
        Initializes maze and dist dataframes
        Set start tile at a dist of 0, and connected surrounding tile at a dist of 1
        Replace the S with the proper pipe char
        Returns the start_coords and the 2 other pipes that the start is connected to
    """
    start_coords = find_maze_start(dfs["maze"])
    dfs["dist"][start_coords[0]][start_coords[1]] = 0
    arounds = {"north":[start_coords[0], start_coords[1]-1], 
               "south":[start_coords[0], start_coords[1]+1], 
               "east":[start_coords[0]+1, start_coords[1]], 
               "west":[start_coords[0]-1, start_coords[1]]}
    start_connected_to = []
    if dfs["maze"][arounds["north"][0]][arounds["north"][1]] in ["|", "7", "F"]:
        start_connected_to.append("north")
    if dfs["maze"][arounds["south"][0]][arounds["south"][1]] in ["|", "L", "J"]:
        start_connected_to.append("south")
    if dfs["maze"][arounds["east"][0]][arounds["east"][1]] in ["-", "J", "7"]:
        start_connected_to.append("east")
    if dfs["maze"][arounds["west"][0]][arounds["west"][1]] in ["-", "L", "F"]:
        start_connected_to.append("west")
    if start_connected_to == ["north", "south"]:
        start_char = "|"
    elif start_connected_to == ["east", "west"]:
        start_char = "-"
    elif start_connected_to == ["north", "east"]:
        start_char = "L"
    elif start_connected_to == ["north", "west"]:
        start_char = "J"
    elif start_connected_to == ["south", "west"]:
        start_char = "7"
    elif start_connected_to == ["south", "east"]:
        start_char = "F"
    else:
        raise Exception("Unknown surroundings of start_char")
    dfs["maze"][start_coords[0]][start_coords[1]] = start_char
    dfs["dist"][arounds[start_connected_to[0]][0]][arounds[start_connected_to[0]][1]] = 1
    dfs["dist"][arounds[start_connected_to[1]][0]][arounds[start_connected_to[1]][1]] = 1
    return start_coords, arounds[start_connected_to[0]], arounds[start_connected_to[1]]

def get_direct_contact_tile_coords_list(coords):
    return [[coords[0]+1, coords[1]], [coords[0]-1, coords[1]], [coords[0], coords[1]+1], [coords[0], coords[1]-1]]

def get_around_tile_coords_list(coords):
    return [[coords[0]+1, coords[1]], [coords[0]-1, coords[1]], [coords[0], coords[1]+1], [coords[0], coords[1]-1], [coords[0]-1, coords[1]-1], [coords[0]+1, coords[1]+1], [coords[0]-1, coords[1]+1], [coords[0]+1, coords[1]-1]]

def get_next_coordinates(maze_char, previous_coords, current_coords):
    """
        Returns current_cords & next_coords
    """
    last_move_vect = [current_coords[0]-previous_coords[0], current_coords[1]-previous_coords[1]]
    if maze_char == "|":
        if last_move_vect == [0, 1]:
            vect = [0, 1]
        elif last_move_vect == [0, -1]:
            vect = [0, -1]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "-":
        if last_move_vect == [1, 0]:
            vect = [1, 0]
        elif last_move_vect == [-1, 0]:
            vect = [-1, 0]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "L":
        if last_move_vect == [0, 1]:
            vect = [1, 0]
        elif last_move_vect == [-1, 0]:
            vect = [0, -1]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "J":
        if last_move_vect == [0, 1]:
            vect = [-1, 0]
        elif last_move_vect == [1, 0]:
            vect = [0, -1]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "7":
        if last_move_vect == [1, 0]:
            vect = [0, 1]
        elif last_move_vect == [0, -1]:
            vect = [-1, 0]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == "F":
        if last_move_vect == [0, -1]:
            vect = [1, 0]
        elif last_move_vect == [-1, 0]:
            vect = [0, 1]
        else:
            raise Exception("Error while getting the next coordinates")
    elif maze_char == ".":
        raise Exception("Error while getting the next coordinates. Ground tile found")
    elif maze_char == "S":
        raise Exception("Error while getting the next coordinates. Start tile found")
    else:
        raise Exception("Error while getting the next coordinates. Unknown tile found")
    next_coords = [current_coords[0]+vect[0], current_coords[1]+vect[1]]
    return current_coords, next_coords

def process_dataframes(dfs):
    """
        Navigate through the maze on both ends of the start tile
        Stops when a navigation agent reach a tile with a distance value already set
    """
    navig_agent_1 = {"previous":None, "current":None}
    navig_agent_2 = {"previous":None, "current":None}
    start_coords, navig_agent_1["current"], navig_agent_2["current"] = init_dataframes(dfs)
    navig_agent_1["previous"] = copy(start_coords)
    navig_agent_2["previous"] = copy(start_coords)
    loop_closed = False
    distance = 1
    while not loop_closed:
        distance += 1
        # Agent 1 Navigation
        navig_agent_1["previous"], navig_agent_1["current"] = get_next_coordinates(maze_char=dfs["maze"][navig_agent_1["current"][0]][navig_agent_1["current"][1]],
                                                                                   previous_coords=navig_agent_1["previous"],
                                                                                   current_coords=navig_agent_1["current"])
        # Agent 1 Distance Value Set
        dfs["dist"][navig_agent_1["current"][0]][navig_agent_1["current"][1]] = distance

        # Agent 2 Navigation
        navig_agent_2["previous"], navig_agent_2["current"] = get_next_coordinates(maze_char=dfs["maze"][navig_agent_2["current"][0]][navig_agent_2["current"][1]],
                                                                                   previous_coords=navig_agent_2["previous"],
                                                                                   current_coords=navig_agent_2["current"])
        # Agent 2 Distance Value Set
        dfs["dist"][navig_agent_2["current"][0]][navig_agent_2["current"][1]] = distance

        #Check if loop closed
        if navig_agent_1["current"] == navig_agent_2["current"]:
            loop_closed = True
    return distance

def get_rough_loop_borders(dfs):
    """
        Returns a rough loop border as a list of interval from left to right
    """
    result = []
    for row in range(0, len(dfs["maze"])):
        west_border_found = False
        east_border_found = False
        to_append_borders = []
        for col_check in range(0, len(dfs["maze"].columns)):
            if dfs["dist"][col_check][row] != -1:
                to_append_borders.append([col_check, row])
                west_border_found = True
                break
        if west_border_found:
            for col_check in range(len(dfs["maze"].columns)-1, -1, -1):
                if dfs["dist"][col_check][row] != -1:
                    to_append_borders.append([col_check, row])
                    east_border_found = True
                    break
        if not west_border_found:
            result.append(None) # To be checked in later use
            continue
        if not east_border_found:
            to_append_borders.append([len(dfs["maze"]), row])
        result.append(to_append_borders)
    return result

def get_ground_tiles_list(df):
    """
        Returns a list of coords of the ground tiles
    """
    result = []
    for col in range(0, len(df.columns)):
        for row in range(0, len(df)):
            if df[col][row] == ".":
                result.append([col, row])
    return result

def get_possible_nest_tile_list(dfs):
    """
        Returns the intersection between the loop borders and all the ground_tiles coordinates
    """
    result = []
    ground_tile_list = get_ground_tiles_list(dfs["maze"])
    for ground_tile in ground_tile_list:
        if loop_borders[ground_tile[1]] is None:
            continue
        # if ground_tile[1] == 4:
            # print(f'{loop_borders[ground_tile[1]][0][0]=}')
            # print(f'{loop_borders[ground_tile[1]][1][0]=}')
            # print(f'{ground_tile[0]=}')
            # print(f'{loop_borders[ground_tile[1]][0][0] <= ground_tile[0] <= loop_borders[ground_tile[1]][1][0]=}')
        if loop_borders[ground_tile[1]][0][0] <= ground_tile[0] <= loop_borders[ground_tile[1]][1][0]:
            result.append(ground_tile)
    return result

def is_contained_by_loop(tile_coords):
    """
        Returns True if tile_coords within the loop_borders, False otherwise
    """
    if loop_borders[tile_coords[1]][0][0] <= tile_coords[0] <= loop_borders[tile_coords[1]][1][0]:
        return True
    else:
        return False

def get_buckets_possible_nested_tile_list(dfs):
    """
        Returns a list of list of possible nested tiles
        Buckets are lists of jointed possible nested tiles
    """
    def _recursive_spread_tile_search(coords, buck):
        """
            Returns a bucket containing all the possible nested tiles that are adjacent to each other
        """
        if buck is None:
            father_of_recursion = True
            buck = []
        else:
            father_of_recursion = False
        buck.append(coords)
        processed_possible_nest_tile_list.append(coords)
        for direct_contact_tiles_coords in get_direct_contact_tile_coords_list(coords):
            if direct_contact_tiles_coords in buck:
                continue
            if direct_contact_tiles_coords in possible_nest_tile_list:
                if direct_contact_tiles_coords in processed_possible_nest_tile_list:
                    continue
                _recursive_spread_tile_search(direct_contact_tiles_coords, buck)
        if father_of_recursion:
            return buck

    possible_nest_tile_list = get_possible_nest_tile_list(dfs)
    processed_possible_nest_tile_list = []
    buckets = []
    for possible_nest_tile in possible_nest_tile_list:
        if possible_nest_tile in processed_possible_nest_tile_list: # Don't process twice the same tile
            continue
        buckets.append(_recursive_spread_tile_search(possible_nest_tile, None))
    return buckets

def get_nested(dfs):
    """
        Returns a list of buckets that are verified nests
    """
    def _get_surrounding_pipes_of_non_direct_path_to_border_tiles(coords):
        """
            Returns a list of surrounding_pipes, if None in the result, then there is a ground path leading to the border of the maze
        """
        def __recursive_spread_tile_search(coords, father_of_recursion):
            """
                Returns a bucket containing all the possible nested tiles that are adjacent to each other
            """
            processed_tiles.append(coords)
            for direct_contact_tiles_coords in get_direct_contact_tile_coords_list(coords):
                if direct_contact_tiles_coords in processed_tiles:
                    continue
                # Continue if ground around
                if dfs["maze"][direct_contact_tiles_coords[0]][direct_contact_tiles_coords[1]] == ".":
                    # print(f'GROUND IN TILE: {direct_contact_tiles_coords}')
                    __recursive_spread_tile_search(direct_contact_tiles_coords, False)

                # Append None if border reached
                if direct_contact_tiles_coords[0] == len(dfs["dist"].columns) - 1 or \
                   direct_contact_tiles_coords[0] == 0 or \
                   direct_contact_tiles_coords[1] == len(dfs["dist"]) - 1 or \
                   direct_contact_tiles_coords[1] == 0:
                    # print(f'BORDER REACHED IN TILE: {direct_contact_tiles_coords}')
                    surrounding_pipes.append(None)
                
                # Stop if not ground around
                if dfs["maze"][direct_contact_tiles_coords[0]][direct_contact_tiles_coords[1]] != ".":
                    # print(f'PIPE IN TILE: {direct_contact_tiles_coords}')
                    if not direct_contact_tiles_coords in surrounding_pipes:
                        surrounding_pipes.append(direct_contact_tiles_coords)
            # Complete surrounding pipes with 'diagonal tiles'
            for complete_surrounding_pipes_coord in get_around_tile_coords_list(coords):
                if not complete_surrounding_pipes_coord in surrounding_pipes:
                    surrounding_pipes.append(complete_surrounding_pipes_coord)
                
            if father_of_recursion:
                return surrounding_pipes
        processed_tiles = []
        surrounding_pipes = []
        result = __recursive_spread_tile_search(coords, True)
        return result
    
    def _get_in_between_nested():
        """
            Returns True if the bucket is nested, False otherwise
        """
        # For [1,0] the order of pipe values is: TOP,BOT
        # For [0,1] the order of pipe values is: LEFT,RIGHT
        allowed_corner_pipes = {[1,0]:[["L", "F"], ["L", "7"], ["J", "F"], ["J", "F"]],
                                [0,1]:[["J", "L"], ["7", "F"], ["7", "L"], ["J", "F"]]}
        pipe_vectors = {"|":[0,1],
                        "-":[1,0],
                        "L":[1,1],
                        "J":[1,1],
                        "7":[1,1],
                        "F":[1,1]}
        pointer = Pointer(False)
        
        def __get_possible_arounds_next_between_pipes(previous_coords, current_coords):
            """
                Returns a list of 3 possible in between pipes position to be checked
            """
            if current_coords[0] % 1 == 0.5:
                positive_arounds = [[current_coords[0]+1, current_coords[1]], [ceil(current_coords[0]), current_coords[1]+0.5], [ceil(current_coords[0]), current_coords[1]-0.5]]
                negative_arounds = [[current_coords[0]-1, current_coords[1]], [int(current_coords[0]), current_coords[1]+0.5], [int(current_coords[0]), current_coords[1]-0.5]]
            elif current_coords[1] % 1 == 0.5:
                positive_arounds = [[current_coords[0], current_coords[1]+1], [current_coords[0]+0.5, ceil(current_coords[1])], [current_coords[0]-0.5, ceil(current_coords[1])]]
                negative_arounds = [[current_coords[0], current_coords[1]-1], [current_coords[0]+0.5, int(current_coords[1])], [current_coords[0]-0.5, int(current_coords[1])]]
            else:
                raise Exception(f"current_coords not in between pipes: {current_coords}")
            if previous_coords in positive_arounds:
                return negative_arounds
            elif previous_coords in negative_arounds:
                return positive_arounds
            else:
                raise Exception(f"previous_coords: {previous_coords} not in negative: {negative_arounds} nor in positive: {positive_arounds}")
            
        def __init_recursive_move_between_pipes():
            """
                Initialize the recursion of __recursive_get_next_move_between_pipes
                Returns a list of dict defining possible in between pipe coordinates ('previous' and 'current') from surrounding_pipes
            """
            pass

        def __recursive_get_next_move_between_pipes(previous_coords, current_coords):
            """
                Returns a list of possible next positions
            """
            possible_arounds = __get_possible_arounds_next_between_pipes(previous_coords, current_coords)
            for possible_between_tile in possible_arounds:
                if not is_contained_by_loop(possible_between_tile): # Loop borders reached
                    pointer.set_val(True)
                    return None
                if __check_if_move_allowed(possible_between_tile): # Recurse on possible next move
                    __recursive_get_next_move_between_pipes(current_coords, possible_between_tile)
            return None

        def __check_if_move_allowed(next_coords):
            """
                Returns True if the move is allowed at the next_coords, False otherwise
            """
            if next_coords[0] % 1 == 0.5:
                D = [0, 1]
                P_CHAR_1 = dfs["maze"][int(next_coords[0])][next_coords[1]] # TOP
                P_CHAR_2 = dfs["maze"][ceil(next_coords[0])][next_coords[1]] # BOT
                if D[1] + pipe_vectors[P_CHAR_1][1] + pipe_vectors[P_CHAR_2][1] != 3: # Vector check
                    return False
            elif next_coords[1] % 1 == 0.5:
                D = [1, 0]
                P_CHAR_1 = dfs["maze"][next_coords[0]][int(next_coords[1])] # LEFT
                P_CHAR_2 = dfs["maze"][next_coords[0]][ceil(next_coords[1])] # RIGHT
                if D[0] + pipe_vectors[P_CHAR_1][0] + pipe_vectors[P_CHAR_2][0] != 3: # Vector check
                    return False
            else:
                raise Exception(f"next_coords not in between pipes: {next_coords}")
            if not [P_CHAR_1, P_CHAR_2] in allowed_corner_pipes[D]: # Corner Check
               return False
            return False

        init_tile_list = __init_recursive_move_between_pipes()
        for init_tile in init_tile_list:
            __recursive_get_next_move_between_pipes(init_tile['previous'], init_tile['current'])

    
    buckets = get_buckets_possible_nested_tile_list(dfs)
    result = [] # list of index_buckets that are truly nested
    for index_bucket, buck in enumerate(buckets):
        # print(f"{index_bucket=}")
        to_check_nest_tile_coords = buck[0] # Only check the first tile of each bucket, its status implies the status of other tile in the same bucket
        # print(f"{to_check_nest_tile_coords=}")
        surrounding_pipes = _get_surrounding_pipes_of_non_direct_path_to_border_tiles(to_check_nest_tile_coords)
        # print(f"{surrounding_pipes=}")
        # print("#"*80)
        if None in surrounding_pipes: # Path of ground tile to border, so not nested
            continue
        
    return result
        


# Input Parsing
input_file_path = os.path.join(os.getcwd(), "input.txt")
data_frames = parse_input(load_input(input_file_path))

# Data Processing
max_distance = process_dataframes(data_frames)
loop_borders = get_rough_loop_borders(data_frames)
nested_buckets = get_nested(data_frames)
print(len(nested_buckets))

# Maze Output
test_maze = data_frames["maze"].to_html()
test_maze_file_path = os.path.join(os.getcwd(), "maze_df.html")
with open(test_maze_file_path, 'w') as f:
    f.write(test_maze)

# Dist Output
test_dist = data_frames["dist"].to_html()
test_dist_file_path = os.path.join(os.getcwd(), "dist_df.html")
with open(test_dist_file_path, 'w') as f:
    f.write(test_dist)
""" 
# Loop Borders Output
loop_borders = get_rough_loop_borders(data_frames)
test_loop_borders = {"data_wrapper":loop_borders}
test_loop_borders_file_path = os.path.join(os.getcwd(), "loop_borders.json")
with open(test_loop_borders_file_path, 'w') as f:
    json.dump(test_loop_borders, f)
 """
""" 
# Possible Nests Output
possible_nest_tile_list = get_possible_nest_tile_list(data_frames)
test_possible_nests = {"data_wrapper":possible_nest_tile_list}
test_possible_nests_file_path = os.path.join(os.getcwd(), "possible_nests.json")
with open(test_possible_nests_file_path, 'w') as f:
    json.dump(test_possible_nests, f)
 """
""" 
# Buckets Output
buckets = get_buckets_possible_nested_tile_list(data_frames)
test_buckets = {"data_wrapper":buckets}
test_buckets_file_path = os.path.join(os.getcwd(), "buckets.json")
with open(test_buckets_file_path, 'w') as f:
    json.dump(test_buckets, f)
 """

# Buckets Output
test_nested_buckets = {"data_wrapper":nested_buckets}
test_nested_buckets_file_path = os.path.join(os.getcwd(), "nested_buckets.json")
with open(test_nested_buckets_file_path, 'w') as f:
    json.dump(test_nested_buckets, f)
