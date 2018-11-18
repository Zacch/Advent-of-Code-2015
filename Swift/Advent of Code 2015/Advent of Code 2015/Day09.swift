//
//  Day09.swift
//  Advent of Code 2015
//
//  Created by Rolf Staflin on 2018-11-15.
//  Copyright © 2018 Piro AB. All rights reserved.
//

import Foundation

class City {
    var name:String
    var neighbors: [String:Int] = [:]
    
    init(_ name:String) {
        self.name = name
    }
    
    func addNeighbor(_ neighborName: String, distance: Int) {
        neighbors[neighborName] = distance
    }
    
    // Will crash if neighbour doesn't exist
    func distanceTo(_ neighbor: String) -> Int {
        return neighbors[neighbor]!
    }
}

class Route: NSObject {
    var stops: [String] = []
    var length = 0
    
    init(_ city: City) {
        stops.append(city.name)
    }
    
    init(_ route: Route) {
        self.stops = route.stops
        self.length = route.length
    }
    
    func contains(_ name: String) -> Bool{
        return stops.first(where: {$0 == name}) != nil
    }
    
    func addStop(_ name: String, distance: Int) {
        stops.append(name)
        length += distance
    }
    
    override var description: String { return "\(stops) has length \(length)" }
}

class Day09 {
    var cities: [String:City] = [:]
    
    fileprivate func pair(_ cityName:String, to neighbor: String , atDistance distance:Int) {
        if let city = cities[cityName] {
            city.addNeighbor(neighbor, distance: distance)
        } else {
            let newCity: City = City(cityName)
            cities[cityName] = newCity
            newCity.addNeighbor(neighbor, distance: distance)
        }
    }
    
    func solve () {
        let lines = Utils.readFileLines("Day09.txt")
        for line in lines {
            let tokens = line.components(separatedBy: " ")
            let distance = Int(tokens[4]) ?? -1
            pair(tokens[0], to: tokens[2], atDistance: distance)
            pair(tokens[2], to: tokens[0], atDistance: distance)
        }
        
        let routes = findRoutes()
        print("Part 1: \(routes.shortest.length)")
        print("Part 2: \(routes.longest.length)")
    }
    
    func findRoutes() ->  (shortest: Route, longest: Route) {
        var shortestRouteSoFar = Route(City("Limbo"))
        shortestRouteSoFar.length = Int.max
        var longestRouteSoFar = Route(City("Limbo"))

        for city in cities.values {
            let startingRoute = Route(city)
            let routes = findRoutes(withStart: startingRoute)
            if routes.shortest.length < shortestRouteSoFar.length {
                shortestRouteSoFar = routes.shortest
            }
            if routes.longest.length > longestRouteSoFar.length {
                longestRouteSoFar = routes.longest
            }
        }
        return (shortestRouteSoFar, longestRouteSoFar)
    }
    
    func findRoutes(withStart start: Route) -> (shortest: Route, longest: Route) {
        let citiesLeft = cities.filter {!start.contains($0.key)}
        
        if citiesLeft.isEmpty {
            return (start, start)
        }
        
        let currentCity = cities[start.stops.last!]!
        var shortestRouteSoFar = Route(currentCity)
        shortestRouteSoFar.length = Int.max
        var longestRouteSoFar = Route(currentCity)

        for city in citiesLeft.values {
            let routeToCity = Route(start)
            routeToCity.addStop(city.name, distance: currentCity.distanceTo(city.name))
            let routes = findRoutes(withStart: routeToCity)
            if routes.shortest.length < shortestRouteSoFar.length {
                shortestRouteSoFar = routes.shortest
            }
            if routes.longest.length > longestRouteSoFar.length {
                longestRouteSoFar = routes.longest
            }
        }
        return (shortestRouteSoFar, longestRouteSoFar)
    }
}
