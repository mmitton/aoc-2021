Function Write-Map($map) {
    Write-Host "Map $($map.GetLength(0))x$($map.GetLength(1))"
    foreach ($y in (0..($map.GetLength(1)-1))) {
        foreach ($x in (0..($map.GetLength(0)-1))) {
            Write-Host -NoNewLine $map[$x, $y]
        }
        Write-Host ""
    }
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

# Write-Map $map

$risk = 0
foreach ($y in (0..($map.GetLength(1) - 1))) {
    foreach ($x in (0..($map.GetLength(0) - 1))) {
        $lowest = $true
        if ($x -ne 0 -and $map[($x-1), $y] -le $map[$x, $y]) {
            $lowest = $false
        }
        if ($x -ne $map.GetLength(0) - 1 -and $map[($x+1), $y] -le $map[$x, $y]) {
            $lowest = $false
        }
        if ($y -ne 0 -and $map[$x, ($y-1)] -le $map[$x, $y]) {
            $lowest = $false
        }
        if ($y -ne $map.GetLength(1) - 1 -and $map[$x, ($y+1)] -le $map[$x, $y]) {
            $lowest = $false
        }

        if ($lowest) {
            Write-Host "Found lowest at $x,$y  $($map[$x, $y])";
            $risk += $map[$x, $y] + 1
        }
    }
}

Write-Host "Risk: $risk"
