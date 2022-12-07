def isUniqueChars(st):
 
    # String length cannot be more than
    # 256.
    if len(st) > 256:
        return False
 
    # Initialize occurrences of all characters
    char_set = [False] * 128
 
    # For every character, check if it exists
    # in char_set
    for i in range(0, len(st)):
 
        # Find ASCII value and check if it
        # exists in set.
        val = ord(st[i])
        if char_set[val]:
            return False
 
        char_set[val] = True
 
    return True



input_list = open("./06/input.txt").readlines()[0]

start = 0
end = 14

while True:
    possible_marker = input_list[start:end]
    
    if isUniqueChars(input_list[start:end]):
        print(end)
        break
    
    start +=1
    end +=1
    


