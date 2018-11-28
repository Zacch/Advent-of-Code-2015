//
//  Day14.swift
//  Advent of Code 2015
//
//  Created by Rolf Staflin on 2018-11-27.
//  Copyright © 2018 Piro AB. All rights reserved.
//

import Foundation

class Deer : Comparable {
    
    var name = ""
    var speed = 0
    var time = 0
    var restTime = 0

    var distance = 0
    var phase = 0
    var score = 0
    
    func run() {
        if phase >= 0 {
            distance += speed
        }
        phase += 1
        if phase == time {
            phase = -restTime
        }
    }

    static func < (lhs: Deer, rhs: Deer) -> Bool {
        return lhs.distance < rhs.distance
    }
    
    static func == (lhs: Deer, rhs: Deer) -> Bool {
        return lhs.distance == rhs.distance
    }
}

class Day14 {
    
    var part1Winner = ""
    var longestDistance = 0
    let goalTime = 2503
    
    var deer: [Deer] = []

    func solve () {
        let lines = Utils.readFileLines("Day14.txt")
        for line in lines {
            let tokens = line.components(separatedBy: " ")
            let deer = Deer()
            self.deer.append(deer)
            deer.name = tokens[0]
            deer.speed = Int(tokens[3])!
            deer.time = Int(tokens[6])!
            deer.restTime = Int(tokens[13])!

            let timePerCycle = deer.time + deer.restTime
            let distancePerCycle = deer.speed * deer.time
            let cycles = goalTime / timePerCycle

            var distance = distancePerCycle * cycles
            let timeLeft = goalTime - timePerCycle * cycles
            if (timeLeft < deer.time) {
                distance += timeLeft * deer.speed
            } else {
                distance += distancePerCycle
            }
            if distance > longestDistance {
                longestDistance = distance
            }
        }
        print("Part 1: \(longestDistance)")

        for _ in 1...goalTime {
            deer.forEach { $0.run() }
            let longest = deer.max()!.distance
            deer.filter { $0.distance == longest }.forEach { $0.score += 1 }
        }
        print("Part 2: \(deer.reduce(Int.min, { max($0, $1.score) }))")
    }
}
