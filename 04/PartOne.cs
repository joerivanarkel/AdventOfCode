// See https://aka.ms/new-console-template for more information
namespace four;

class FourPartOne
{
public void Archived()
{


Console.WriteLine("Hello, World!");

var input = File.ReadAllLines("./input.txt");

List<List<(int, int)>> splitInput = new();
foreach (var item in input)
{
    var split = item.Split(',');

    List<(int left, int right)> range = new();
    foreach (var i in split)
    {
        var k = i.Split('-');
        range.Add((int.Parse(k[0]), int.Parse(k[1])));
    }

    splitInput.Add(range);
}

var result = 0;
foreach (var item in splitInput)
{
    var iresult = 0;
    iresult += RangeOverlap(item[0], item[1]);
    iresult += RangeOverlap(item[1], item[0]);
    if (iresult == 2)
    {
        Console.WriteLine(item[0].ToString() + item[1].ToString());
        iresult = 1;
    }
    result += iresult;
}

int RangeOverlap((int left, int right) tupleOne, (int left, int right) tupleTwo)
{
    if (
        tupleOne.left <= tupleTwo.left &&
        tupleOne.right >= tupleTwo.left &&
        tupleOne.left <= tupleTwo.right &&
        tupleOne.right >= tupleTwo.right
        )
    {
        return 1;
    }
    return 0;
}

Console.WriteLine(result);

}

}