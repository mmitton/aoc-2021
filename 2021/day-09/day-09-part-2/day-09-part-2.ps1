Function Contains-Point($points, $x, $y) {
    foreach ($point in $points) {
        if ($point[0] -eq $x -and $point[1] -eq $y) {
            return $true
        }
    }
    return $false
}

Function Find-Basin($map, $basins, $x0, $y0) {
    $points = @()
    $points += , @($x0, $y0)

    $map[$x0, $y0] = 9

    for ($i = 0; $i -lt $points.length; $i++) {
        $x1 = $points[$i][0];
        $y1 = $points[$i][1];

        $deltas = @(
            @(-1, 0),
            @(1, 0),
            @(0, -1),
            @(0, 1)
        )
        foreach ($delta in $deltas) {
            $x = $x1 + $delta[0]
            $y = $y1 + $delta[1]

            if ($x -lt 0) {
                continue;
            }
            if ($x -ge $map.GetLength(0)) {
                continue;
            }
            if ($y -lt 0) {
                continue;
            }
            if ($y -ge $map.GetLength(1)) {
                continue;
            }
            if ($map[$x, $y] -eq 9) {
                continue;
            }
            $points += , @($x, $y)
            $map[$x, $y] = 9
        }
    }

    return , $points
}

Function Find-Basins($map) {
    $basins = @()

    foreach ($y in (0..($map.GetLength(1) - 1))) {
        foreach ($x in (0..($map.GetLength(0) - 1))) {
            if ($map[$x, $y] -eq 9) {
                continue
            }

            $ignore = $false
            foreach ($basin in $basins) {
                if (Contains-Point $basin $x $y) {
                    $ignore = $true
                    break
                }
            }
            if ($ignore) {
                continue
            }

            $basin = Find-Basin $map $basins $x $y
            $basins += , $basin
        }
    }
    return , $basins
}

$file = Get-Content ../input.txt
$lines = $file -split "\n"

$map = New-Object 'int[,]' ($lines[0].length, $lines.length)
foreach ($y in (0..($lines.length - 1))) {
    foreach ($x in (0..($lines[$y].length - 1))) {
        $c = $($lines[$y][$x])
        $d = [int][String]$c
        $map[$x, $y] = $d
    }
}

$basins = Find-Basins $map
$basin_sizes = @()

Write-Host "Basins $($basins.length)"
foreach ($basin in $basins) {
    $basin_sizes += $basin.length
}

$basin_sizes = $basin_sizes | Sort-Object -Desc
Write-Host "Basin Sizes: $basin_sizes"

Write-Host "Answer: $($basin_sizes[0] * $basin_sizes[1] * $basin_sizes[2])"
