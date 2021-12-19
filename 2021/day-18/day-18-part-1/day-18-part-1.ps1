$SAMPLE = $false

Function Get-Next($number, $idx, $dir) {
    for ($i = $idx + $dir; $i -ge 0 -and $i -lt $number.Count; $i += $dir) {
        if ($number[$i] -is 'int') {
            return $i
        }
    }

    return $null
}

Function Do-Reduce-Iter($number) {
    $depth = 0;
    for ($i = 0; $i -lt $number.Count; $i++) {
        switch ($number[$i]) {
            '[' { $depth++ }
            ']' { $depth-- }
            default {
                if ($depth -gt 4 -and $number[$i+1] -is 'int') {
                    $prev = Get-Next $number $i -1
                    $next = Get-Next $number ($i+1) 1

                    if ($prev -ne $null) {
                        $number[$prev] += $number[$i]
                    }
                    if ($next -ne $null) {
                        $number[$next] += $number[$i + 1]
                    }

                    $number[$i] = [int]0
                    $number.RemoveAt($i+2)
                    $number.RemoveAt($i+1)
                    $number.RemoveAt($i-1)

                    return $true
                }
            }
        }
    }

    for ($i = 0; $i -lt $number.Count; $i++) {
        if ($number[$i] -is 'int' -and $number[$i] -ge 10) {
            $lhs = [int][Math]::Floor($number[$i] / 2);
            $rhs = [int][Math]::Ceiling($number[$i] / 2);
            $number[$i] = '['
            $number.Insert($i + 1, ']');
            $number.Insert($i + 1, $rhs);
            $number.Insert($i + 1, $lhs);
            return $true
        }
    }

    return $false
}

Function Do-Reduce($number) {
    while (Do-Reduce-Iter $number) {
    }
}

Function Add-Numbers($n1, $n2) {
    if ($n1 -eq $null) {
        $n1 = New-Object System.Collections.ArrayList
        foreach ($t in $n2) {
            $null = $n1.Add($t)
        }

        return $n1
    }

    $n = New-Object System.Collections.ArrayList

    $null = $n.Add('[');
    foreach ($t in $n1) {
        $null = $n.Add($t)
    }
    foreach ($t in $n2) {
        $null = $n.Add($t)
    }
    $null = $n.Add(']')

    Do-Reduce $n
    return $n
}

Function Get-Magnitude($number) {
    $tokens = New-Object System.Collections.ArrayList
    foreach ($t in $number) {
        $null = $tokens.Add($t)
    }

    while ($tokens.Count -ne 1) {
        for ($i = 0; $i -lt $tokens.Count - 1; $i++) {
            if ($tokens[$i] -is 'int' -and $tokens[$i+1] -is 'int') {
                $tokens[$i] = (3 * $tokens[$i]) + (2 * $tokens[$i + 1])
                $tokens.RemoveAt($i+2)
                $tokens.RemoveAt($i+1)
                $tokens.RemoveAt($i-1)
                break
            }
        }
    }

    return $tokens[0]
}

if ($SAMPLE) {
    $file = Get-Content ../input-sample.txt
} else {
    $file = Get-Content ../input.txt
}
$lines = $file -split "\n"
$numbers = New-Object System.Collections.ArrayList
foreach ($line in $lines) {
    $line = $line.trim();
    if ($line -eq "") {
        break
    }

    $number = New-Object System.Collections.ArrayList

    for ($i = 0; $i -lt $line.length; $i++) {
        if ($line[$i] -eq ',') {
            continue
        }

        if ($line[$i] -eq '[' -or $line[$i] -eq ']') {
            $null = $number.Add($line[$i]);
        } else {
            $null = $number.Add([int][string]$line[$i]);
        }
    }

    $null = $numbers.Add($number)
}

$n = $null
foreach ($number in $numbers) {
    $n = Add-Numbers $n $number
}

Write-Host $n
Write-Host "magnitude: $(Get-Magnitude $n)"
