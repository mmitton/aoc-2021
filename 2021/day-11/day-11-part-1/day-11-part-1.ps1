Function Print-Grid($grid) {
    Write-Host "Grid"
    for ($y = 0; $y -lt $grid.GetLength(1); $y++) {
        for ($x = 0; $x -lt $grid.GetLength(0); $x++) {
            Write-Host -NoNewLine $grid[$x, $y]
        }
        Write-Host ""
    }
}

Function Inc($grid, $x, $y) {
    if ($x -lt 0 -or $x -ge $grid.GetLength(0)) {
        return 0
    }
    if ($y -lt 0 -or $y -ge $grid.GetLength(1)) {
        return 0
    }
    if ($grid[$x, $y] -eq 10) {
        return 0
    }

    $flashes = 0
    $grid[$x, $y]++
    if ($grid[$x, $y] -eq 10) {
        # FLASH ME

        $flashes = 1
        foreach ($x1 in (-1..1)) {
            foreach ($y1 in (-1..1)) {
                $flashes += Inc $grid ($x+$x1) ($y+$y1)
            }
        }
    }

    return $flashes
}

$file = Get-Content ../input.txt
$grid = New-Object 'Int[,]' ($file[0].length, $file.length)

for ($y = 0; $y -lt $file.length; $y++) {
    for ($x = 0; $x -lt $file[$y].length; $x++) {
        $grid[$x, $y] = [int]$file[$y][$x] - [int][char]'0';
    }
}

$flashes = 0;
for ($i = 0; $i -lt 100; $i++) {
    for ($y = 0; $y -lt $grid.GetLength(1); $y++) {
        for ($x = 0; $x -lt $grid.GetLength(0); $x++) {
            $flashes += Inc $grid $x $y
        }
    }
    for ($y = 0; $y -lt $grid.GetLength(1); $y++) {
        for ($x = 0; $x -lt $grid.GetLength(0); $x++) {
            if ($grid[$x, $y] -eq 10) {
                $grid[$x, $y] = 0;
            }
        }
    }
}

Write-Host $flashes
