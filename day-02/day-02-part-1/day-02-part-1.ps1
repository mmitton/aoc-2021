$file = Get-Content ../input.txt
$lines = $file.Split("\n")
$moves = @()
foreach ($line in $lines) {
    $line = $line.Trim()
    if ($line -eq "") {
        continue;
    }
    $parts = $line.Split(" ");
    $parts[1] = [int]$parts[1];
    $move = [PSCustomObject]@{
        command = $parts[0]
        delta = $parts[1]
    };
    $moves += $move
}

$depth = 0;
$horizontal = 0;

foreach ($move in $moves) {
    if ($move.command -eq "forward") {
        $horizontal += $move.delta;
    } elseif ($move.command -eq "down") {
        $depth += $move.delta;
    } elseif ($move.command -eq "up") {
        $depth -= $move.delta;
    }
}

$answer = $depth * $horizontal;
Write-Host "depth:$depth horizontal:$horizontal answer:$answer"
