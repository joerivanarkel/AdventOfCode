// See https://aka.ms/new-console-template for more information
// Not a solution
Console.WriteLine("Hello, World!");


List<List<char>> crates = new();

List<char> one = new()      { 'G', 'T', 'R', 'W' };
List<char> two = new()      { 'G', 'C', 'H', 'P', 'M', 'S', 'V', 'W' };
List<char> three = new()    { 'C', 'L', 'T', 'S', 'G', 'M' };
List<char> four = new()     { 'J', 'H', 'D', 'M', 'W', 'R', 'F' };
List<char> five = new()     { 'P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J' };
List<char> six = new()      { 'P', 'J', 'D', 'N', 'F', 'M', 'S' };
List<char> seven = new()    { 'Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J' };
List<char> eight = new()    { 'R', 'T', 'B' };
List<char> nine = new()     { 'H', 'N', 'W', 'L', 'C' };


var input = File.ReadAllLines("./input.txt");

List<List<int>> moveList = new();
foreach (var line in input)
{
    var split = line.Split(' ');

    moveList.Add(
        new List<int>() {int.Parse(split[0]), int.Parse(split[1]), int.Parse(split[2])}
    );
}

Dictionary<int, List<char>> lists = new() {
    { 1, one },
    { 2, two },
    { 3, three},
    { 4, four },
    { 5, five },
    { 6, six },
    { 7, seven },
    { 8, eight},
    { 9, nine}
};

foreach (var item in moveList)
{
    MoveCrate(item[0], item[1], item[2], lists);
}

var resultString = string.Empty;
foreach (var item in lists)
{
    resultString += item.Value.Last();
}
Console.WriteLine(resultString);


void MoveCrate(int amount, int orgin, int destination, Dictionary<int, List<char>> lists)
{
    Console.WriteLine($"Moving {amount} crates from {orgin} to {destination}");
    var orginList = lists[orgin];
    var destinationList = lists[destination];

    if (orginList.Count == 0 || destinationList.Count == 0)
    {
        Console.WriteLine("Empty");
    }

    for (int i = 1; i <= amount; i++)
    {
        destinationList.Add(orginList.Last());
        orginList.Remove(orginList.Last());
    }

    PrintLists(lists);
}

void PrintLists(Dictionary<int, List<char>> lists)
{
    foreach (var item in lists)
    {
        var text = string.Join(",", item.Value.ToArray());
        Console.WriteLine($"{item.Key} {text}");
    }
    Console.WriteLine('\n');
}


// JCMHLVGMG