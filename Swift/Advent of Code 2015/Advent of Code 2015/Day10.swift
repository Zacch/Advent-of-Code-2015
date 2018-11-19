//
//  Day10.swift
//  Advent of Code 2015
//
//  Created by Rolf Staflin on 2018-11-19.
//  Copyright © 2018 Piro AB. All rights reserved.
//

import Foundation

class Day10 {
    func solve () {
        var input = "1321131112"
        
        for _ in 1...40 {
            input = evolve(input)
        }
        print("Part 1: \(input.count)")

        for _ in 1...10 {
            input = evolve(input)
        }
        print("Part 2: \(input.count)")
    }
    
    func evolve(_ input: String) -> String {
        var output = ""
        var lastDigit: Character = "x"
        var count = 0
        for digit in input {
            if digit == lastDigit {
                count += 1
            } else {
                if count > 0 {
                    output.append("\(count)\(lastDigit)")
                }
                lastDigit = digit
                count = 1
            }
        }
        if count > 0 {
            output.append("\(count)\(lastDigit)")
        }
        return output
    }
}
