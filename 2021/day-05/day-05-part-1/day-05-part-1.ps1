$file = Get-Content ../input.txt

Function Resize-Map($Map, $X, $Y) {
    if ($X -ge $Map[0].length -or $Y -ge $Map.length) {
        $max_y = (($Y + 1), $Map.length | Measure-Object -Maximum).Maximum
        $max_x = (($X + 1), $Map[0].length | Measure-Object -Maximum).Maximum

        while ($Map.length -lt $max_y) {
            $Map += , @()
        }

        for ($i = 0; $i -lt $max_y; $i++) {
            while ($Map[$i].length -le $max_x) {
                $Map[$i] += , 0
            }
        }
    }

    return $Map
}

Function Print-Map($Map) {
    Write-Host "Map:  $($Map.length)"
    foreach ($row in $Map) {
        Write-Host -NoNewLine "  "
        foreach ($col in $row) {
            if ($col -eq 0) {
                Write-Host -NoNewLine "."
            } else {
                Write-Host -NoNewLine $col
            }
        }
        Write-Host 
    }
}

Function Load-Map {
    $Map = @()
    $Map += , $()
    
    foreach ($line in $file -split "\n") {
        $line = $line.trim()
        if ($line -eq "") {
            continue
        }
    
        $coords = $line -split " -> "
    
        $c1 = $coords[0] -split ","
        $x1 = [int]$c1[0]
        $y1 = [int]$c1[1]
    
        $c2 = $coords[1] -split ","
        $x2 = [int]$c2[0]
        $y2 = [int]$c2[1]
    
        $x_delta = 0
        $y_delta = 0
    
        if ($x1 -eq $x2) {
            if ($y1 -gt $y2) {
                $y_delta = [int]-1
            } else {
                $y_delta = [int]1
            }
        } elseif ($y1 -eq $y2) {
            if ($x1 -gt $x2) {
                $x_delta = [int]-1
            } else {
                $x_delta = [int]1
            }
        }
    
        if ($x_delta -ne 0 -or $y_delta -ne 0) {
            $max_x = ($x1, $x2 | Measure-Object -Maximum).Maximum
            $max_y = ($y1, $y2 | Measure-Object -Maximum).Maximum
    
            $Map = Resize-Map -Map $Map -X $max_x -Y $max_y
    
            $x = $x1
            $y = $y1
    
            $Map[$y][$x]++
            while ($x -ne $x2 -or $y -ne $y2) {
                $x += $x_delta
                $y += $y_delta
    
                $Map[$y][$x]++
            }
        }
    }

    return $Map
}

$map = Load-Map
$dangerous = 0
foreach ($row in $map) {
    foreach ($col in $row) {
        if ($col -ge 2) {
            $dangerous++
        }
    }
}

Write-Host "Dangerous: $dangerous"
