import os
import json
from copy import copy
import sys

FIVE_OF_KIND = (6, "Five Of Kind")
FOUR_OF_KIND = (5, "Four Of Kind")
FULL_HOUSE = (4, "Full House")
THREE_OF_KIND = (3, "Three Of Kind")
TWO_PAIR = (2, "Two Pair")
ONE_PAIR = (1, "One Pair")
HIGH_CARD = (0, "High Card")
POWER_CARD_MAPPER = {"2":1,
                     "3":2,
                     "4":3,
                     "5":4,
                     "6":5,
                     "7":6,
                     "8":7,
                     "9":8,
                     "T":9,
                     "J":10,
                     "Q":11,
                     "K":12,
                     "A":13}

def load_input(file_path):
    data = []
    with open(file_path, 'r') as f:
        for line in f.readlines():
            data.append(line.strip().rstrip())
    return data

def parse_input(raw_data_input):
    """
        returns a list of dict like:
        [{"hand":str, "bid":int}, ...]
    """
    result = []
    for line in raw_data_input:
        result.append({"hand":line.split(" ")[0], "bid":int(line.split(" ")[1])})
    return result

def make_serializable(rico_hands):
    result = []
    for rico_hand in rico_hands:
        result.append(str(rico_hand))
    return result

class RicoHand:
    
    def __init__(self, hand, bid):
        """
        """
        self.hand = hand
        self.bid = bid
        self.rank = None # Set to None for now, will be updated later
        self.parsed_hand = self.get_parsed_hand() # dict like {card:nb}
        self.type = self.get_type() # int defined in the constants in begining of script
        self.comparable_hand = self.get_comparable_hand()

    def __repr__(self):
        return f'RicoHand(hand:{self.hand} || bid:{self.bid} || type:{self.type[1]} || rank:{self.rank} || parsed_hand: {self.parsed_hand})'
    
    def __str__(self):
        return f'RicoHand(hand:{self.hand} || bid:{self.bid} || type:{self.type[1]} || rank:{self.rank} || comp_hand:{self.comparable_hand}'

    def get_parsed_hand(self):
        """
            Parse the hand in a dict like:
            {card(str):nb(int)}
        """
        dico = {}
        for card in self.hand:
            if card not in dico.keys():
                dico[card] = 1
            else:
                dico[card] += 1
        return dico
    
    def get_comparable_hand(self):
        """
            Returns int representing the overall card values
        """
        result = 0
        for card in self.hand:
            result = 100*result + POWER_CARD_MAPPER[card]
        return result 
        

    def get_type(self):
        """
            Define the hand type and its associated card
        """
        three_found = False
        two_found = [False, False]
        result = None
        for card in self.parsed_hand.keys():
            if self.parsed_hand[card] == 5: 
                result = FIVE_OF_KIND
            elif self.parsed_hand[card] == 4: 
                result = FOUR_OF_KIND
            elif self.parsed_hand[card] == 3:
                if not three_found:
                    three_found = True
            elif self.parsed_hand[card] == 2:
                if not two_found[0]:
                    two_found[0] = True
                elif not two_found[1]:
                    two_found[1] = True
        if result is None:
            if three_found and two_found[0]:
                result = FULL_HOUSE
            elif three_found and not two_found[0]:
                result = THREE_OF_KIND
            elif two_found[0] and two_found[1]:
                result = TWO_PAIR
            elif two_found[0] and not two_found[1]:
                result = ONE_PAIR
            else:
                result = HIGH_CARD
        return result

def process_data_input(data_input):
    """
        Returns a list of RicoHand objects based on the data_input
    """
    result = []
    for dico in data_input:
        result.append(RicoHand(dico["hand"], dico["bid"]))
    return result

def update_ranks(rico_hands):
    """
        Update the rank attrib for each RicoHand object in the rico_hands list
    """
    buckets = {FIVE_OF_KIND:[], 
               FOUR_OF_KIND:[], 
               FULL_HOUSE:[], 
               THREE_OF_KIND:[], 
               TWO_PAIR:[], 
               ONE_PAIR:[],  
               HIGH_CARD:[]}
    # First sort based on the type
    for rico_hand in rico_hands:
        buckets[rico_hand.type].append(rico_hand)
    # Then go through each bucket to compare their card values, starting from the lowest not to bother about rank values
    type_key_to_iterate_over = [HIGH_CARD, ONE_PAIR, TWO_PAIR, THREE_OF_KIND, FULL_HOUSE, FOUR_OF_KIND, FIVE_OF_KIND]
    rank_value = 1
    for card_type in type_key_to_iterate_over:
        sorted_list = sorted(buckets[card_type], key=lambda x: x.comparable_hand)
        for rico_hand in sorted_list:
            rico_hand.rank = rank_value
            rank_value += 1
        print(f"{card_type} Done")


input_file_path = os.path.join(os.getcwd(), "input.txt")
data_input = parse_input(load_input(input_file_path))
processed_hand_list = process_data_input(data_input)
update_ranks(processed_hand_list)

#"""
result = 0
for card in processed_hand_list:
    result += card.bid * card.rank
print(result)
#"""

output_file_path = os.path.join(os.getcwd(), "data.json")
with open(output_file_path, 'w') as fd:
    json.dump(make_serializable(processed_hand_list), fd)