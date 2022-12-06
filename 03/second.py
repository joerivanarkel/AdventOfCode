
import string


input_list = open("./3/input.txt").readlines()

result = []

threeitems = []
i=0

for item in input_list:
    item= item.replace("\n", "")
    threeitems.append(item)
    i+=1
    
    if i == 3:
        result.append(threeitems)
        
        threeitems = []
        i=0

badges = []
for strings in result:
    common = list(set.intersection(*map(set,strings)))
    
    if len(common) < 1:
        exit()
        
    badges.append(common[0])
    
priority = {}

i = 1
for letter in string.ascii_lowercase:
    priority[letter] = i
    i+=1
    
for letter in string.ascii_uppercase:
    priority[letter] = i
    i+=1

result = 0
for letter in badges:
    result+=priority[letter]

print(result)