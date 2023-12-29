import os
import json
from copy import deepcopy

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def raw_data_input_to_data_input(raw_data_input):
    """
        Returns data_input from raw_data_input. data_input is a dict like:
        {
            seeds:[seed_range, location_id], #location_id set to None for now
            seed_to_soil:[
                            {
                                src_start:int,
                                dst_start:int,
                                range:int,
                            },
                            {...},
                         ]
            ...
        }
    """
    result = {}
    # Get input seeds
    result["seeds"] = []
    i = 0
    for tmp in [int(x) for x in raw_data_input[0].split(":")[1].split(" ") if x != ""]:
        if i%2 == 0:
            start = tmp
        else:
            result["seeds"].append([range(start, start + tmp), None])
        i += 1
    # Define our mapp names [0] and mapp names from the raw_data_input [1]
    _mapp_names = [("seed_soil","seed-to-soil map:"), ("soil_fertilizer", "soil-to-fertilizer map:"),\
                   ("fertilizer_water", "fertilizer-to-water map:"), ("water_light", "water-to-light map:"),\
                   ("light_temperature", "light-to-temperature map:"), ("temperature_humidity", "temperature-to-humidity map:"),\
                   ("humidity_location", "humidity-to-location map:")]
    offset = 2
    for mapp_name in _mapp_names:
        for index_line, line in enumerate(raw_data_input[offset:]):
            if line == "": # Break in case we finish this mapping to go the next
                offset +=  index_line + 1
                break
            elif mapp_name[1] in line: # Create key for the mapping if first line of mapping
                result[mapp_name[0]] = {}
                result[mapp_name[0]]["mapping_rules"] = []
            else: # Append required data to the result
                dico = {}
                dico["dst_start"], dico["src_start"], dico["range"] = (int(x) for x in line.split(" ") if x != "")
                result[mapp_name[0]]["mapping_rules"].append(dico)
    return result

def full_load_input(file_path):
    return raw_data_input_to_data_input(load_input(file_path))

def get_serializable_data_input(data_input):
    """
        Make the data_input serializable
        HARD CODED...
    """
    def _replace_range_obj(range_obj):
        return f'range(start:{range_obj.start}, stop:{range_obj.stop}, step:{range_obj.step})'
    copy_data_input = deepcopy(data_input)
    # Define places where we have unserializable objects
    for mapp_name in copy_data_input.keys():
        if mapp_name == "seeds":
            for seed in copy_data_input[mapp_name]:
                seed[0] = _replace_range_obj(seed[0])
                if not seed[1] is None:
                    for index_location_range in range(0, len(seed[1])):
                        seed[1][index_location_range] = _replace_range_obj(seed[1][index_location_range])
        else:
            for vect_dict in copy_data_input[mapp_name]["mapping_vectors"]:
                vect_dict["src"] = _replace_range_obj(vect_dict["src"])
                vect_dict["dst"] = _replace_range_obj(vect_dict["dst"])
    return copy_data_input

