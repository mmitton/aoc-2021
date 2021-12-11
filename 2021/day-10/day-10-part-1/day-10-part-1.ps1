$file = Get-Content ../input.txt

$answer = 0;
foreach ($line in $file -split "\n") {
    $stack = New-Object Collections.Generic.Stack[char]
    $bad = $false
    for ($i = 0; $i -lt $line.length; $i++) {
        switch ($line[$i]) {
            '(' { $stack.Push('('); }
            '[' { $stack.Push('['); }
            '{' { $stack.Push('{'); }
            '<' { $stack.Push('<'); }
            ')' { if ($stack.Pop() -ne '(') { $answer += 3; $invalid = $true; break; } }
            ']' { if ($stack.Pop() -ne '[') { $answer += 57; $invalid = $true; break; } }
            '}' { if ($stack.Pop() -ne '{') { $answer += 1197; $invalid = $true; break; } }
            '>' { if ($stack.Pop() -ne '<') { $answer += 25137; $invalid = $true; break; } }
        }
    }
}

Write-Host "Answer: $answer"
