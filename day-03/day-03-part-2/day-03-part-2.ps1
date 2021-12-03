$file = Get-Content ../input.txt
$lines = $file -split "\n"
$o2_lines = $lines;
$co2_lines = $lines;

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

    if ($o2_lines.length -gt 1) {
        $zeros = Get-Count $o2_lines $bit '0';
        $ones = Get-Count $o2_lines $bit '1';

        if ( $ones -ge $zeros ) {
            $mc = '1';
        } else {
            $mc = '0';
        }

        $o2_lines = @($o2_lines | Where { $_[$bit] -eq $mc })
    }

    if ($co2_lines.length -gt 1) {
        $zeros = Get-Count $co2_lines $bit '0';
        $ones = Get-Count $co2_lines $bit '1';

        if ( $ones -lt $zeros ) {
            $lc = '1';
        } else {
            $lc = '0';
        }

        $co2_lines = @($co2_lines | Where { $_[$bit] -eq $lc })
    }
}

$o2 = [Convert]::ToInt64($o2_lines[0], 2);
$co2 = [Convert]::ToInt64($co2_lines[0], 2);

Write-Host "gamma:$gamma  epsilon:$epsilon  answer:$($gamma * $epsilon)"
Write-Host "o2:$o2  co2:$co2  answer:$($o2 * $co2)"
