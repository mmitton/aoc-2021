Function Get-SpawnCount($days) {
    $counts = New-Object 'int64[]' ($days+1)

    for ($day = $days; $day -gt 0; $day--) {
        $counts[$day] = 1;
        if ($day -le $days - 7) {
            $counts[$day] += $counts[$day + 7];
        }
        if ($day -le $days - 9) {
            $counts[$day] += $counts[$day + 9];
        }
    }

    return , $counts
}

$file = Get-Content ../input.txt
$inputs = ($file -split "\n")[0] -split ","

$counts = Get-SpawnCount 80
$total = [int]0

foreach ($num in $inputs) {
    $num = [int]$num
    $total += 1 + $counts[$num + 1];
}

Write-Host "Total Fish: $total"
