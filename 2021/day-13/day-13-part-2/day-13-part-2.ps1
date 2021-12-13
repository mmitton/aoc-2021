Function Get-Point($points, $x, $y) {
    foreach ($point in $points) {
        if ($point.X -eq $x -and $point.Y -eq $y) {
            return $true
        }
    }

    return $false
}

Function Print-Points($points) {
    $w, $h = 0, 0
    foreach ($point in $points) {
        if ($point.X + 1 -gt $w) {
            $w = $point.X + 1
        }
        if ($point.Y + 1 -gt $h) {
            $h = $point.Y + 1
        }
    }

    Write-Host "$w x $h"
    for ($y = 0; $y -lt $h; $y++) {
        for ($x = 0; $x -lt $w; $x++) {
            if (Get-Point $points $x $y) {
                Write-Host -NoNewLine "X"
            } else {
                Write-Host -NoNewLine " "
            }
        }
        Write-Host
    }
}

Function Do-Fold($points, $fold) {
    foreach ($point in $points) {
        switch ($fold.axis) {
            "x" {
                if ($point.X -gt $fold.pos) {
                    $point.X = (2 * $fold.pos) - $point.X
                }
            }
            "y" {
                if ($point.Y -gt $fold.pos) {
                    $point.Y = (2 * $fold.pos) - $point.Y
                }
            }
        }
    }

    for ($i = $points.Count - 1; $i -gt 0; $i--) {
        for ($j = $i - 1; $j -ge 0; $j--) {
            if ($points[$i].X -eq $points[$j].X -and $points[$i].Y -eq $points[$j].Y) {
                $points.RemoveAt($i);
                break;
            }
        }
    }
}


$file = Get-Content ../input.txt

$points = New-Object System.Collections.Generic.List[PSCustomObject]
$folds = New-Object System.Collections.Generic.List[PSCustomObject]

foreach ($line in $file -split "\n") {
    if ($line -eq "") {
        continue
    } elseif ($line.StartsWith("fold along ")) {
        $folds.Add([PSCustomObject] @{
            axis = $line[11]
            pos = [int]$line.SubString(13)
        })
    } else {
        $x, $y = $line -split ","
        $points.Add([PSCustomObject] @{
            X = [int]$x
            Y = [int]$y
        })
    }
}

# Print-Points $points

foreach ($fold in $folds) {
    Do-Fold $points $fold
}

Print-Points $points

Write-Host "Dots: $($points.Count)"
