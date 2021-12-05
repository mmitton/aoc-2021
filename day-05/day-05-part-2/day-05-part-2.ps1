$file = Get-Content ../input.txt

Function Print-Map($Map) {
    Write-Host "Map:  $($Map.GetLength(0)) $($Map.GetLength(1))"
    for ($x = 0; $x -lt $Map.GetLength(0); $x++) {
        for ($y = 0; $y -lt $Map.GetLength(1); $y++) {
            if ($Map[$x, $y] -eq 0) {
                Write-Host -NoNewLine "."
            } else {
                Write-Host -NoNewLine $($Map[$x, $y])
            }
        }
        Write-Host 
    }
}

Function Load-Map {
    $Map = $null

    $max_x = 0
    $max_y = 0
    $coords = @()

    foreach ($line in $file -split "\n") {
        $line = $line.trim()
        if ($line -eq "") {
            continue
        }
    
        $parts = $line -split " -> "
    
        $c1 = $parts[0] -split ","
        $x1 = [int]$c1[0]
        $y1 = [int]$c1[1]
    
        $c2 = $parts[1] -split ","
        $x2 = [int]$c2[0]
        $y2 = [int]$c2[1]

        [int[]]$coord = @($x1, $y1, $x2, $y2)
        $coords += , $coord

        $max_x = ($max_x, $x1, $x2 | Measure-Object -Maximum).Maximum
        $max_y = ($max_y, $y1, $y2 | Measure-Object -Maximum).Maximum
    }

    $max_x++
    $max_y++
    $Map = New-Object 'int[,]' $max_x, $max_y

    foreach ($coord in $coords) {
        $x1, $y1, $x2, $y2 = $coord

        $x_delta = 0
        $y_delta = 0
    
        if ($y1 -gt $y2) {
            $y_delta = [int]-1
        } elseif ($y2 -gt $y1) {
            $y_delta = [int]1
        }

        if ($x1 -gt $x2) {
            $x_delta = [int]-1
        } elseif ($x2 -gt $x1) {
            $x_delta = [int]1
        }
    
        if ($x_delta -ne 0 -or $y_delta -ne 0) {
            $max_x = ($x1, $x2 | Measure-Object -Maximum).Maximum
            $max_y = ($y1, $y2 | Measure-Object -Maximum).Maximum
    
            $x = $x1
            $y = $y1

            $Map[$x, $y]++
            while ($x -ne $x2 -or $y -ne $y2) {
                $x += $x_delta
                $y += $y_delta
    
                $Map[$x, $y]++
            }
        }
    }

    return , $Map
}

$map = Load-Map
$dangerous = 0
foreach ($num in $map) {
    if ($num -ge 2) {
        $dangerous++
    }
}

Write-Host "Dangerous: $dangerous"
