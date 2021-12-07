$file = Get-Content ../input.txt
$initial = @()
$min = $null;
$max = $null;
foreach ($pos in $file -split ",") {
    $pos = [int]$pos
    $initial += $pos

    if ($null -eq $min -or $min -gt $pos) {
        $min = $pos
    }
    if ($null -eq $max -or $max -lt $pos) {
        $max = $pos
    }
}

Write-Host "min=$min max=$max $initial"

$best_cost = $null;
for ($i = $min; $i -le $max; $i++) {
    $cost = 0;
    foreach ($pos in $initial) {
        $cost += [Math]::Abs($pos - $i)
    }

    if ($null -eq $best_cost -or $best_cost -gt $cost) {
        $best_cost = $cost
    }
}

Write-Host $best_cost
