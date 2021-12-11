$file = Get-Content ../input.txt
$lines = $file.Split("\n")
$moves = @()
foreach ($line in $lines) {
    $line = $line.Trim()
    if ($line -eq "") {
        continue;
    }
    $parts = $line.Split(" ");
    $move = [PSCustomObject]@{
        command = $parts[0]
        delta = [int]$parts[1]
    };
    $moves += $move
}

$depth = [int]0;
$horizontal = [int]0;
$aim = [int]0;

foreach ($move in $moves) {
    if ($move.command -eq "forward") {
        $horizontal += $move.delta;
        $depth += $move.delta * $aim;
    } elseif ($move.command -eq "down") {
        $aim += $move.delta;
    } elseif ($move.command -eq "up") {
        $aim -= $move.delta;
    }
}

$answer = $depth * $horizontal;
Write-Host "depth:$depth horizontal:$horizontal aim:$aim answer:$answer"
