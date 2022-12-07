caloriesList = open("./01/input.txt").readlines()
results = []

i = 0
for item in caloriesList:
    if item == "\n":
        i+=1
        continue
    if i == len(results):
        results.insert(i, 0)
    results[i] += int(item)
    
sorted_list = sorted(results)
print(sorted_list[-1] + sorted_list[-2] + sorted_list[-3])