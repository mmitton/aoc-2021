Function Play-Move($board, $number) {
    $sum = 0
    foreach ($row in $board) {
        foreach ($idx in (0..($row.length - 1))) {
            if ($row[$idx] -eq $number) {
                $row[$idx] = -1;
            }
            if ($row[$idx] -ne -1) {
                $sum = $sum + [int]$row[$idx]
            }
        }
    }
    $ans = $sum * $number

    foreach ($y in (0..($board.length - 1))) {
        $won = $true
        foreach ($x in (0..($board[0].length - 1))) {
            if ($board[$y][$x] -ne -1) {
                $won = $false
                break;
            }
        }

        if ($won -eq $true) {
            return $true, $ans
        }
    }

    foreach ($x in (0..($board[0].length - 1))) {
        $won = $true
        foreach ($y in (0..($board.length - 1))) {
            if ($board[$y][$x] -ne -1) {
                $won = $false
                break;
            }
        }

        if ($won -eq $true) {
            return $true, $ans
        }
    }

    return $false, $ans
}


$file = Get-Content ../input.txt
$lines = $file -split "\n"

$called_numbers = $lines[0] -split ","

$boards = @();

for ($i = 2; $i -lt $lines.length; $i += 6) {
    $board = @();
    for ($j = 0; $j -lt 5; $j++) {
        $board += , @($lines[$i + $j] -replace '\s+', ' ' -split " ")
    }

    $boards += , $board
}

foreach ($called_number in $called_numbers) {
    foreach ($board in $boards) {
        $win, $ans = Play-Move $board $called_number
        if ($win -eq $true) {
            Write-Host $ans
            return
        }
    }
}

Write-Host "No solution"
return
