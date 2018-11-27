//
//  Day13.swift
//  Advent of Code 2015
//
//  Created by Rolf Staflin on 2018-11-26.
//  Copyright © 2018 Piro AB. All rights reserved.
//

import Foundation

class Guest: Equatable {
    let name:String
    var happiness:[String:Int] = [:]
    
    init(_ name: String) {
        self.name = name
    }

    static func == (lhs: Guest, rhs: Guest) -> Bool {
        return lhs.name == rhs.name
    }
}
class Placement: NSObject {
    var guests: [String] = []
    var happiness = 0
    
    init(_ guest: Guest) {
        guests.append(guest.name)
    }
    
    init(_ placement: Placement) {
        self.guests = placement.guests
        self.happiness = placement.happiness
    }
    
    func contains(_ name: String) -> Bool{
        return guests.first(where: {$0 == name}) != nil
    }
    
    func addGuest(_ name: String, happiness: Int) {
        guests.append(name)
        self.happiness += happiness
    }
    
    override var description: String { return "\(guests) has happiness \(happiness)" }
}

class Day13 {
    var guests:[Guest] = []

    fileprivate func findGuest(_ guestName: String) -> Guest {
        var guest = guests.first(where: {$0.name == guestName})
        if guest == nil {
            guest = Guest(guestName)
            guests.append(guest!)
        }
        return guest!
    }
    
    func solve () {
        let lines = Utils.readFileLines("Day13.txt")
        for line in lines {
            let tokens = line.components(separatedBy: " ")
            let guest = findGuest(tokens[0])
            
            var happiness = Int(tokens[3])!
            if tokens[2] == "lose" {
                happiness = -happiness
            }
            let neighborName = String(tokens.last!.dropLast())
            
            guest.happiness[neighborName] = happiness
        }
        print("Part 1: \(findBestPlacement())")
        let me = Guest("Me")
        guests.forEach {
            me.happiness[$0.name] = 0
            $0.happiness["Me"] = 0
        }
        guests.append(me)
        print("Part 2: \(findBestPlacement())")
    }
    
    func findBestPlacement() ->  Placement {
        return findBestPlacement(withStart: Placement(guests.first!))
    }
    
    func findBestPlacement(withStart start: Placement) -> Placement {
        let guestsLeft = guests.filter {!start.contains($0.name)}
        
        if guestsLeft.isEmpty {
            let firstGuest = guests.first(where: {$0.name == start.guests.first!})!
            let lastGuest = guests.first(where: {$0.name == start.guests.last!})!
            start.happiness += firstGuest.happiness[lastGuest.name]! + lastGuest.happiness[firstGuest.name]!
            return start
        }
        let currentGuest = guests.first(where: {$0.name == start.guests.last!})!

        var bestPlacementSoFar = Placement(currentGuest)
        
        for nextGuest in guestsLeft {
            let PlacementToGuest = Placement(start)
            let deltaJoy = currentGuest.happiness[nextGuest.name]! + nextGuest.happiness[currentGuest.name]!
            PlacementToGuest.addGuest(nextGuest.name, happiness: deltaJoy)
            let newPlacement = findBestPlacement(withStart: PlacementToGuest)
            if newPlacement.happiness > bestPlacementSoFar.happiness {
                bestPlacementSoFar = newPlacement
            }
        }
        return bestPlacementSoFar
    }
}
