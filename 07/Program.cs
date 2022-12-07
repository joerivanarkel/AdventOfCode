// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");

var currentDir = string.Empty;
List<FileObject> files = new();

var input = File.ReadAllLines("./input.txt");
var i = 0;
for (i = 0; i < input.Count(); i++)
{
    var item = input[i];
    item.Replace("\n", "");
    switch (item.Substring(2, 2))
    {
        case "cd":
            SwitchDirectory(item);
            // Console.WriteLine(currentDir);
            continue;
        case "ls":
            ListDirectory(item);
            continue;
    }
}

Dictionary<string, int> directorySizes = new();
foreach (var item in files)
{
    CalculateDirectorySize(item);
}

// var result = 0;
// foreach (var item in directorySizes)
// {
//     if (item.Value <= 100000)
//     {
//         Console.WriteLine($"{item.Key} : {item.Value}");
//         result += item.Value;
//     }
//     Console.WriteLine("Directory to big");
// }
// Console.WriteLine(result);

var sortedDictionary = directorySizes.OrderBy(x => x.Value).ToDictionary(x => x.Key, x => x.Value);
var filtered = sortedDictionary.Where(x => x.Value >= 8381165).OrderBy(x => x.Value);

Console.WriteLine("");
//

void SwitchDirectory(string command)
{
    if (command == "$ cd ..")
    {
        currentDir = currentDir.Remove(currentDir.Length - 1, 1);
        while (currentDir.Last() != '/')
        {
            currentDir = currentDir.Remove(currentDir.Length - 1, 1);
        }
        return;
    }
    currentDir += command.Replace("$ cd ", "") + "/";
}

void ListDirectory(string command)
{
    command = input[i += 1];
    while (command.Substring(0, 1) != "$")
    {
        var splitString = command.Split(' ');
        files.Add(new FileObject(
            currentDir,
            splitString[0],
            splitString[1]
        ));
        if (i >= input.Count()) return;
        if (i + 1 >= input.Count()) return;
        if (input[i + 1].Substring(0, 1) == "$") return;

        i += 1;
        command = input[i];
    }
}

void CalculateDirectorySize(FileObject fileObject)
{
    var result = int.TryParse(fileObject.Value, out int parseResult);
    if (!result) return;

    var directory = fileObject.Location.Remove(fileObject.Location.Length - 1, 1);
    var parsedDirectories = directory.Split('/');
    
    var dirLocation = string.Empty;
    foreach (var dirName in parsedDirectories)
    {
        dirLocation += "/" + dirName;
        if (dirLocation == "") continue;
        if (!directorySizes.ContainsKey(dirLocation))
        {
            directorySizes.Add(dirLocation, parseResult);
        }
        else
        {
            directorySizes[dirLocation] += parseResult;
        }
    }
}

class FileObject
{
    public string Location { get; set; }
    public string Value { get; set; }
    public string Name { get; set; }

    public FileObject(string location, string value, string name)
    {
        Location = location;
        Value = value;
        Name = name;

        // Console.WriteLine($"Found {value} {name} at {location}");
    }
}