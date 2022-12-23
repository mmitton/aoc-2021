Function Print-List($List) {
    # Print out list in turn order
    Write-Host "In Turn Order"
    for ($idx = 0; $idx -lt $List.Count; $idx++) {
        $item = $List[$idx];
        Write-Host "$idx  Num:$($item.Num)  Prev:$($item.Prev)  Next:$($item.Next)";
    }
    
    # Print out list in list order
    Write-Host "In List Order"
    $idx = $zero_at;
    for ($c = 0; $c -lt $List.Count; $c++) {
        $item = $List[$idx];
        Write-Host "$idx  Num:$($item.Num)  Prev:$($item.Prev)  Next:$($item.Next)";
        $idx = $item.Next;
    }

    $nums = @(
        $idx = $zero_at;
        for ($c = 0; $c -lt $List.Count; $c++) {
            $List[$idx].Num
            $idx = $List[$idx].Next;
        }
    );
    Write-Host ($nums -join ", ")
}

Function Get-Number($List, $StartingAt, $After) {
    $idx = $StartingAt;
    for ($i = 0; $i -lt $After; $i++) {
        $idx = $List[$idx].Next;
    }

    return $List[$idx].Num;
}

$lines = (Get-Content "input.txt") -split "\n"

$list = @();
$zero_at = -1;
foreach ($line in $lines) {
    $item = [PSCustomObject] @{
        Num = [int]$line
        Prev = $list.Count - 1
        Next = $list.Count + 1
    };

    if ($item.Num -eq 0) {
        $zero_at = $list.Count;
    }

    $list += $item;
}

$list[0].Prev = $list.Count - 1;
$list[$list.Count - 1].Next = 0;

Print-List -List $list

Write-Host "Mix the numbers!"
# Run through the list in the order they were put in the list
for ($idx = 0; $idx -lt $List.Count; $idx++) {
    # Get the item from the list
    $item = $list[$idx];

    # Remove it from the "linked list" and link the previous item to the next item (and vice versa)
    $list[$item.Prev].Next = $item.Next;
    $list[$item.Next].Prev = $item.Prev;

    $insert_after = $item.Prev;
    $num = $item.Num;
    while ($num -ne 0) {
        if ($num -lt 0) {
            $insert_after = $list[$insert_after].Prev;
            $num += 1;
        } else {
            $insert_after = $list[$insert_after].Next;
            $num -= 1;
        }
    }

    $insert_before = $list[$insert_after].Next;
    $list[$idx].Next = $insert_before;
    $list[$idx].Prev = $insert_after;
    $list[$insert_after].Next = $idx;
    $list[$insert_before].Prev = $idx;
}

Print-List -List $list

$num1 = Get-Number -List $list -StartingAt $zero_at -After 1000
$num2 = Get-Number -List $list -StartingAt $zero_at -After 2000
$num3 = Get-Number -List $list -StartingAt $zero_at -After 3000

$ans = $num1 + $num2 + $num3;

Write-Host "$num1 + $num2 + $num3 = $ans";
