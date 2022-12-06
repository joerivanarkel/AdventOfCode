import string


input_list = open("./3/input.txt").readlines()
compartement_results = []

for item in input_list:
    item = item.replace("\n", "")
    intermediate = []
    index = int(len(item)/2)
    
    intermediate.append(item[:index])
    intermediate.append(item[index:])
    compartement_results.append(intermediate)

common_char_results = []
for item in compartement_results:
    common_char = "".join(set(item[0]).intersection(item[1]))
    common_char_results.append(common_char)
    
priority = {}

i = 1
for letter in string.ascii_lowercase:
    priority[letter] = i
    i+=1
    
for letter in string.ascii_uppercase:
    priority[letter] = i
    i+=1

result = 0
for letter in common_char_results:
    result+=priority[letter]

print(result)