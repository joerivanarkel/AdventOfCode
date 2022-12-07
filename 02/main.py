startegyGuide = open("./02/input.txt").readlines()

score = 0

for round in startegyGuide:
    round = round.replace("\n", "")
    match round:
        case "A X": # Rock Lose
            score +=3 # Scissors
            score +=0
        case "A Y": # Rock Draw
            score +=1
            score +=3
        case "A Z": # Rock Win
            score +=2
            score +=6
        case "B X":
            score +=1
            score +=0
        case "B Y":
            score +=2
            score +=3
        case "B Z":
            score +=3
            score +=6
        case "C X":
            score +=2
            score +=0
        case "C Y":
            score +=3
            score +=3
        case "C Z":
            score +=1
            score +=6
            
print(score)