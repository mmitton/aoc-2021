class Node: System.IComparable {
    [int] $x
    [int] $y
    [int] $risk

    Node($x, $y, $risk) {
        $this.x = $x
        $this.y = $y
        $this.risk = $risk
    }

    [int]CompareTo($b) {
        return 0 - $this.risk.CompareTo($b.risk)
    }
}

Function Find-Shortest-Path($single_map, $multiple) {
    $map = New-Object 'UInt16[,]' (($single_map.GetLength(0) * $multiple), ($single_map.GetLength(1) * $multiple))
    $lowest = New-Object 'UInt16[,]' (($single_map.GetLength(0) * $multiple), ($single_map.GetLength(1) * $multiple))
    for ($y = 0; $y -lt $lowest.GetLength(1); $y++) {
        for ($x = 0; $x -lt $lowest.GetLength(0); $x++) {
            $lowest[$x, $y] = $([UInt16]::MaxValue)
    
            $x_dist = [Math]::Floor($x / $single_map.GetLength(0))
            $single_x = [int]($x % $single_map.GetLength(0))
    
            $y_dist = [Math]::Floor($y / $single_map.GetLength(1))
            $single_y = [int]($y % $single_map.GetLength(1))
    
            $risk = $single_map[$single_x, $single_y] + $x_dist + $y_dist
            if ($risk -gt 9) {
                $risk -= 9
            }
            $map[$x, $y] = $risk
        }
    }
    
    $end_x = $lowest.GetLength(0) - 1
    $end_y = $lowest.GetLength(1) - 1
    
    $lowest[0, 0] = 0
    $lowest_risk = $([UInt16]::MaxValue)
    
    $deltas = @(1, 0), @(0, 1), @(-1, 0), @(0, -1)

    $changed = $true
    while ($changed) {
        $changed = $false;

        for ($y = 0; $y -le $end_y; $y++) {
            for ($x = 0; $x -le $end_x; $x++) {
                for ($i = 0; $i -lt 4; $i++) {
                    $x1 = $x + $deltas[$i][0];
                    $y1 = $y + $deltas[$i][1];

                    if ($x1 -lt 0 -or $x1 -gt $end_x -or $y1 -lt 0 -or $y1 -gt $end_y) {
                        continue
                    }

                    if ($lowest[$x1, $y1] -ne $([UInt16]::MaxValue)) {
                        $new_risk = $lowest[$x1, $y1] + $map[$x, $y]
                        if ($lowest[$x, $y] -eq $([UInt16]::MaxValue) -or $new_risk -lt $lowest[$x, $y]) {
                            $lowest[$x, $y] = $new_risk
                            $changed = $true
                        }
                    }
                }
            }
        }
    }
   
    return $lowest[$end_x, $end_y]
}

Function Write-Map($map, $space) {
    Write-Host "Map"
    for ($y = 0; $y -lt $map.GetLength(1); $y++) {
        for ($x = 0; $x -lt $map.GetLength(0); $x++) {
            if ($space) {
                Write-Host -NoNewLine $map[$x, $y] " "
            } else {
                Write-Host -NoNewLine $map[$x, $y] " "
            }
        }
        Write-Host
    }
}

foreach ($filename in "../input-sample.txt", "../input.txt") {
    $file = Get-Content $filename
    $lines = $file -split "\n"
    $single_map = New-Object 'UInt16[,]' ($lines[0].length, $lines.length)
    for ($y = 0; $y -lt $single_map.GetLength(1); $y++) {
        for ($x = 0; $x -lt $single_map.GetLength(0); $x++) {
            $single_map[$x, $y] = [int]$lines[$y][$x] - [int][char]'0'
        }
    }

    foreach ($multiple in 1, 5) {
        $start = Get-Date
        $shortest_path = Find-Shortest-Path $single_map $multiple
        
        $end = Get-Date
        Write-Host "$($end-$start)  $($filename):$multiple Shortest Path: $shortest_path"
    }
}