class RicoMappingVector:
    """
        Stores data about vector bounds from both side of the mapping
    """

    def __init__(self, mapping_rules):
        """
            vectors_dicts is a list of dict containing range objects for src and dst, they are of the same size
            The full processing of mapping rules is done on class instanciation
            vectors_dicts format: [{src:range, dst:range}, {...}] in a ascending order based on src_start
        """
        self.max_range_stop = 99999999999
        self.vectors_dicts = []
        for mapping_rule in mapping_rules:
            insertion_index = self.process_a_mapping_rule(mapping_rule)
        self.complete_vectors_dicts()

    def process_a_mapping_rule(self, mapping_rule):
        """
        Process a single mapping_rule, and updates vectors accordingly
        mapping_rule format: 
            {
                "dst_start": XXXX,
                "src_start": XXXX,
                "range": XXXX
            }
        Range object are inserted with a ascending order (based on src value)
        Returns the index of insertion
        """
        SS = mapping_rule["src_start"]
        SD = mapping_rule["dst_start"]
        R = mapping_rule["range"]
        range_dico = {"src":range(SS, SS + R), "dst":range(SD, SD + R)}
        index_vect = -1
        if len(self.vectors_dicts) == 0: # Init the vect
            self.vectors_dicts.append(range_dico)
            return index_vect
        for index_vect, vect_dict in enumerate(self.vectors_dicts):
            if SS > vect_dict["src"].start: # If higher means that we need to insert inplace of the previous value <=> the current index
                if index_vect == len(self.vectors_dicts) - 1: # We reached the end of the vectors_dicts list, so we append, not insert
                    self.vectors_dicts.append(range_dico)
                    return index_vect
            else: # Insert at the current index, shifting other vect_dicts to the right of the list
                self.vectors_dicts.insert(index_vect, range_dico)
                return index_vect
        self.vectors_dicts.insert(0, range_dico) # If we didn't returned during the for loop, it means SS < all current vect_dict[src], so insert it as first vect_dict
        return index_vect
    
    def complete_vectors_dicts(self):
        """
            Complete a vectors_dicts in order to start at 0 and end at max_range_stop
        """
        # Complete the start (if needed aka not affected by any mapping rule) and the end (always up to self.max_range_stop)
        if self.vectors_dicts[0]["src"].start != 0:
            range_dict = {'src':range(0, self.vectors_dicts[0]["src"].start), 'dst':range(0, self.vectors_dicts[0]["src"].start)}
            self.vectors_dicts.insert(0, range_dict)
        range_dict = {'src':range(self.vectors_dicts[-1]["src"].stop, self.max_range_stop), 'dst':range(self.vectors_dicts[-1]["src"].stop, self.max_range_stop)}
        self.vectors_dicts.append(range_dict)
        # Check if incompletness in the vectors_dicts, due to non affected mapp
        src_vects_list = [] # Will contains all source vectors (start, stop, index_vect_dict)
        for index_vect_dict, vect_dict in enumerate(self.vectors_dicts):
            # We store the index as well to be able to insert the completing range at the right place 
            src_vects_list.append((vect_dict["src"].start, vect_dict["src"].stop, index_vect_dict)) # Should be in ascending order thx to self.vectors_dicts being in ascending order based on src starts
        for index_src_vect, src_vect in enumerate(src_vects_list):
            if index_src_vect == 0: # Initialize the loop
                previous_src_vect = src_vect # Not checked but should always be 0...
                continue
            else:
                if previous_src_vect[1] != src_vect[0]: # If the previous stop is not the current start then we have incomplete interval in our vectors_dicts
                    range_dict = {'src':range(previous_src_vect[1], src_vect[0]), 'dst':range(previous_src_vect[1], src_vect[0])} # Define the range we want to complete
                    self.vectors_dicts.insert(src_vect[2], range_dict) # Insert it in place of the current vect_dict, which will be shifted to the right
            previous_src_vect = src_vect
    
def create_all_mappingvectors(data_input):
    """
        Update the data_input dict, adding the list of vector dicts to each mapp_name
    """
    for mapp_name in data_input.keys():
        if mapp_name == 'seeds': # Skip seeds key
            continue
        mapping_vector = RicoMappingVector(data_input[mapp_name]["mapping_rules"])
        data_input[mapp_name]["mapping_vectors"] = mapping_vector.vectors_dicts

