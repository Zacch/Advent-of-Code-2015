//
//  Day12.swift
//  Advent of Code 2015
//
//  Created by Rolf Staflin on 2018-11-25.
//  Copyright © 2018 Piro AB. All rights reserved.
//

import Foundation

class Day12 {
    
    func solve () {
        let json = Utils.readFileAsJson("Day12.txt")
        print("Part 1: \(sumContents(json, part: 1))")
        print("Part 2: \(sumContents(json, part: 2))")
    }

    func sumContents(_ json: Any, part: Int) -> Int {
        if let number = json as? Int {
            return number
        }
        var sum = 0
        if let dictionary = json as? [String: Any] {
            for (_, value) in dictionary {
                if part == 2,
                   let str = value as? String,
                   str == "red" {
                    return 0
                }
                sum += sumContents(value, part: part)
            }
        }
        if let array = json as? [Any] {
            for value in array {
                sum += sumContents(value, part: part)
            }
        }
        return sum
    }
}
