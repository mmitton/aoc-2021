Function Get-Number($binary, $n) {
    $num = $binary.bits.SubString($binary.pos, $n)
    $binary.pos += $n
    return [Convert]::ToInt64($num, 2)
}

Function Get-Packet($binary) {
    $packet = [PSCustomObject] @{
        version = Get-Number $binary 3
        type = Get-Number $binary 3
        imm = 0
        sub_packets = @()
    }

    if ($packet.type -eq 4) {
        # Load Immediate Value
        $last_chunk = $false
        while (!$last_chunk) {
            $last_chunk = (Get-Number $binary 1) -eq 0
            $packet.imm = $packet.imm -shl 4
            $packet.imm = $packet.imm -bor (Get-Number $binary 4)
        }
    } else {
        $len_type = Get-Number $binary 1
        if ($len_type -eq 0) {
            # Next 15 bits are the length of the sub packets
            $len = Get-Number $binary 15
            $sub_binary = [PSCustomObject] @{
                bits = $binary.bits.SubString($binary.pos, $len)
                pos = 0
            }
            $binary.pos += $len
            while ($sub_binary.pos -lt $sub_binary.bits.length) {
                $packet.sub_packets += Get-Packet $sub_binary
            }
        } else {
            # Next 11 bits are the number of sub packets
            $len = Get-Number $binary 11
            for ($i = 0; $i -lt $len; $i++) {
                $packet.sub_packets += Get-Packet $binary
            }
        }
    }

    return $packet
}

Function Get-Version-Sum($packet) {
    $sum = $packet.version

    foreach ($sub_packet in $packet.sub_packets) {
        $sum += Get-Version-Sum $sub_packet
    }

    return $sum
}

Function Get-Eval($packet) {
    $val = 0

    switch ($packet.type) {
        4 { $val = $packet.imm }
        0 { 
            # SUM
            foreach ($sub_packet in $packet.sub_packets) {
                $val += Get-Eval $sub_packet
            }
        }
        1 { 
            # PRODUCT
            $val = 1
            foreach ($sub_packet in $packet.sub_packets) {
                $val *= Get-Eval $sub_packet
            }
        }
        2 {
            # MIN
            $val = $([Int64]::MaxValue)
            foreach ($sub_packet in $packet.sub_packets) {
                $sub_val = Get-Eval $sub_packet
                if ($sub_val -lt $val) {
                    $val = $sub_val
                }
            }
        }
        3 {
            # MAX
            foreach ($sub_packet in $packet.sub_packets) {
                $sub_val = Get-Eval $sub_packet
                if ($sub_val -gt $val) {
                    $val = $sub_val
                }
            }
        }
        5 {
            # GREATER THAN
            if ((Get-Eval $packet.sub_packets[0]) -gt (Get-Eval $packet.sub_packets[1])) {
                $val = 1
            }
        }
        6 {
            # LESS THAN
            if ((Get-Eval $packet.sub_packets[0]) -lt (Get-Eval $packet.sub_packets[1])) {
                $val = 1
            }
        }
        7 {
            # EQUAL
            if ((Get-Eval $packet.sub_packets[0]) -eq (Get-Eval $packet.sub_packets[1])) {
                $val = 1
            }
        }
    }

    return $val
}

$SAMPLE = $false

if ($SAMPLE) {
    $file = Get-Content ../input-sample.txt
} else {
    $file = Get-Content ../input.txt
}

foreach ($line in $file -split "\n") {
    $line = $line.trim()
    if ($line -eq "") {
        continue
    }
    $bits = ""
    for ($i = 0; $i -lt $line.length; $i++) {
        switch ($line[$i]) {
            "0" { $bits += "0000" }
            "1" { $bits += "0001" }
            "2" { $bits += "0010" }
            "3" { $bits += "0011" }
            "4" { $bits += "0100" }
            "5" { $bits += "0101" }
            "6" { $bits += "0110" }
            "7" { $bits += "0111" }
            "8" { $bits += "1000" }
            "9" { $bits += "1001" }
            "A" { $bits += "1010" }
            "B" { $bits += "1011" }
            "C" { $bits += "1100" }
            "D" { $bits += "1101" }
            "E" { $bits += "1110" }
            "F" { $bits += "1111" }
        }
    }

    $binary = [PSCustomObject] @{
        bits = $bits
        pos = 0
    }

    $packet = Get-Packet $binary

    if ($SAMPLE) {
        Write-Host "Line: $line"
        Write-Host "Binary: $binary"
        Write-Host "Packet: $packet"
    }
    Write-Host "Version Sum: $(Get-Version-Sum $packet)"
    Write-Host "Eval: $(Get-Eval $packet)"
}
