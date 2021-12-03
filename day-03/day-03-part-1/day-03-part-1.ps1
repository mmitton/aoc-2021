$file = Get-Content ../input.txt
$lines = $file -split "\n"

Function Get-Count {
    param(
        [String[]] $lines,
        [int] $bit,
        [char] $c
    )

    $count = 0;
    foreach ($line in $lines) {
        if ($line[$bit] -eq $c) {
            $count += 1;
        }
    }

    return $count;
}

$gamma = 0;
$epsilon = 0;
foreach ($bit in (0..($lines[0].length - 1))) {
    $gamma = $gamma -shl 1;
    $epsilon = $epsilon -shl 1;

    $zeros = Get-Count $lines $bit '0';
    $ones = Get-Count $lines $bit '1';

    if ($ones -gt $zeros) {
        $gamma = $gamma -bor 1;
    } else {
        $epsilon = $epsilon -bor 1;
    }
}

Write-Host "gamma:$gamma  epsilon:$epsilon  answer:$($gamma * $epsilon)"