def get_mapped_range(range_obj, vectors_dicts):
    """
        Returns a list of range objects once mapped from range_obj using the vectors_dicts
    """
    vector_dicts_index_list = [] # List containing 2 indexes of vect_dict, one for bot bound, one for top bound of the range_obj
    range_obj_start_inclusion_found = False
    range_obj_comparison_bound = range_obj.start # Define if we compare bot or top bound of the range object
    # Get the indexes of inclusion of bot and top bounds of the range_obj
    for index_vect_dict, vect_dict in enumerate(vectors_dicts):
        if range_obj_comparison_bound < vect_dict["src"].stop:
            vector_dicts_index_list.append(index_vect_dict)
            if not range_obj_start_inclusion_found: # Cover the situation where the 2 bounds of the range_obj are included in the same vect_dict
                range_obj_start_inclusion_found = True
                range_obj_comparison_bound = range_obj.stop
                if range_obj_comparison_bound < vect_dict["src"].stop:
                    vector_dicts_index_list.append(index_vect_dict)
                    break # Found the 2 bounds we need, we can leave the loop
            else:
                break # Found the 2 bounds we need, we can leave the loop
    
    vector_dicts_index_list = list(set(vector_dicts_index_list)) # Remove duplicates (in case bot and top bounds are the same) we don't want to process them twice in the next step
    vector_dicts_index_list.sort() # When we list(set([])) we have an undefined behaviour on the order of the base list, safer to manually sort it
    vector_dicts_index_list = [x for x in range(vector_dicts_index_list[0], vector_dicts_index_list[-1] + 1)]
    result = [] # List containing range objects, resulting from the mapping of each vect_dict within the interval defined by: vector_dicts_index_list
    #print(f'{vector_dicts_index_list=}')
    #print("-"*70)
    for index_in_iter_list, index_vect_dict in enumerate(vector_dicts_index_list, 0):
        vect_dict = vectors_dicts[index_vect_dict]
        mapped_bot_index = 0
        mapped_top_index = vect_dict["src"].stop - vect_dict["src"].start - 1
        if index_in_iter_list == 0: # Cover the case of the first vect_dict, we don't want to have the full range, only from the range_obj bot bound
            mapped_bot_index = range_obj.start - vect_dict["src"].start
        if index_in_iter_list == len(vector_dicts_index_list) - 1: # Cover the case of the last vect_dict, we don't want to have the full range, only up to the range_obj top bound
            mapped_top_index = range_obj.stop - vect_dict["src"].start - 1
        #print(f'{mapped_bot_index=} || {mapped_top_index=} \n{range_obj=} \n{vect_dict["src"]=} \n{vect_dict["dst"]=}')
        result.append(range(vect_dict["dst"][mapped_bot_index], vect_dict["dst"][mapped_top_index])) # Append the destination range from valid bot and top bounds
        #print("-"*70)
    #print("#"*80)
    return result

def process_all_seed_ranges(data_input):
    """
        Add a list of ranges of location_id for each seed range
    """
    nb_seeds = len(data_input["seeds"])
    for seed_index, seed in enumerate(data_input["seeds"], 1):
        seed_range = seed[0] # The seed input range
        object_ranges = [seed_range] # Defined as a list since once mapped it can become few ranges
        print(f'PROCESSING SEED: {seed_index}/{nb_seeds}')
        for mapp_name in data_input.keys():
            if mapp_name == "seeds": # Skip the seeds key
                continue
            print(f'PROCESSING MAPPING VECTOR: {mapp_name}')
            tmp_obj_ranges = []
            for range_obj in object_ranges:
                tmp_obj_ranges += get_mapped_range(range_obj, data_input[mapp_name]["mapping_vectors"])
            object_ranges = tmp_obj_ranges
            print('-'*50)
        print('#'*80)
        seed[1] = object_ranges

# Data processing
input_file_path = os.path.join(os.getcwd(), "input.txt")
data_input = full_load_input(input_file_path)
create_all_mappingvectors(data_input)
process_all_seed_ranges(data_input)

#'''
# Result processing, we only check start of each range, since it evolves linearly, the start is always the smallest possible within the considered range
result = None
for seed_obj in data_input["seeds"]:
    for seed_location_range in seed_obj[1]:
        if result is None:
            result = seed_location_range.start
        else:
            if result > seed_location_range.start:
                result = seed_location_range.start
print(result)
#'''

# Data visualization
data_input_file_path = os.path.join(os.getcwd(), "data_input.json")
with open(data_input_file_path, 'w') as fd:
    json.dump(get_serializable_data_input(data_input), fd)
